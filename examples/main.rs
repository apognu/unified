use std::{env, error::Error};

use unified::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let host = args.get(1).unwrap();
  let username = args.get(2).unwrap();
  let password = args.get(3).unwrap();

  let unifi = Unified::new(host).auth(username, password).await?;

  let site = unifi.site(SiteRef::Description("ACME Corp - HQ")).await?;

  println!("{:#?}", site);

  for device in unifi.devices("default").await? {
    println!("{}", device.name);
  }

  Ok(())
}
