use get_ip::get_ip;
use get_location::{Coordinates, get_location};
use std::net::IpAddr;

use clap::Parser;
mod get_ip;
mod get_location;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
    let ip: Result<IpAddr, anyhow::Error> = get_ip();
    println!("{:?}", ip);

    // let location: Coordinates = get_location(ip).expect("reason");
    // println!("{}, {}", location.latitude, location.longitude)
}
