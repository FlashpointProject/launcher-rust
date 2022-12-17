use cfg_if::cfg_if;
use flashpoint_config::types::{Config, Preferences};
use flashpoint_database::types::DbState;
use std::path::Path;
use tokio::fs::File;

cfg_if!(
  if #[cfg(feature = "services")] {
    use flashpoint_config::types::Services;
  }
);
cfg_if!(
  if #[cfg(feature = "websocket")] {
    mod ws;
    use ws::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;
    use tokio::io::AsyncReadExt;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_tungstenite::tungstenite::Message;
    use futures_channel::mpsc::{unbounded, UnboundedSender};
    use std::net::SocketAddr;
    use futures_util::StreamExt;
    type Tx = UnboundedSender<Message>;
    type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
  }
);

pub mod signals;
use signals::*;

#[derive(Clone, Copy, Debug)]
pub enum InitLoad {
  Services,
  Database,
  Playlists,
  Extensions,
  ExecMappings,
  Curate,
}

signal!(ExitSignal<ExitRecv, i32>);
signal!(InitLoadSignal<InitLoadRecv, InitLoad>);

pub struct FlashpointSignals {
  pub exit_code: ExitSignal,
  pub init_load: InitLoadSignal,
}

pub struct FlashpointService {
  pub db: DbState,
  pub initialized: bool,
  pub base_path: String,
  pub config: Config,
  pub prefs: Preferences,
  #[cfg(feature = "services")]
  pub services_info: Services,
  pub signals: FlashpointSignals,
}

impl FlashpointService {
  pub async fn new(base_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let config_path = base_path.join("config.json");
    println!("Config Path: {:?}", config_path.canonicalize()?);
    let config = load_config_file(&config_path).await?;

    let prefs_path = base_path
      .join(config.flashpoint_path.clone())
      .join("preferences.json");
    println!("Prefs Path: {:?}", prefs_path.canonicalize()?);
    let prefs = load_prefs_file(&prefs_path).await?;

    let db_path = base_path
      .join(config.flashpoint_path.clone())
      .join(prefs.json_folder_path.clone())
      .join("flashpoint.sqlite")
      .to_str()
      .unwrap()
      .to_string();

    #[cfg(feature = "services")]
    let services_info = load_services(
      &base_path
        .join(config.flashpoint_path.clone())
        .join(prefs.json_folder_path.clone())
        .join("services.json"),
    )
    .await?;
    Ok(Self {
      db: flashpoint_database::initialize(&db_path)?,
      initialized: false,
      base_path: base_path.canonicalize()?.to_str().unwrap().to_string(),
      config,
      prefs: prefs.clone(),
      #[cfg(feature = "services")]
      services_info,
      signals: FlashpointSignals {
        exit_code: ExitSignal::new(),
        init_load: InitLoadSignal::new(),
      },
    })
  }

  pub fn init(&mut self) {
    if self.initialized {
      return;
    }
    // TODO
    self.signals.init_load.emit(InitLoad::Services);
    // TODO
    self.signals.init_load.emit(InitLoad::Database);
    // TODO
    self.signals.init_load.emit(InitLoad::Playlists);
    // TODO
    self.signals.init_load.emit(InitLoad::Extensions);
    // TODO
    self.signals.init_load.emit(InitLoad::ExecMappings);
    // TODO
    self.signals.init_load.emit(InitLoad::Curate);
    self.initialized = true;
  }

  #[cfg(feature = "websocket")]
  pub fn ping(&self, data: String) -> String {
    data
  }

  #[cfg(feature = "websocket")]
  pub async fn listen(self) {
    use std::process::exit;

    let registers = Arc::new(Mutex::new(WebsocketRegisters {
      init_data: Box::new(|fp_service, _: ()| {
        Ok(WebsocketRes {
          data: InitDataRes {
            config: fp_service.config.clone(),
            prefs: fp_service.prefs.clone(),
            #[cfg(feature = "services")]
            services_info: fp_service.services_info.clone(),
          },
        })
      }),
      view_all_games: Box::new(|mut fp_service, _| {
        Ok(WebsocketRes {
          data: flashpoint_database::game::view_all_games(&mut fp_service.db),
        })
      }),
      all_games: Box::new(|mut fp_service, _| {
        Ok(WebsocketRes {
          data: flashpoint_database::game::find_all_games(&mut fp_service.db),
        })
      }),
      all_tag_categories: Box::new(|mut fp_service, _| {
        Ok(WebsocketRes {
          data: flashpoint_database::tag::find_tag_categories(&mut fp_service.db),
        })
      }),
      create_tag_category: Box::new(|mut fp_service, data| {
        Ok(WebsocketRes {
          data: flashpoint_database::tag::create_tag_category(&mut fp_service.db, data)?,
        })
      }),
      find_tag_by_name: Box::new(|mut fp_service, data| {
        let (tag, aliases) = flashpoint_database::tag::find_tag_by_name(&mut fp_service.db, data)?;
        let primary = aliases
          .iter()
          .find(|a| a.id == tag.primary_alias_id.unwrap())
          .unwrap();
        Ok(WebsocketRes {
          data: TagRes {
            id: tag.id,
            date_modified: tag.date_modified,
            category_id: tag.category_id,
            description: tag.description,
            primary_alias: primary.clone(),
            aliases,
          },
        })
      }),
      add: Box::new(|_, data| {
        Ok(WebsocketRes {
          data: data.first + data.second,
        })
      }),
    }));

    // Create listener state
    let fp_state = Arc::new(Mutex::new(self));
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr)
      .await
      .expect("Failed to bind websocket listener");

    println!("Listening on: {}", addr);

