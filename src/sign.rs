use crate::errors::{ESDumpResult};
use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
use std::time::SystemTime;
use crate::es_dump::ESDump;

pub fn get_signed_request(
    es_dump: &ESDump,
    url: String
) -> ESDumpResult<http::Request<String>> {
    let mut request = http::Request::builder()
        .uri(url)
        .body(es_dump.query.clone())?;
    request
        .headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());
    // Set up information and settings for the signing
    let signing_settings = SigningSettings::default();
    let signing_params = SigningParams::builder()
        .access_key(es_dump.key_id.as_str())
        .secret_key(es_dump.secret_key.as_str())
        .region(es_dump.region.as_str())
        .service_name("es")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()?;
    // Convert the HTTP request into a signable request
    let signable_request = SignableRequest::from(&request);
    // Sign and then apply the signature to the request
    let (signing_instructions, _signature) = sign(signable_request, &signing_params)?.into_parts();
    signing_instructions.apply_to_request(&mut request);
    Ok(request)
}
