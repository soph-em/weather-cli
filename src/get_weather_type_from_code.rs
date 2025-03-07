use std::collections::HashMap;

pub fn get_weather_description(code: i8) -> &'static str {
    let weather_map: HashMap<i8, &str> = [
        (-1, "Trace rain"),
        (0, "Clear night 🌌"),
        (1, "Sunny day ☀️"),
        (2, "Partly cloudy 🌤️💤"),
        (3, "Partly cloudy 🌤️"),
        (5, "Mist 🌫️"),
        (6, "Fog 🌁"),
        (7, "Cloudy ☁️"),
        (8, "Overcast 🌥️"),
        (9, "Light rain shower 🌦️💤"),
        (10, "Light rain shower 🌦️"),
        (11, "Drizzle ☔️"),
        (12, "Light rain ☔️☔️"),
        (13, "Heavy rain shower ☔️☔️☔️💤"),
        (14, "Heavy rain shower ☔️☔️☔️"),
        (15, "Heavy rain ☔️☔️☔️"),
        (16, "Sleet shower ❄️☔️💤"),
        (17, "Sleet shower ❄️☔️"),
        (18, "Sleet ❄️☔️"),
        (19, "Hail shower 🧊💤"),
        (20, "Hail shower 🧊"),
        (21, "Hail 🧊"),
        (22, "Light snow shower 🌨️💤"),
        (23, "Light snow shower 🌨️"),
        (24, "Light snow 🌨️"),
        (25, "Heavy snow shower 🌨️🌨️☃️💤"),
        (26, "Heavy snow shower 🌨️🌨️☃️"),
        (27, "Heavy snow 🌨️🌨️☃️"),
        (28, "Thunder shower 🌩️⚡️💤"),
        (29, "Thunder shower 🌩️⚡️"),
        (30, "Thunder 🌩️⚡️"),
    ]
    .iter()
    .copied()
    .collect();

    weather_map.get(&code).unwrap_or(&"Unknown weather code")
}

// fn main() {
//     let code: i8 = 10; // Example API response
//     println!("Weather: {}", get_weather_description(code));
// }
