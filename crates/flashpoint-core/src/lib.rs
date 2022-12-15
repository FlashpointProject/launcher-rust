use cfg_if::cfg_if;
use flashpoint_config::types::{Config, Preferences};
use std::collections::HashMap;
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
  pub initialized: bool,
  pub base_path: String,
  pub config: Config,
  pub prefs: Preferences,
  #[cfg(feature = "services")]
  pub services_info: Services,
  pub signals: FlashpointSignals,
}

impl FlashpointService {
  pub async fn new(base_path_str: String) -> Self {
    let base_path = Path::new(&base_path_str);

    let config_path = base_path.join("config.json").as_os_str().to_str().unwrap().to_string();
    println!("Config Path: {}", config_path);
    let config = load_config_file(&config_path).await.unwrap();

    let prefs_path = base_path
      .parent()
      .unwrap()
      .join("preferences.json")
      .as_os_str()
      .to_str()
      .unwrap()
      .to_string();
    println!("Prefs Path: {}", prefs_path);
    let prefs = load_prefs_file(&prefs_path).await.unwrap();

    #[cfg(feature = "services")]
    let services_info = load_services(&base_path_str, &prefs).await.unwrap();

    Self {
      initialized: false,
      base_path: base_path_str.clone(),
      config,
      prefs: prefs.clone(),
      #[cfg(feature = "services")]
      services_info,
      signals: FlashpointSignals {
        exit_code: ExitSignal::new(),
        init_load: InitLoadSignal::new(),
      },
    }
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
      init_data: Box::new(|fp_service, _: ()| InitDataRes {
        config: fp_service.config.clone(),
        prefs: fp_service.prefs.clone(),
        #[cfg(feature = "services")]
        services_info: fp_service.services_info.clone(),
      }),
    }));

    // Create listener state
    let fp_state = Arc::new(Mutex::new(self));
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind websocket listener");

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
      tokio::spawn(handle_connection(fp_lock, state.clone(), stream, addr, registers.clone()));
    }
  }

  pub fn exit(&self) {
    self.signals.exit_code.emit(0);
  }
}

#[cfg(feature = "services")]
async fn load_services(base_path: &str, prefs: &Preferences) -> Result<Services, Box<dyn std::error::Error>> {
  let p = Path::new(&base_path);
  let services_path = p
    .parent()
    .unwrap()
    .join(prefs.json_folder_path.clone())
    .join("services.json")
    .as_os_str()
    .to_str()
    .unwrap()
    .to_string();
  println!("Services Path: {}", services_path);
  let services = load_services_file(&services_path).await.unwrap();
  Ok(services)
}

#[cfg(feature = "services")]
async fn load_services_file(path: &str) -> Result<Services, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let services: Services = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
  Ok(services)
}

async fn load_config_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let config: Config = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
  Ok(config)
}

async fn load_prefs_file(path: &str) -> Result<Preferences, Box<dyn std::error::Error>> {
  let mut file = File::open(path).await?;
  let mut contents = vec![];
  file.read_to_end(&mut contents).await?;
  let prefs: Preferences = serde_json::from_str(std::str::from_utf8(&contents).unwrap())?;
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
      // Take control of registers and fp states
      let registers = registers.lock().unwrap();
      let mut fp_service = fp_service.lock().unwrap();
      let peers = peer_map.lock().unwrap();

      // Deserialize incoming message
      let r: Result<Value, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());
      match r {
        Ok(rec_msg) => {
          let op = rec_msg["op"].as_str().unwrap();
          let data = rec_msg["data"].clone();
          let res_str: String;
          match op {
            "init" => {
              println!("Init Data");
              ws_execute!(registers.init_data, res_str, fp_service);
            }
            _ => {
              res_str = "{ \"error\": \"unknown operation\" }".to_string();
            }
          };
          let res_msg = Message::text(res_str);

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
