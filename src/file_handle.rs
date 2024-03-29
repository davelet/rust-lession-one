use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
pub async fn redis() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("clientId", HeaderValue::from_static("6090ae45-e3c3-4547-bc2e-51a8a8fa057a"));
    let url = "https://pep-service-internal.latest.shdrapps.disney.com/unified-list-service/redis/inspect?key=inv_attZootopiaHotPursuit_ts_20240323&field=";
    for id in 20356 ..= 20402 {
        let u = url.to_owned() + &id.to_string();
        let response = client.get(u).headers(headers.clone()).send().await?;

        println!("id = {}", id);
        let status = response.status();
        let text = response.text().await?;
        if status == 200 {
            println!("{}", text);
        } else {
        }
        println!("{}", text);
    }

    Ok(())
}