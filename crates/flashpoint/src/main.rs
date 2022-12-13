use flashpoint_core::{signals::Signal, FlashpointService};
use std::process::exit;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
  let (_tx, rx) = oneshot::channel();
  let mut fp_service =
    FlashpointService::new(r"C:\Users\colin\Downloads\Flashpoint 11 Infinity\Launcher".to_string())
      .await;
  println!("Created Flashpoint Service");

  fp_service.signals.exit_code.connect(|_, data: i32| {
    println!("Exit Code: {}", data);
    exit(data);
  });

  ctrlc::set_handler(move || {
    println!("Received Exit Signal, shutting down...");
    fp_service.signals.exit_code.emit(0);
  })
  .expect("Error setting Ctrl-C handler");

  match rx.await {
    Ok(code) => exit(code),
    Err(e) => println!("Error: {}", e),
  }
}
