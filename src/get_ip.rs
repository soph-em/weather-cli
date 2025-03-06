use anyhow::Result;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
struct IpResponse {
    ip: String,
}

pub fn get_ip() -> Result<IpAddr> {
    let response: IpResponse =
        reqwest::blocking::get("https://api.ipify.org?format=json")?.json()?;
    let ip_addr: IpAddr = response.ip.parse()?;
    Ok(ip_addr)
}
