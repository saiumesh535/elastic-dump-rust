use log::info;

use crate::errors;

pub async fn api(request: http::Request<String>) -> errors::ESDumpResult<String> {
    let client = reqwest::Client::new();
    let res = client
        .request(request.method().clone(), &request.uri().to_string())
        .headers(request.headers().clone())
        .body(request.body().clone())
        .send()
        .await?;

    let status = res.status();
    let response = res.text().await?;
    if status != http::StatusCode::OK {
        info!("Request failed with code {}", status);
        info!("Response from ES \n {}", response);
        return Err(errors::ESDumpError::CustomError(response));
    }
    Ok(response)
}
