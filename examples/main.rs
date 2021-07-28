use std::{env, error::Error};

use unified::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  let unifi = Unified::auth(Scheme::Https, args.get(1).unwrap(), args.get(2).unwrap(), args.get(3).unwrap()).await?;
  let site = unifi.site(SiteRef::Description("ACME Corp - HQ")).await?;

  println!("{:?}", site);

  for device in unifi.devices("default").await? {
    println!("{}", device.name);
  }

  Ok(())
}
