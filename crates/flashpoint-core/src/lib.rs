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
#[cfg(feature = "websocket")]
pub struct WebsocketMessage {}

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
  pub base_path: String,
  pub config: Config,
  pub prefs: Preferences,
  #[cfg(feature = "services")]
  pub services_info: Services,
  pub signals: FlashpointSignals,
  #[cfg(feature = "websocket")]
  pub registers: WebsocketRegisters,
}

impl FlashpointService {
  pub async fn new(base_path_str: String) -> Self {
    let base_path = Path::new(&base_path_str);

    let config_path = base_path
      .join("config.json")
      .as_os_str()
      .to_str()
      .unwrap()
      .to_string();
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

    let registers = WebsocketRegisters {
      ping: WebsocketRegister {
        cls: Box::new(|ping| ping),
      },
    };

    Self {
      base_path: base_path_str.clone(),
      config,
      prefs: prefs.clone(),
      #[cfg(feature = "services")]
      services_info: load_services(&base_path_str, &prefs).await.unwrap(),
      signals: FlashpointSignals {
        exit_code: ExitSignal::new(),
        init_load: InitLoadSignal::new(),
      },
      registers,
    }
  }

  pub async fn init(&mut self) {
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
  }

  #[cfg(feature = "websocket")]
  pub async fn listen(self) {
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
      ctrlc_fp_state.lock().unwrap().signals.exit_code.emit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Each connection spawns own tokio task
    while let Ok((stream, addr)) = listener.accept().await {
      let fp_lock = fp_state.clone();
      tokio::spawn(handle_connection(fp_lock, state.clone(), stream, addr));
    }
  }

  pub fn exit(&self) {
    self.signals.exit_code.emit(0);
  }
}

#[cfg(feature = "services")]
async fn load_services(
  base_path: &str,
  prefs: &Preferences,
) -> Result<Services, Box<dyn std::error::Error>> {
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
  fp_mutex_guard: Arc<Mutex<FlashpointService>>,
  peer_map: PeerMap,
  raw_stream: TcpStream,
  addr: SocketAddr,
) {
  use futures_util::{future, pin_mut, TryStreamExt};

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
    let fp_lock = fp_mutex_guard.clone();
    async move {
      // Take control of fp state
      let fp = fp_lock.lock().unwrap();
      println!("JSON Folder Path: {}", fp.prefs.json_folder_path);
      Ok(())
    }
  });

  let receive_from_others = rx.map(Ok).forward(outgoing);

  pin_mut!(broadcast_incoming, receive_from_others);
  future::select(broadcast_incoming, receive_from_others).await;

  println!("{} disconnected", &addr);
  peer_map.lock().unwrap().remove(&addr);
}
