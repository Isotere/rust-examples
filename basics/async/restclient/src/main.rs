use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.9517&longitude=-92.3341&current_weather=true";

    let response = reqwest::get(URL).await?;

    // Full Response
    // let weather: serde_json::Value = response.json().await?;

    // Typed Response
    let weather: Weather = response.json().await?;

    println!("{:#?}", weather);

    Ok(())
}
