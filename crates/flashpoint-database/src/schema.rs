// @generated automatically by Diesel CLI.

diesel::table! {
    additional_app (id) {
        id -> Text,
        applicationPath -> Text,
        autoRunBefore -> Bool,
        launchCommand -> Text,
        name -> Text,
        waitForExit -> Bool,
        parentGameId -> Nullable<Text>,
    }
}

diesel::table! {
    test_game (id) {
        id -> Text,
        parentGameId -> Nullable<Text>,
        title -> Text,
    }
}

diesel::table! {
    game (id) {
        id -> Text,
        parentGameId -> Nullable<Text>,
        title -> Text,
        // alternateTitles -> Text,
        // series -> Text,
        // developer -> Text,
        // publisher -> Text,
        // dateAdded -> Timestamp,
        // dateModified -> Timestamp,
        // platform -> Text,
        // broken -> Bool,
        // extreme -> Bool,
        // playMode -> Text,
        // status -> Text,
        // notes -> Text,
        // source -> Text,
        // applicationPath -> Text,
        // launchCommand -> Text,
        // releaseDate -> Text,
        // version -> Text,
        // originalDescription -> Text,
        // language -> Text,
        // library -> Text,
        // orderTitle -> Text,
        // activeDataId -> Nullable<Integer>,
        // activeDataOnDisk -> Bool,
        // tagsStr -> Text,
    }
}

diesel::table! {
    game_data (id) {
        id -> Integer,
        gameId -> Nullable<Text>,
        title -> Text,
        dateAdded -> Timestamp,
        sha256 -> Text,
        crc32 -> Integer,
        presentOnDisk -> Bool,
        path -> Nullable<Text>,
        size -> Integer,
        parameters -> Nullable<Text>,
    }
}

diesel::table! {
    game_tags_tag (gameId, tagId) {
        gameId -> Text,
        tagId -> Integer,
    }
}

diesel::table! {
    playlist (id) {
        id -> Text,
        title -> Text,
        description -> Text,
        author -> Text,
        icon -> Nullable<Text>,
        library -> Text,
        extreme -> Bool,
    }
}

diesel::table! {
    playlist_game (id) {
        id -> Integer,
        playlistId -> Text,
        order -> Integer,
        notes -> Text,
        gameId -> Nullable<Text>,
    }
}

diesel::table! {
    source (id) {
        id -> Integer,
        name -> Text,
        dateAdded -> Timestamp,
        lastUpdated -> Timestamp,
        sourceFileUrl -> Text,
        baseUrl -> Text,
        count -> Integer,
    }
}

diesel::table! {
    source_data (id) {
        id -> Integer,
        sourceId -> Nullable<Integer>,
        sha256 -> Text,
        urlPath -> Text,
    }
}

diesel::table! {
    tag (id) {
        id -> Integer,
        dateModified -> Timestamp,
        primaryAliasId -> Nullable<Integer>,
        categoryId -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tag_alias (id) {
        id -> Integer,
        tagId -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    tag_category (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(additional_app -> game (parentGameId));
diesel::joinable!(game_data -> game (gameId));
diesel::joinable!(game_tags_tag -> game (gameId));
diesel::joinable!(game_tags_tag -> tag (tagId));
diesel::joinable!(playlist_game -> game (gameId));
diesel::joinable!(playlist_game -> playlist (playlistId));
diesel::joinable!(source_data -> source (sourceId));
diesel::joinable!(tag -> tag_category (categoryId));

diesel::allow_tables_to_appear_in_same_query!(
  additional_app,
  game,
  game_data,
  game_tags_tag,
  playlist,
  playlist_game,
  source,
  source_data,
  tag,
  tag_alias,
  tag_category,
);
