use std::{env, error::Error};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use unified::*;

const PREFIX: &str = "ACME-";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let host = args.get(1).unwrap();
  let username = args.get(2).unwrap();
  let password = args.get(3).unwrap();

  let unifi = Unified::new(host).auth(username, password).await?;

  let suffix: String = thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
  let passphrase = format!("{}{}", PREFIX, suffix);

  if let Some(mut network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME - Guests")).await? {
    network.passphrase = Some(passphrase);
    network.update().await?;
  }

  Ok(())
}