    // Capture Ctrl+C to trigger Exit signal on FlashpointService
    let ctrlc_fp_state = fp_state.clone();
    ctrlc::set_handler(move || {
      println!("Received Exit Signal, shutting down...");
      match ctrlc_fp_state.lock() {
        Ok(fp) => {
          fp.signals.exit_code.emit(0);
        }
        Err(e) => {
          println!("Error exiting FlashpointService: {}", e);
          exit(1);
        }
      }
    })
    .expect("Error setting Ctrl-C handler");

    // Each connection spawns own tokio task
    while let Ok((stream, addr)) = listener.accept().await {
      let fp_lock = fp_state.clone();
      tokio::spawn(handle_connection(
        fp_lock,
        state.clone(),
        stream,
        addr,
        registers.clone(),
      ));
    }
  }

  pub fn exit(&self) {
    self.signals.exit_code.emit(0);
  }
}

#[cfg(feature = "services")]
async fn load_services(services_path: &Path) -> Result<Services, Box<dyn std::error::Error>> {
  println!("Services Path: {:?}", services_path.canonicalize()?);
  let services = load_services_file(&services_path).await.unwrap();
  Ok(services)
}

#[cfg(feature = "services")]
async fn load_services_file(path: &Path) -> Result<Services, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let services: Services = serde_json::from_str(std::str::from_utf8(&contents)?)?;
  Ok(services)
}

async fn load_config_file(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let config: Config = serde_json::from_str(std::str::from_utf8(&contents)?)?;
  Ok(config)
}

async fn load_prefs_file(path: &Path) -> Result<Preferences, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let prefs: Preferences = serde_json::from_str(std::str::from_utf8(&contents)?)?;
  Ok(prefs)
}

#[cfg(feature = "websocket")]
async fn handle_connection(
  fp_service: Arc<Mutex<FlashpointService>>,
  peer_map: PeerMap,
  raw_stream: TcpStream,
  addr: SocketAddr,
  registers: Arc<Mutex<WebsocketRegisters>>,
) {
  use futures_util::{future, pin_mut, TryStreamExt};
  use serde_json::Value;

  println!("Incoming TCP connection from: {}", addr);

  let ws_stream = tokio_tungstenite::accept_async(raw_stream)
    .await
    .expect("Error during the websocket handshake occurred");

  println!("WebSocket connection established: {}", addr);

  // Insert the write part of this peer to the peer map.
  let (tx, rx) = unbounded();
  peer_map.lock().unwrap().insert(addr, tx);

  let (outgoing, incoming) = ws_stream.split();

  let broadcast_incoming = incoming.try_for_each(|msg| {
    println!("{}: {:?}", &addr, msg.to_text().unwrap());
    // Create copies of the state locks to use inside async function
    let fp_service = fp_service.clone();
    let registers = registers.clone();
    let peer_map = peer_map.clone();
    async move {
      // Take control of registers and peer states
      let registers = registers.lock().unwrap();
      let peers = peer_map.lock().unwrap();

      // Deserialize incoming message
      let r: Result<Value, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());
      match r {
        Ok(rec_msg) => {
          let op = rec_msg["op"].as_str().unwrap();
          let data = rec_msg["data"].clone();

          // Execute the registered function
          let res = execute_register(registers, fp_service, op, data);
          let res_msg =
            Message::text(res.unwrap_or_else(|err| format!("{{ \"error\": \"{:?}\" }}", err)));

          // Broadcast response only to ourselves
          let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr == &&addr)
            .map(|(_, ws_sink)| ws_sink);

          for recp in broadcast_recipients {
            recp.unbounded_send(res_msg.clone()).unwrap();
          }
        }
        Err(e) => {
          println!("Error deserializing message: {}", e);
          let res_msg = Message::text("{ \"error\": \"error deserializing message\" }".to_string());

          // Broadcast response only to ourselves
          let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr == &&addr)
            .map(|(_, ws_sink)| ws_sink);

          for recp in broadcast_recipients {
            recp.unbounded_send(res_msg.clone()).unwrap();
          }
        }
      }

      Ok(())
    }
  });

  let receive_from_others = rx.map(Ok).forward(outgoing);

  pin_mut!(broadcast_incoming, receive_from_others);
  future::select(broadcast_incoming, receive_from_others).await;

  println!("{} disconnected", &addr);
  peer_map.lock().unwrap().remove(&addr);
}

#[cfg(feature = "websocket")]
fn execute_register(
  registers: std::sync::MutexGuard<WebsocketRegisters>,
  fp_service: Arc<Mutex<FlashpointService>>,
  op: &str,
  data: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
  use flashpoint_database::tag::InsertableTagCategory;

  let res_str: String;
  match op {
    "init" => {
      println!("Init Data");
      ws_execute!(registers.init_data, res_str, fp_service);
    }
    "view_all_games" => {
      println!("All Games");
      ws_execute!(registers.view_all_games, res_str, fp_service);
    }
    "all_games" => {
      println!("All Games");
      ws_execute!(registers.all_games, res_str, fp_service);
    }
    "all_tag_categories" => {
      println!("All Categories");
      ws_execute!(registers.all_tag_categories, res_str, fp_service);
    }
    "create_tag_category" => {
      println!("Create Tag Category");
      ws_execute!(
        &data,
        registers.create_tag_category,
        res_str,
        fp_service,
        InsertableTagCategory
      );
    }
    "find_tag_by_name" => {
      println!("Find Tag By Name");
      ws_execute!(
        &data,
        registers.find_tag_by_name,
        res_str,
        fp_service,
        String
      );
    }
    "add" => {
      println!("Add");
      ws_execute!(&data, registers.add, res_str, fp_service, AddRecv);
    }
    _ => {
      res_str = "{ \"error\": \"unknown operation\" }".to_string();
    }
  };
  Ok(res_str)
}
