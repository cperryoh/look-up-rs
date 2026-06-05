use crate::{HASSIO_API_KEY, HASSIO_URL, errors::Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Deserialize, Clone)]
pub struct Config {
    pub distance: f32,
    pub notify_entity: String,
    pub min_height: u32,
    pub aircraft_types: Vec<String>,
    pub location_entity: String,
    pub static_location: Location,
    pub update_interval_min: u64,
}
#[derive(Clone, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Aircraft {
    #[serde(rename = "hex")]
    pub icao_address: String,

    #[serde(rename = "type")]
    pub source_type: String,

    #[serde(rename = "flight")]
    pub callsign: Option<String>,

    #[serde(rename = "r")]
    pub registration: Option<String>,

    #[serde(rename = "t")]
    pub aircraft_type: Option<String>,

    #[serde(rename = "alt_baro")]
    pub altitude_barometric: Option<u32>,

    #[serde(rename = "alt_geom")]
    pub altitude_geometric: Option<i32>,

    #[serde(rename = "calc_track")]
    pub track: Option<f32>,

    #[serde(rename = "category")]
    pub aircraft_category: Option<String>,

    #[serde(rename = "nav_qnh")]
    pub nav_qnh: Option<f32>,

    #[serde(rename = "nav_altitude_mcp")]
    pub nav_altitude_mcp: Option<i32>,

    #[serde(rename = "lat")]
    pub latitude: Option<f64>,

    #[serde(rename = "lon")]
    pub longitude: Option<f64>,

    #[serde(rename = "nic")]
    pub navigational_integrity_category: Option<i32>,

    #[serde(rename = "rc")]
    pub rc: Option<i32>,

    #[serde(rename = "seen_pos")]
    pub seen_position_seconds: Option<f32>,

    #[serde(rename = "version")]
    pub adsb_version: Option<i32>,

    #[serde(rename = "nic_baro")]
    pub nic_baro: Option<i32>,

    #[serde(rename = "nac_p")]
    pub nac_position: Option<i32>,

    #[serde(rename = "sil")]
    pub sil: Option<i32>,

    #[serde(rename = "sil_type")]
    pub sil_type: Option<String>,

    #[serde(rename = "gva")]
    pub geometric_vertical_accuracy: Option<i32>,

    #[serde(rename = "sda")]
    pub sda: Option<i32>,

    #[serde(rename = "alert")]
    pub alert: Option<i32>,

    #[serde(rename = "spi")]
    pub special_position_identification: Option<i32>,

    #[serde(rename = "mlat")]
    pub mlat: Vec<serde_json::Value>,

    #[serde(rename = "tisb")]
    pub tisb: Vec<serde_json::Value>,

    #[serde(rename = "messages")]
    pub message_count: i32,

    #[serde(rename = "seen")]
    pub seen_seconds: f32,

    #[serde(rename = "rssi")]
    pub signal_strength_dbm: f32,

    #[serde(rename = "dst")]
    pub distance_km: f32,

    #[serde(rename = "dir")]
    pub direction_degrees: f32,
}

pub fn bearing_to_target(
    observer_lat: f64,
    observer_lon: f64,
    target_lat: f64,
    target_lon: f64,
) -> f32 {
    let lat1 = observer_lat.to_radians();
    let lat2 = target_lat.to_radians();
    let dlon = (target_lon - observer_lon).to_radians();

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();

    let bearing = y.atan2(x).to_degrees();
    ((bearing + 360.0) % 360.0) as f32
}
pub fn degrees_to_cardinal(degrees: f32) -> &'static str {
    let normalized = ((degrees + 22.5) % 360.0) as u32 / 45;
    match normalized {
        0 => "N",
        1 => "NE",
        2 => "E",
        3 => "SE",
        4 => "S",
        5 => "SW",
        6 => "W",
        7 => "NW",
        _ => "?",
    }
}
pub async fn get_origin_location(config: &Config) -> Result<Location> {
    let endpoint = format!("/api/states/{}", config.location_entity);
    let url = format!("{}{}", &*HASSIO_URL, endpoint);
    let client = Client::new();
    let req = client
        .get(url)
        .bearer_auth(&*HASSIO_API_KEY)
        .send()
        .await?
        .error_for_status()?;
    let res = req.text().await?;
    let res: Value = serde_json::from_str(&res).unwrap();
    Ok(Location {
        lat: res
            .get("attributes")
            .unwrap()
            .get("latitude")
            .unwrap()
            .as_f64()
            .unwrap(),
        lon: res
            .get("attributes")
            .unwrap()
            .get("longitude")
            .unwrap()
            .as_f64()
            .unwrap(),
    })
}
