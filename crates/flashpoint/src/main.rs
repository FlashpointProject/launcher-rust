use flashpoint_core::{signals::Signal, FlashpointService};
use std::process::exit;

#[tokio::main]
async fn main() {
  println!("-- Flashpoint Service --");

  let mut fp_service = FlashpointService::new(r"C:\Users\colin\Downloads\Flashpoint 11 Infinity\Launcher".to_string()).await;
  println!("Created Flashpoint Service");

  fp_service.signals.init_load.connect(|_, data| {
    println!("Initialized: {:?}", data);
  });

  fp_service.signals.exit_code.connect(|_, data: i32| {
    println!("Exit Code: {}", data);
    exit(data);
  });

  fp_service.listen().await;
}
