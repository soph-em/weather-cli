use anyhow::Result;
use get_ip::get_ip;
use get_location::{ Coordinates, get_location };
use get_weather::{Weather, get_weather};
use weather_now::display_weather_now;
// use std::net::IpAddr;

// use clap::Parser;
mod get_ip;
mod get_location;
mod get_weather;
mod weather_now;
mod get_weather_type_from_code;

/// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
    /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
// }

fn main() -> Result<()> {
    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
    let ip = get_ip()?;
    // println!("{}", ip);

    let location: Coordinates = get_location(ip).expect("reason");
    // println!("{}, {}", location.lat, location.lon);

    let weather: Weather = get_weather(location).expect("reason");
    // println!("{}", weather.current.temperature_2m);
    // println!("{}", weather.current.weather_code);

    display_weather_now(weather);
    Ok(())
}
