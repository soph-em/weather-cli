use anyhow::{ Ok, Result };
use serde::Deserialize;

use crate::get_location::Coordinates;

#[derive(Deserialize)]
pub struct Weather {
    pub current: Current,
    pub hourly: Hourly,
}

#[derive(Deserialize)]
pub struct Current {
    pub temperature_2m: f64,
    pub weather_code: i8,
    pub wind_speed_10m: f64,
}

#[derive(Deserialize)]
pub struct Hourly {
    pub temperature_2m: Vec<f64>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<i8>,
    pub cloud_cover: Vec<u8>,
}

pub fn get_weather(coordinates: Coordinates) -> Result<Weather> {
    let response: Weather = reqwest::blocking
        ::get(
            format!(
                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code,wind_speed_10m&hourly=temperature_2m,precipitation,weather_code,cloud_cover&daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_probability_max&forecast_days=1&models=ukmo_seamless",
                "53.4809",
                "-2.2374"
            )
        )?
        .json()?;
    Ok(response)
}
