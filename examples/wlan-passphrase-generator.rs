///! Usage:
///!
///! ```
///! $ cargo run --example wlan-passphrase-generator -- <HOST> <API_USER> <API_PASSWORD> <SSID>
///! ```
use std::{env, error::Error};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use unified::*;

const PREFIX: &str = "ACME-";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let (host, username, password, ssid) = get_args();
  let unifi = Unified::new(&host).auth(&username, &password).await?;

  let suffix: String = thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
  let passphrase = format!("{}{}", PREFIX, suffix);

  if let Some(mut network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid(&ssid)).await? {
    network.passphrase = Some(passphrase);
    network.update().await?;
  }

  Ok(())
}

fn get_args() -> (String, String, String, String) {
  let args: Vec<String> = env::args().collect();
  let host = args.get(1).unwrap().to_string();
  let username = args.get(2).unwrap().to_string();
  let password = args.get(3).unwrap().to_string();
  let ssid = args.get(4).unwrap().to_string();

  (host, username, password, ssid)
}
