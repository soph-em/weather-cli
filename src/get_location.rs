use anyhow::Result;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
pub struct Coordinates {
    pub lon: f64,
    pub lat: f64,
}

pub fn get_location(ip: IpAddr) -> Result<Coordinates> {
    let body: Coordinates = reqwest::blocking
        ::get(format!("http://ip-api.com/json/{}?fields=192", ip))?
        .json()?;
    Ok(body)
}
