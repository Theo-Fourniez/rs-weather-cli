use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CityInfo {
    pub name: String,
    country: String,
    latitude: String,
    longitude: String,
    elevation: String,
    sunrise: String,
    sunset: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ForecastInfo {
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CurrentCondition {
    date: String,
    hour: String,
    tmp: i32,
    wnd_spd: i32,
    wnd_gust: i32,
    wnd_dir: String,
    pressure: f64,
    humidity: i32,
    condition: String,
    condition_key: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FcstDay {
    date: String,
    day_short: String,
    day_long: String,
    tmin: i32,
    tmax: i32,
    pub condition: String,
    pub condition_key: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct WeatherData {
    pub city_info: CityInfo,
    pub forecast_info: ForecastInfo,
    pub current_condition: CurrentCondition,
    pub fcst_day_0: FcstDay,
    pub fcst_day_1: FcstDay,
    pub fcst_day_2: FcstDay,
    pub fcst_day_3: FcstDay,
    pub fcst_day_4: FcstDay,
}

pub(crate) enum WeatherConditions {
    Eclaircies,
    Ensoleille,
    NuitClaire,
    NuitNuageuse,
    FortementNuageux,
    FaiblementNuageux,
    FaiblesPassagesNuageux,
    NuitBienDegagee,
    CielVoile,
    DeveloppementNuageux,
    NuitAvecDeveloppementNuageux,
    NuitAvecAverses,
    AversesDePluieFaible,
    OrageModere,
    PluieForte,
    NuitLegerementVoilee,
    Autre,
}

impl Display for WeatherConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            WeatherConditions::Eclaircies => write!(f, "⛅"),
            WeatherConditions::Ensoleille => write!(f, "☀️"),
            WeatherConditions::NuitClaire => write!(f, "🌙"),
            WeatherConditions::NuitNuageuse => write!(f, "☁️🌙"),
            WeatherConditions::FortementNuageux => write!(f, "☁️☁️"),
            WeatherConditions::FaiblementNuageux => write!(f, "🌥️"),
            WeatherConditions::FaiblesPassagesNuageux => write!(f, "🌤️"),
            WeatherConditions::NuitBienDegagee => write!(f, "🌃"),
            WeatherConditions::CielVoile => write!(f, "🌫️"),
            WeatherConditions::DeveloppementNuageux => write!(f, "☁️"),
            WeatherConditions::NuitAvecDeveloppementNuageux => write!(f, "☁️🌙"),
            WeatherConditions::NuitAvecAverses => write!(f, "🌧️🌙"),
            WeatherConditions::AversesDePluieFaible => write!(f, "🌦️"),
            WeatherConditions::OrageModere => write!(f, "⛈️"),
            WeatherConditions::PluieForte => write!(f, "🌧️"),
            WeatherConditions::NuitLegerementVoilee => write!(f, "🌫️🌙"),
            WeatherConditions::Autre => write!(f, "❓"),
        }
    }
}

impl From<&str> for WeatherConditions {
    fn from(value: &str) -> Self {
        match value {
            "eclaircies" => Self::Eclaircies,
            "ensoleille" => Self::Ensoleille,
            "nuit-claire" => Self::NuitClaire,
            "nuit-nuageuse" => Self::NuitNuageuse,
            "fortement-nuageux" => Self::FortementNuageux,
            "faiblement-nuageux" => Self::FaiblementNuageux,
            "faibles-passages-nuageux" => Self::FaiblesPassagesNuageux,
            "nuit-bien-degagee" => Self::NuitBienDegagee,
            "ciel-voile" => Self::CielVoile,
            "developpement-nuageux" => Self::DeveloppementNuageux,
            "nuit-avec-developpement-nuageux" => Self::NuitAvecDeveloppementNuageux,
            "nuit-avec-averses" => Self::NuitAvecAverses,
            "averses-de-pluie-faible" => Self::AversesDePluieFaible,
            "orage-modere" => Self::OrageModere,
            "pluie-forte" => Self::PluieForte,
            "nuit-legerement-voilee" => Self::NuitLegerementVoilee,
            _ => Self::Autre,
        }
    }
}
