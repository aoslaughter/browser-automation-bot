use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub webdriver_url: String,
    pub user: UserConfig,
    pub search: SearchConfig,
    pub reservation: ReservationConfig,
    pub site_config: SiteConfig,
    pub home_dom: HomeDomConfig,
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SearchConfig {
    pub zipcode: String,
}

#[derive(Deserialize)]
pub struct ReservationConfig {
    pub weeks: i8,
    pub time: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SiteConfig {
    pub base_url: String,
}

#[derive(Deserialize)]
pub struct HomeDomConfig {
    pub form_element: String,
    pub location_element: String,
    pub submit_element: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Read file contents
        let content = fs::read_to_string(path)?;

        // Parse TOML
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
