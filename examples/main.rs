use std::{env, error::Error};

use unified::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let unifi = Unified::from_credentials(
        Scheme::Https,
        args.get(1).unwrap(),
        args.get(2).unwrap(),
        args.get(3).unwrap(),
    )
    .await?;

    let client = unifi.site(SiteRef::Description("ACME Corp - HQ")).await?;

    println!("{:?}", client);

    Ok(())
}
