use anyhow::Result;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

pub fn get_location(ip: IpAddr) -> Result<Coordinates> {
    let body: Coordinates =
        reqwest::blocking::get(format!("http://ip-api.com/json/{}", &ip))?.json()?;
    Ok(body)
}
