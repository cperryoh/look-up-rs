mod errors;
mod notification_senders;
mod util;

use errors::Result;
use std::{collections::HashMap, env, sync::LazyLock, time::{Duration, Instant}};

use serde_json::Value;
const API: &str = "https://api.adsb.lol/v2";
use crate::{
    notification_senders::send_notification,
    util::{
        Aircraft, Config, Location, bearing_to_target, degrees_to_cardinal, get_origin_location,
    },
};

static HASSIO_API_KEY: LazyLock<String> = LazyLock::new(|| env::var("HASSIO_API_KEY").unwrap());
static HASSIO_URL: LazyLock<String> = LazyLock::new(|| env::var("HASSIO_SERVER_URL").unwrap());
static NTFY_API_KEY: LazyLock<String> = LazyLock::new(|| env::var("NTFY_API_KEY").unwrap());
static NTFY_URL: LazyLock<String> = LazyLock::new(|| env::var("NTFY_SERVER_URL").unwrap());

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
        .iter()
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
    if let Some(atype) = &aircraft.aircraft_type
        && military_types.iter().any(|t| atype.contains(t)) {
            return true;
        }
    let alt = aircraft
        .altitude_barometric
        .as_ref()
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    if alt >= config.min_height
    {
        return true;
    }

    false
}
fn load_config() -> Config {
    let cfg_str = std::fs::read_to_string("./config.toml").unwrap();
    toml::from_str(&cfg_str).unwrap()
}
async fn send_notification_for_interesting_plane(
    _config: &Config,
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

    let title = format!(
        "{} spotted",
        aircraft
            .aircraft_type
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Unknown".into())
    );

    let alt = aircraft
        .altitude_barometric
        .as_ref()
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let distance_miles = aircraft.distance_km * 0.621371;

    let message = format!(
        "Look {} • {}ft • {:.1} mi away",
        direction, alt, distance_miles,
    );
    send_notification(
        &title,
        &message,
        Some(format!("https://adsb.lol/?icao={}", aircraft.icao_address)),
    )
    .await?;
    Ok(())
}
async fn check_for_planes(config: &Config, location: &Location) -> Result<Vec<Aircraft>> {
    let planes = get_data(config, location).await?;
    Ok(planes
        .into_iter()
        .filter(|a| is_interesting(config, a))
        .collect())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    if let Err(e) = send_notification("Starting", "Look up rs is watching the skies", None).await {
        eprintln!("Failed to send startup notification: {e}");
    }

    let config = load_config();
    let mut seen_cache: HashMap<String, Instant> = HashMap::new();

    loop {
        let location = get_origin_location(&config)
            .await
            .unwrap_or_else(|_| config.static_location.clone());

        match check_for_planes(&config, &location).await {
            Ok(planes) => {
                // Prune old aircraft cache entries (> 1 hour)
                seen_cache.retain(|_, time| time.elapsed() < Duration::from_secs(3600));

                for plane in planes {
                    if seen_cache.contains_key(&plane.icao_address) {
                        continue;
                    }

                    if let Err(e) = send_notification_for_interesting_plane(&config, &location, &plane).await {
                        eprintln!("Failed notification for {}: {e}", plane.icao_address);
                    } else {
                        seen_cache.insert(plane.icao_address.clone(), Instant::now());
                    }
                }
            }
            Err(e) => eprintln!("Failed checking airspace: {e}"),
        }

        tokio::time::sleep(Duration::from_secs(config.update_interval_min * 60)).await;
    }
}
