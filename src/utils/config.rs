use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub webdriver_url: String,
    pub user: UserConfig,
    pub search: SearchConfig,
    pub reservation: ReservationConfig,
    pub site: SiteConfig,
    pub home_dom: HomeDomConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchConfig {
    pub param_order: Vec<String>,
    #[serde(flatten)]
    pub location: String,
    pub date: String,
    pub time: String,
    pub sport: String,
    pub commit: String,
    pub utf8: String,
    pub params: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReservationConfig {
    pub weeks: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SiteConfig {
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeDomConfig {
    pub form_element: String,
    pub location_element: String,
    pub date_element: String,
    pub date_picker: String,
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

    pub fn to_search_params(&self) -> HashMap<String, String> {
        let mut url_params = HashMap::new();

        // Process parameters in the order specified
        for param_name in &self.search.param_order {
            if param_name == "param_order" {
                continue;
            }

            // Get value if it exists
            if let Some(value) = self.search.params.get(param_name) {
                let key = format!("search[{}]", param_name);

                // Convert value to string based on its type
                let str_value = match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "".to_string(),
                    _ => continue,
                };

                url_params.insert(key, str_value);
            }
        }

        url_params
    }

    pub fn set_date(&mut self, date: String) {
        self.search.date = date;
    }

    pub fn set_time(&mut self, time: String) {
        self.search.time = time;
    }
}
