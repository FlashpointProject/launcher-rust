use core::panic;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};
use syn::{
  parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, DataEnum, DeriveInput, Field,
  FieldsUnnamed, Ident, Variant,
};

#[proc_macro_derive(TableQueryBuilder)]
pub fn table_query_builder_macro(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // First, a whole bunch of defaults. We will update them from the attributes as appropriate.
  // This will be appended to the struct's name to make the enum's name. We default to "Relation".
  let mut struct_suffix = "Relation".to_string();
  // The name of the table this struct is associated with. We default to snake_case("StructName") = "struct_name".
  let mut table_name = input.ident.to_string().to_case(Case::Snake);
  // The database backend to use. We default to sqlite.
  let mut db_backend = "Sqlite".to_string();
  // The secondary sort column to use. We default to the first column in the struct.
  let mut unique_column: Option<(String, Ident)> = None;

  // Read the attributes to determine overrides for the above defaults.
  let mut non_consumed_attrs: Vec<Attribute> = vec![];
  for a in input.attrs {
    // If any of these match, the attribute is consumed by our macro, and won't be copied to the enum.
    if let Some(s) = get_attr_value(&a, "flashpoint_derive", "struct_suffix") {
      struct_suffix = s;
    } else if let Some(s) = get_attr_value(&a, "diesel", "table_name") {
      table_name = s;
    } else if let Some(s) = get_attr_value(&a, "flashpoint_derive", "db_backend") {
      db_backend = s;
    } else if let Some(s) = get_attr_value(&a, "flashpoint_derive", "unique_column") {
      let temp = Ident::new(&s, Span::call_site());
      unique_column = Some((s, temp));
    } else {
      non_consumed_attrs.push(a);
    }
  }

  // Turn the configurable names into TokenStream-compatible identities.
  let enum_ident = Ident::new(
    &(input.ident.to_string() + &struct_suffix),
    Span::call_site(),
  );
  let table_name_ident = Ident::new(&table_name, Span::call_site());
  let db_backend_ident = Ident::new(&db_backend, Span::call_site());

  // Split out the struct fields from the parsed data.
  let struct_data = match input.data {
    syn::Data::Struct(d) => d,
    _ => panic!("MyMacro can only be used on structs!"),
  };
  let struct_fields = match struct_data.fields {
    syn::Fields::Named(a) => a.named,
    syn::Fields::Unnamed(a) => a.unnamed,
    _ => panic!(),
  };

  // We're going to loop over the fields and build these up.
  // This one is going to hold the enum fields.
  let mut fields_enum = Punctuated::<Variant, Comma>::new();
  // These two are for the bodies of the match statments in filter_column().
  let mut whitelist_filters = proc_macro2::TokenStream::new();
  let mut blacklist_filters = proc_macro2::TokenStream::new();
  // These two are for the bodies of the match statments in order_query().
  let mut ascending_orders = proc_macro2::TokenStream::new();
  let mut descending_orders = proc_macro2::TokenStream::new();
  // These two are for the bodies of the match statements in page().
  let mut ascending_page = proc_macro2::TokenStream::new();
  let mut descending_page = proc_macro2::TokenStream::new();

  // Loop over the struct's fields.
  for field in struct_fields {
    // The name of the field. In a properly-formed struct, we'll have field names - no clue why this is an Option<Ident>.
    let field_ident = field.ident.expect("No field name?");
    // The column name associated with this field. Defaults to the camelCase version of the column name.
    let mut column_name = field_ident.to_string().to_case(Case::Camel);

    // Check the field attributes for a different column name, e.g. #[diesel(column_name = differentName)].
    for a in field.attrs {
      // If there are multiple column-name-overriding attributes, accept the last one.
      if let Some(name) = get_attr_value(&a, "diesel", "column_name") {
        column_name = name;
      }
    }
    // Make an Ident out of the column name.
    let column_ident = Ident::new(&column_name, Span::call_site());

    let unique_column_ident = match &unique_column {
      Some((_, b)) => b.clone(),
      // Check if unique_column is still None. If it is, set it to this column.
      // This should only happen for the first column in the struct.
      None => {
        let temp = Ident::new(&column_name, Span::call_site());
        unique_column = Some((column_name, temp.clone()));
        temp
      }
    };

    // For some reason an enum variant's enclosed type is a punctuated list of fields,
    // where each field is null except for the type.
    let mut temp = Punctuated::<Field, Comma>::new();
    let type_only_field = Field {
      vis: syn::Visibility::Inherited,
      attrs: vec![],
      ident: None,
      colon_token: None,
      // Copy the type from the struct's field.
      ty: field.ty.clone(),
    };
    temp.push(type_only_field);
    let variant = Variant {
      // Enum variants don't inherit attributes from struct fields.
      attrs: vec![],
      ident: field_ident.clone(),
      fields: syn::Fields::Unnamed(FieldsUnnamed {
        paren_token: syn::token::Paren(Span::call_site()),
        unnamed: temp,
      }),
      discriminant: None,
    };
    // Push the enum variant to the list.
    fields_enum.push(variant);

    // Now we build some match statement bodies.
    // This should be a colon-separated list of segments (Punctuated<PathSegment,Colon>)
    // corresponding to e.g. alloc::string::String.
    let path = match field.ty {
      syn::Type::Path(k) => k.path.segments,
      _ => panic!("Bad type path?"),
    };
    // Depending on the type, our filter conditions will be slightly different.
    // Note that we're building two conditions here: one for the whitelist matching, and the other for blacklist matching.
    let type_str = quote! {#path}.to_string();
    let (match_arm_white, match_arm_black) = match type_str.as_str() {
      // If it's a string, use like() and not_like() for non-exact matching.
      "String" => {
        (
          quote! {
              #enum_ident::#field_ident(val) => FilterDsl::filter(q,
                  #table_name_ident::#column_ident.like("%".to_owned() + val + "%")),
          },
          quote! {
              #enum_ident::#field_ident(val) => FilterDsl::filter(q,
                  #table_name_ident::#column_ident.not_like("%".to_owned() + val + "%")),
          },
        )
      }
      // If it's an Option<T>, handle it specially. The Some() arm should be normal, the None arm should be IS (NOT) NULL.
      opt if matches!(get_option_inner(&type_str), Some(_)) => {
        match get_option_inner(opt).unwrap().as_str() {
          "String" => {
            (
              quote! {
                  #enum_ident::#field_ident(Some(val)) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.like("%".to_owned() + val + "%")),
                  #enum_ident::#field_ident(None) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.is_null()),
              },
              quote! {
                  #enum_ident::#field_ident(Some(val)) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.not_like("%".to_owned() + val + "%")),
                  #enum_ident::#field_ident(None) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.is_not_null()),
              },
            )
          }
          _ => {
            (
              quote! {
                  #enum_ident::#field_ident(Some(val)) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.eq(val.clone())),
                  #enum_ident::#field_ident(None) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.is_null()),
              },
              quote! {
                  #enum_ident::#field_ident(Some(val)) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.ne(val.clone())),
                  #enum_ident::#field_ident(None) => FilterDsl::filter(q,
                      #table_name_ident::#column_ident.is_not_null()),
              },
            )
          }
        }
      }
      // For other types, use exact matching (eq() and ne()).
      _ => {
        (
          quote! {
              #enum_ident::#field_ident(val) => FilterDsl::filter(q,
                  #table_name_ident::#column_ident.eq(val.clone())),
          },
          quote! {
              #enum_ident::#field_ident(val) => FilterDsl::filter(q,
                  #table_name_ident::#column_ident.ne(val.clone())),
          },
        )
      }
    };

    // Append the match arms to the TokenStreams for the filter match bodies.
    whitelist_filters.append_all(match_arm_white);
    blacklist_filters.append_all(match_arm_black);

    // Also add match arms for the order_by() function.
    ascending_orders.append_all(quote! {
            #enum_ident::#field_ident(_) => q
            .order((#table_name_ident::#column_ident.asc(), #table_name_ident::#unique_column_ident.asc())),
        });
    descending_orders.append_all(quote! {
            #enum_ident::#field_ident(_) => q
            .order((#table_name_ident::#column_ident.desc(), #table_name_ident::#unique_column_ident.desc())),
        });

    // And for the page() function.
    ascending_page.append_all(quote!{
            #enum_ident::#field_ident(s) => q.filter(#table_name_ident::#column_ident.ge(s.clone()))
            .order((#table_name_ident::#column_ident.asc(), #table_name_ident::#unique_column_ident.asc())),
        });
    descending_page.append_all(quote!{
            #enum_ident::#field_ident(s) => q.filter(#table_name_ident::#column_ident.le(s.clone()))
            .order((#table_name_ident::#column_ident.desc(), #table_name_ident::#unique_column_ident.desc())),
        });
  }

  let vis = input.vis;

  // Stick the TokenStreams we've built into an impl block with the relevant functions.
  let impl_functions = quote! {
      impl #enum_ident {
          /// Applies a single filter to the query.
          /// For String columns, this is a WHERE column (NOT) LIKE %value%.
          /// For other types, this is a WHERE column (!)= value.
          /// * `q` - The query to add this filter to.
          /// * `whitelist` - Whether this filter should be a whitelist or blacklist.
          #vis fn filter_column<'a, 'b>(
            &self,
            q: IntoBoxed<'a, #table_name_ident::table, #db_backend_ident>,
            whitelist: bool,
          ) -> IntoBoxed<'b, #table_name_ident::table, #db_backend_ident>
          where
            'a: 'b,
          {
            use diesel::query_dsl::methods::FilterDsl;

            if whitelist {
              match self {
                #whitelist_filters
              }
            } else {
              match self {
                #blacklist_filters
              }
            }
          }

          /// Adds an ORDER BY clause to a query. Uses the unique column as a secondary sort.
          /// * `q` - The query to add this order-by to.
          /// * `asc` - Whether to order ascending or descending.
          #vis fn order_query<'a, 'b>(
            &self,
            q: IntoBoxed<'a, #table_name_ident::table, #db_backend_ident>,
            asc: bool,
          ) -> IntoBoxed<'b, #table_name_ident::table, #db_backend_ident>
          where
            'a: 'b,
          {
            if asc {
              match self {
                #ascending_orders
              }
            } else {
              match self {
                #descending_orders
              }
            }
          }

          /// Adds the requisite WHERE, LIMIT, OFFSET, and ORDER BY clauses for keyset pagination.
          /// Uses the current relation as the cutoff key.
          /// * `q` - The query to add the relevant clauses to.
          /// * `asc` - Whether to order ascending or descending.
          /// * `page_size` - The page size to use.
          #vis fn page<'a, 'b>(
            &self,
            q: IntoBoxed<'a, #table_name_ident::table, #db_backend_ident>,
            asc: bool,
            page_size: i64,
          ) -> IntoBoxed<'b, #table_name_ident::table, #db_backend_ident>
          where
            'a: 'b,
          {
            let k = if asc {
              match self {
                #ascending_page
              }
            } else {
              match self {
                #descending_page
              }
            };
            k.offset(1).limit(page_size)
          }
      }
  };

  // Assemble the enum from its variants.
  let data_enum = DataEnum {
    enum_token: syn::token::Enum {
      span: Span::call_site(),
    },
    brace_token: syn::token::Brace {
      span: Span::call_site(),
    },
    variants: fields_enum,
  };
  // The new enum should inherit the unconsumed attributes, the visibility, and the generics.
  // Actually nvm, forget about inheriting attrs.
  let built_enum = DeriveInput {
    attrs: vec![], //non_consumed_attrs,
    vis,
    ident: enum_ident,
    generics: input.generics,
    data: syn::Data::Enum(data_enum),
  };

  // Combine the built enum and the functions into a single tokenstream.
  let combined_tokenstream = quote! {
      #[allow(non_camel_case_types)]
      #built_enum
      #impl_functions
  };

  // Hand the output tokens back to the compiler
  TokenStream::from(combined_tokenstream)
}

/// Read the value from a #\[wrapper(key = value)\] attribute.
/// Returns None if the structure is invalid, or the wrapper and key aren't the expected ones.
fn get_attr_value(attr: &Attribute, expected_wrapper: &str, expected_key: &str) -> Option<String> {
  // For enum variants.
  use proc_macro2::TokenTree::*;

  // Check the wrapper.
  let attr_wrapper = attr.path.segments.first()?.ident.to_string();
  if attr_wrapper != expected_wrapper {
    return None;
  }
  // Make an interator on the inner group of tokens (that is, the "key = value" part).
  let mut iterable_tokens = match attr.tokens.clone().into_iter().next()? {
    Group(g) => g,
    _ => return None,
  }
  .stream()
  .into_iter();

  // The first token should be "key".
  let key = match iterable_tokens.next()? {
    Ident(i) => i.to_string(),
    _ => return None,
  };
  if key != expected_key {
    return None;
  }

  // The next token should be '='.
  let relation = match iterable_tokens.next()? {
    Punct(p) => p.as_char(),
    _ => return None,
  };
  if relation != '=' {
    return None;
  }

  // The final token is what we're looking for.
  let name = match iterable_tokens.next()? {
    Ident(i) => i.to_string(),
    _ => return None,
  };

  Some(name)
}

fn get_option_inner(type_str: &str) -> Option<String> {
  let no_option = type_str.strip_prefix("Option")?.trim();
  let no_brackets = no_option.strip_prefix("<")?.strip_suffix(">")?.trim();
  return Some(no_brackets.to_string());
}
