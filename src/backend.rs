use dioxus::{ prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: Results,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub uv: f32,
    #[serde(rename = "uv_time")]
    pub uv_time: String,
    #[serde(rename = "uv_max")]
    pub uv_max: f64,
    #[serde(rename = "uv_max_time")]
    pub uv_max_time: String,
    pub ozone: f64,
    // #[serde(rename = "ozone_time")]
    // pub ozone_time: String,
    // #[serde(rename = "safe_exposure_time")]
    // pub safe_exposure_time: SafeExposureTime,
    // #[serde(rename = "sun_info")]
    // pub sun_info: SunInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeExposureTime {
    pub st1: Value,
    pub st2: Value,
    pub st3: Value,
    pub st4: Value,
    pub st5: Value,
    pub st6: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunInfo {
    #[serde(rename = "sun_times")]
    pub sun_times: SunTimes,
    #[serde(rename = "sun_position")]
    pub sun_position: SunPosition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunTimes {
    pub solar_noon: String,
    pub nadir: String,
    pub sunrise: String,
    pub sunset: String,
    pub sunrise_end: String,
    pub sunset_start: String,
    pub dawn: String,
    pub dusk: String,
    pub nautical_dawn: String,
    pub nautical_dusk: String,
    pub night_end: String,
    pub night: String,
    pub golden_hour_end: String,
    pub golden_hour: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunPosition {
    pub azimuth: f64,
    pub altitude: f64,
}

#[cfg(feature = "server")]
use lazy_static::lazy_static;
#[cfg(feature = "server")]
lazy_static! {
    static ref TOKEN: String = String::from_utf8(std::fs::read("token.txt").unwrap()).unwrap();
}

#[server]
pub async fn weather_get() -> Result<Root, ServerFnError> {
    let url = format!("https://api.openuv.io/api/v1/uv?lat=-32.056946&lng=115.743889");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("x-access-token", TOKEN.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    Ok(response)
}
