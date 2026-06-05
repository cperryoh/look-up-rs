mod errors;
mod notification_senders;
mod util;

use errors::Result;
use std::{env, sync::LazyLock, time::Duration};

use serde_json::Value;
const API: &str = "https://api.adsb.lol/v2";
use crate::{
    notification_senders::send_notification,
    util::{
        Aircraft, Config, Location, bearing_to_target, degrees_to_cardinal, get_origin_location,
    },
};

static HASSIO_API_KEY: LazyLock<String> = LazyLock::new(|| env::var("HASSIO_API_KEY").unwrap());
static HASSIO_URL: LazyLock<String> = LazyLock::new(|| env::var("HASSIO_URL").unwrap());

async fn get_data(config: &Config, location: &Location) -> Result<Vec<Aircraft>> {
    let url = format!(
        "{API}/lat/{}/lon/{}/dist/{}",
        location.lat,
        location.lon,
        miles_to_nm(&config.distance)
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
fn miles_to_nm(miles: &f32) -> f32 {
    miles / 1.151
}
fn is_interesting(config: &Config, aircraft: &Aircraft) -> bool {
    let military_types = &config.aircraft_types;

    // Known military type is a strong signal
    if let Some(atype) = &aircraft.aircraft_type {
        if military_types.iter().any(|t| atype.contains(t)) {
            return true;
        }
    }

    if let Some(alt) = aircraft.altitude_barometric {
        if alt > config.min_height {
            return true;
        }
    }

    false
}
fn load_config() -> Config {
    let cfg_str = std::fs::read_to_string("./config.toml").unwrap();
    toml::from_str(&cfg_str).unwrap()
}
async fn check_for_planes(config: &Config, location: &Location) -> Result<Vec<Aircraft>> {
    Ok(get_data(config, location)
        .await?
        .into_iter()
        .filter(|a| is_interesting(&config, a))
        .collect::<Vec<_>>())
}
async fn send_notification_for_interesting_plane(
    config: &Config,
    origin_location: &Location,
    aircraft: &Aircraft,
) -> Result<()> {
    let bearing = bearing_to_target(
        origin_location.lat,
        origin_location.lon,
        aircraft.latitude.unwrap_or(0.0),
        aircraft.longitude.unwrap_or(0.0),
    );
    let direction = degrees_to_cardinal(bearing);

    let aircraft_id = aircraft
        .callsign
        .as_ref()
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .unwrap_or_else(|| aircraft.aircraft_type.as_deref().unwrap_or("Aircraft"));

    let title = format!("{} spotted", aircraft_id);

    let altitude = aircraft.altitude_barometric.unwrap_or(0);
    let distance_miles = aircraft.distance_km * 0.621371;

    let message = format!(
        "Look {} • {}ft • {:.1} mi away",
        direction, altitude, distance_miles,
    );
    send_notification(&config, &title, &message).await?;
    Ok(())
}
#[tokio::main]

async fn main() {
    dotenv::dotenv().expect("Failed to load env");
    let check_thread = tokio::spawn(async {
        let config = load_config();
        loop {
            let location = get_origin_location(&config)
                .await
                .unwrap_or_else(|_| config.static_location.clone());
            let planes = check_for_planes(&config, &location).await;
            let Ok(planes) = planes else {
                println!("Failed to check for planes");
                continue;
            };
            for plane in planes {
                send_notification_for_interesting_plane(&config, &location, &plane)
                    .await
                    .expect("Failed to send notification");
            }
            tokio::time::sleep(Duration::from_mins(config.update_interval_min)).await;
        }
    });
    let _ = check_thread.await;
}
