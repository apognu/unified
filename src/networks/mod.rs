mod list;

use ipnet::IpNet;

pub enum NetworkRef<'r> {
    Id(&'r str),
    Name(&'r str),
    Domain(&'r str),
    Subnet(&'r str),
}

#[derive(Debug)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub purpose: String,
    pub subnet: Option<IpNet>,
    pub domain: Option<String>,
    pub vlan: Option<u16>,
}
