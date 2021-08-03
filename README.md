# unified

_Work in progress._

Rust library to query and manipulate the configuration of a UniFi network through its controller.

## Example

```rust
let unifi = Unified::new("unifi.acme.corp")
  .auth("apiuser", "apipassword")
  .await?;

for network in unifi.networks("default").await? {
  println!(
    "{} ({})",
    network.name,
    network.subnet.map(|ip| ip.to_string()).unwrap_or_else(|| "-".to_string()),
  );
}
```

### UDM Pro

The controller embedded into the UniFi Dream Machine Pro uses a different API, therefore you should opt into it when creating your `unified` instance, like so:

```rust
let unifi = Unified::new("unifi.acme.corp")
  .udm_pro()
  .auth("apiuser", "apipassword")
  .await?;
```

## Example (actual)

This example lives in `examples/main.rs`.

```shell
$ cargo run --example main -- unifi.acme.corp apiuser apipassword
1/ HEALTH

 System | Status
 WWW    | ✓
 WAN    | ✓
 LAN    | ✓
 WLAN   | ✓
 VPN    | ✗

2/ ALERTS

 Time                | Device              | System | Message
 2021-07-31 23:19:55 | USW-Pro-48-PoE:1768 | LAN    | Switch[24:5a:4c:00:00:00] was disconnected

3/ EVENTS

 Time                | Device          | System | SSID | Message
 2021-08-01 11:31:08 | Access Point 04 | WLAN   | -    | AP[24:5a:4c:00:00:00] was connected
 2021-08-01 11:31:08 | Access Point 04 | WLAN   | -    | AP[24:5a:4c:00:00:00] was automatically readopted
 2021-08-01 11:31:04 | Access Point 03 | WLAN   | -    | AP[24:5a:4c:00:00:00] was connected
 2021-08-01 11:31:04 | Access Point 03 | WLAN   | -    | AP[24:5a:4c:00:00:00] was automatically readopted
 2021-08-01 11:31:03 | Switch          | LAN    | -    | Switch[24:5a:4c:00:00:00] was connected
 2021-08-01 11:31:03 | Switch          | LAN    | -    | Switch[24:5a:4c:00:00:00] was automatically readopted
 2021-08-01 11:31:02 | Access Point 01 | WLAN   | -    | AP[24:5a:4c:00:00:00] was connected
 2021-08-01 11:31:02 | Access Point 02 | WLAN   | -    | AP[24:5a:4c:00:00:00] was connected
 2021-08-01 11:31:02 | Access Point 01 | WLAN   | -    | AP[24:5a:4c:00:00:00] was automatically readopted
 2021-08-01 11:31:02 | Access Point 02 | WLAN   | -    | AP[24:5a:4c:00:00:00] was automatically readopted

4/ DEVICES

 Name             | State     | Model   | Version
 Access Point 02  | Connected | UAL6    | 5.60.9.12980
 Access Point 03  | Connected | UAL6    | 5.60.9.12980
 Switch           | Connected | US48PRO | 5.64.8.13083
 ACME - HQ        | Connected | UDMPRO  | 1.10.0.3686
 Access Point 01  | Connected | UAL6    | 5.60.9.12980
 Access Point 04  | Connected | UAL6    | 5.60.9.12980

5/ NETWORKS

 Name             | Subnet            | VLAN
 WAN1             | -                 | -
 WAN2             | -                 | -
 000 - Management | 10.10.10.254/24   | -
 110 - Employees  | 10.10.11.254/24   | 110
 120 - Guests     | 10.10.12.254/24   | 120

6/ WIRELESS NETWORKS

 SSID             | Network ID               | Passphrase
 ACME             | abcdef1234567890abcdef12 | amazingpassphrase
 ACME - Guests    | abcdef1234567890abcdef12 | ACME-hello
```
