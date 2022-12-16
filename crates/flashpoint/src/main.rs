use flashpoint_core::{signals::Signal, FlashpointService};
use std::{path::Path, process::exit};

#[tokio::main]
async fn main() {
  println!("-- Flashpoint Service --");

  let base_path = Path::new(r"C:\Users\colin\Downloads\Flashpoint 11 Infinity\Launcher");
  let mut fp_service = FlashpointService::new(base_path).await;
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
