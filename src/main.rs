//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use std::{fmt::Display, process::exit};

use clap::Parser;
use cli::{Cli, Commands};
use favorite_city::{get_favorite_city, set_favorite_city};
use reqwest::{Client, Response, StatusCode};
use weather_types::{WeatherConditions, WeatherData};

mod cli;
mod favorite_city;
mod weather_types;

const CITIES: [&str; 10] = [
    "bruxelles-1",
    "aalbeke",
    "gent",
    "beaumont",
    "antwerpen-1",
    "brugge",
    "grivegnee-liege",
    "hasselt",
    "mons",
    "namur",
];

#[tokio::main]
async fn main() -> () {
    let client: Client = Client::new();

    let parsed = Cli::parse();
    match parsed.command {
        Commands::Get { city, day } => match city {
            cli::CityNameOrFavorite::CityName(city) => {
                print_daily_weather_forecast(&client, &city, day.into())
                    .await
                    .unwrap_or_else(|x| println!("Forecast API error : {}", x));
                exit(-1);
            }
            cli::CityNameOrFavorite::Favorite => {
                print_daily_weather_forecast(&client, get_favorite_city().as_str(), day.into())
                    .await
                    .unwrap_or_else(|x| println!("Forecast API error : {}", x));
                exit(-1);
            }
        },
        Commands::Set { city_name } => {
            set_favorite_city(city_name);
        }
    };
}

enum ForecastApiError {
    CityNotFound(String),
    ForecastDayOutOfRange,
}

impl Display for ForecastApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ForecastApiError::CityNotFound(city) => {
                write!(f, "City {} not found when calling the forecast API", city)
            }
            ForecastApiError::ForecastDayOutOfRange => {
                write!(f, "Day not supported by the forecast API")
            }
        }
    }
}

/// Prints the daily forecast of a city
async fn print_daily_weather_forecast(
    client: &Client,
    city: &str,
    day: usize,
) -> Result<(), ForecastApiError> {
    let url = format!("https://www.prevision-meteo.ch/services/json/{}", city);
    let response: Response = client.get(&url).send().await.unwrap();
    assert!(
        &response.status() == &StatusCode::OK,
        "Response from weather API was not OK, was {}",
        &response.status()
    );
    if day > 5 {
        return Err(ForecastApiError::ForecastDayOutOfRange);
    }

    let weather_data: WeatherData = response
        .json::<WeatherData>()
        .await
        .map_err(|_err| ForecastApiError::CityNotFound(city.into()))?;

    let forecasts = [
        weather_data.fcst_day_0,
        weather_data.fcst_day_1,
        weather_data.fcst_day_2,
        weather_data.fcst_day_3,
        weather_data.fcst_day_4,
    ];

    let forecast = forecasts.get(day).expect("Forecast day out of range 0..5");

    println!(
        "Weather in {} will be {} {}  in {} days",
        weather_data.city_info.name,
        forecast.condition,
        WeatherConditions::from(forecast.condition_key.as_str()),
        day
    );
    Ok(())
}
