mod list;

pub enum SiteRef<'r> {
    Id(&'r str),
    Name(&'r str),
    Description(&'r str),
}

#[derive(Debug)]
pub struct Site {
    pub id: String,
    pub name: String,
    pub description: String,
    pub alarms: u64,
    pub health: SiteHealth,
}

#[derive(Debug, Default)]
pub struct SiteHealth {
    pub www: bool,
    pub wan: bool,
    pub lan: bool,
    pub wlan: bool,
    pub vpn: bool,
}
