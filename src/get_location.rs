use anyhow::Result;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
struct Coordinates {
    longitude: f64,
    latitude: f64,
}

fn get_location(ip: IpAddr) -> Result<Coordinates> {
    let body: Coordinates =
        reqwest::blocking::get(format!("http://ip-api.com/json/{}", &ip))?.json()?;
    Ok(body)
}
