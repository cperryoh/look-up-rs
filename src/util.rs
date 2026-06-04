use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct Config {
    pub lat: f32,
    pub lon: f32,
    pub distance: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Aircraft {
    #[serde(rename = "hex")]
    pub icao_address: String,

    #[serde(rename = "type")]
    pub source_type: String,

    #[serde(rename = "flight")]
    pub callsign: String,

    #[serde(rename = "r")]
    pub registration: String,

    #[serde(rename = "t")]
    pub aircraft_type: String,

    #[serde(rename = "alt_baro")]
    pub altitude_barometric: Option<i32>,

    #[serde(rename = "alt_geom")]
    pub altitude_geometric: Option<i32>,

    #[serde(rename = "calc_track")]
    pub track: Option<f32>,

    #[serde(rename = "category")]
    pub aircraft_category: String,

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
