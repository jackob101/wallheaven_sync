use std::{thread::sleep, time::Duration};

use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder, Response},
    header::{HeaderName, RETRY_AFTER},
    StatusCode, Url,
};

use crate::prompts;

pub fn download_image(url: Url) -> Result<Vec<u8>, String> {
    download_image_with_client(&create_client(), url)
}

fn download_image_with_client(client: &Client, url: Url) -> Result<Vec<u8>, String> {
    let request = client
        .get(url)
        .header(HeaderName::from_static("accept"), "image/*");

    let response = repeating(&client, request);

    match response.status() {
        StatusCode::OK => Ok(response.bytes().unwrap().to_vec()),
        _ => {
            return Err(format!(
                "Unhandled response code received in client, {}",
                response.status()
            ))
        }
    }
}

fn create_client() -> Client {
    ClientBuilder::new()
        .user_agent("wallheaven_sync/pietrzyk.jakub001@gmail.com")
        .build()
        .unwrap()
}

fn repeating(client: &Client, request: RequestBuilder) -> Response {
    let request = request.build().expect("failed to build request");

    loop {
        let response = match client.execute(request.try_clone().unwrap()) {
            Ok(value) => value,
            Err(_) => {
                panic!("Failed to get response")
            }
        };

        match response.headers().get(RETRY_AFTER) {
            Some(value) => {
                let retry_after = value.to_str().unwrap();
                prompts::info(&format!(
                    "Reached request per minute limit, waiting {} seconds...",
                    retry_after
                ));
                sleep(Duration::from_secs_f64(
                    retry_after
                        .parse::<f64>()
                        .expect("retry-after header is not numeric value"),
                ));
            }
            None => (),
        };

        return response;
    }
}

