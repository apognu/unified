use std::{env, error::Error};

use colored::Colorize;
use prettytable::*;
use unified::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let host = args.get(1).unwrap();
  let username = args.get(2).unwrap();
  let password = args.get(3).unwrap();

  let unifi = Unified::new(host).no_tls_verify().udm_pro().auth(username, password).await?;

  print_section("1/ HEALTH");

  let mut table = new_table(row![b -> "System", b -> "Status"]);
  if let Some(site) = unifi.site(SiteRef::Name("default")).await? {
    table.add_row(row!["WWW", if site.health.www { "✓".green() } else { "✗".red() }]);
    table.add_row(row!["WAN", if site.health.wan { "✓".green() } else { "✗".red() }]);
    table.add_row(row!["LAN", if site.health.lan { "✓".green() } else { "✗".red() }]);
    table.add_row(row!["WLAN", if site.health.wlan { "✓".green() } else { "✗".red() }]);
    table.add_row(row!["VPN", if site.health.vpn { "✓".green() } else { "✗".red() }]);
  }
  table.printstd();

  print_section("2/ ALERTS");

  let mut table = new_table(row![b -> "Time", b -> "Device", b -> "System", b -> "Message"]);
  for alert in unifi.alerts("default", Some(10)).await? {
    table.add_row(row![alert.time, alert.device.unwrap_or_else(|| "-".to_string()), alert.subsystem, alert.message]);
  }
  table.printstd();

  print_section("3/ EVENTS");

  let mut table = new_table(row![b -> "Time", b -> "Device", b -> "System", b -> "SSID", b -> "Message"]);
  for event in unifi.events("default", Some(10)).await? {
    table.add_row(row![
      event.time,
      event.device.unwrap_or_else(|| "-".to_string()),
      event.subsystem,
      event.ssid.unwrap_or_else(|| "-".to_string()),
      event.message
    ]);
  }
  table.printstd();

  print_section("4/ DEVICES");

  let mut table = new_table(row![b -> "Name", b -> "State", b -> "Model", b -> "Version"]);
  for device in unifi.devices("default").await? {
    table.add_row(row![device.name, device.state, device.model, device.version]);
  }
  table.printstd();

  print_section("5/ NETWORKS");

  let mut table = new_table(row![b -> "Name", b -> "Subnet", b -> "VLAN"]);
  for network in unifi.networks("default").await? {
    table.add_row(row![
      network.name,
      network.subnet.map(|ip| ip.to_string()).unwrap_or_else(|| "-".to_string()),
      network.vlan.map(|vlan| vlan.to_string()).unwrap_or_else(|| "-".to_string())
    ]);
  }
  table.printstd();

  print_section("6/ WIRELESS NETWORKS");
  let mut table = new_table(row![b -> "SSID", b -> "Network ID", b -> "Passphrase"]);
  for network in unifi.wireless_networks("default").await? {
    table.add_row(row![
      network.name,
      network.network.unwrap_or_else(|| "-".to_string()),
      network.passphrase.unwrap_or_else(|| "-".to_string())
    ]);
  }
  table.printstd();

  Ok(())
}

fn print_section(title: &str) {
  println!();
  println!("{}", title.green());
  println!();
}

fn new_table(headers: Row) -> Table {
  let mut table = Table::new();
  table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
  table.add_row(headers);

  table
}
