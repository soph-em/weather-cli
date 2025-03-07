use crate::{get_weather::Weather, get_weather_type_from_code::get_weather_description};

pub fn display_weather_now(weather: Weather){
    println!("The current temperature is {}°c", weather.current.temperature_2m);
    println!("It's currently: {}", get_weather_description(weather.current.weather_code))
}