mod errors;
mod util;

use errors::Result;
use std::fmt::format;

use serde::Deserialize;
use serde_json::Value;
const API: &str = "https://api.adsb.lol/v2";
use crate::util::{Aircraft, Config};

async fn get_data(config: &Config) -> Result<Vec<Aircraft>> {
    let url = format!(
        "{API}/lat/{}/lon/{}/dist/{}",
        config.lat, config.lon, config.distance
    );
    let req = reqwest::get(url).await?.error_for_status()?;
    let res = req.text().await?;
    let res: Value = serde_json::from_str(&res)?;
    let aircraft_array = res.get("ac").unwrap().as_array().unwrap();
    let aircraft_array = aircraft_array
        .into_iter()
        .map(|a| serde_json::from_value(a.clone()).unwrap())
        .collect::<Vec<_>>();

    Ok(aircraft_array)
}

fn load_config() -> Config {
    let cfg_str = std::fs::read_to_string("./config.toml").unwrap();
    toml::from_str(&cfg_str).unwrap()
}
#[tokio::main]
async fn main() {
    let config = load_config();
    let aircraft = get_data(&config).await.unwrap();
    println!("{:?}", aircraft[0])
}
