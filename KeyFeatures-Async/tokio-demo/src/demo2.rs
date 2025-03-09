use log::Level;
use tokio::{io::AsyncReadExt, time};

async fn sleeper() {
  log::info!("Sleeping");
  time::sleep(time::Duration::from_secs(1)).await;
  log::info!("Awake!");
}

async fn reader() {
  log::info!("Reading some data");
  let mut f = tokio::fs::File::open("a.csv").await.unwrap();
  let mut contents = vec![];
  f.read_to_end(&mut contents).await.unwrap();
  log::info!("Read  {} bytes", contents.len());
}

async fn run() {
  sleeper().await;
  reader().await;
}

async fn run2() {
  tokio::join!(sleeper(), reader());
}

fn main() {
  simple_logger::init_with_level(Level::Info).unwrap();
  let rt = tokio::runtime::Runtime::new().unwrap();
  let future = run();
  rt.block_on(future);
}
