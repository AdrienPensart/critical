use crate::fingerprinting::signature_format::DecodedSignature;
use crate::fingerprinting::user_agent::USER_AGENTS;
use crate::music::errors::CriticalErrorKind;
use rand::seq::IndexedRandom;
use reqwest::header::HeaderMap;
use serde_json::{Value, json};
use std::time::Duration;
use std::time::SystemTime;
use uuid::Uuid;

#[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
pub async fn recognize_song_from_signature(
    signature: &DecodedSignature,
) -> Result<Value, CriticalErrorKind> {
    let timestamp_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();

    let post_data = json!({
        "geolocation": {
            "altitude": 300,
            "latitude": 45,
            "longitude": 2
        },
        "signature": {
            "samplems": u32::try_from((f64::from(signature.number_samples) / f64::from(signature.sample_rate_hz) * 1000.) as i64)?,
            "timestamp": (timestamp_ms % u128::from(u32::MAX)) as u32,
            "uri": signature.encode_to_uri()?
        },
        "timestamp": timestamp_ms as u32,
        "timezone": "Europe/Paris"
    });

    let uuid_1 = Uuid::new_v4().hyphenated().to_string().to_uppercase();
    let uuid_2 = Uuid::new_v4().hyphenated().to_string();
    let url = format!("https://amp.shazam.com/discovery/v5/en/US/android/-/tag/{uuid_1}/{uuid_2}");

    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        USER_AGENTS.choose(&mut rand::rng()).unwrap().parse()?,
    );
    headers.insert("Content-Language", "en_US".parse()?);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&post_data)
        .timeout(Duration::from_secs(20))
        .query(&[
            ("sync", "true"),
            ("webv3", "true"),
            ("sampling", "true"),
            ("connected", ""),
            ("shazamapiversion", "v3"),
            ("sharehub", "true"),
            ("video", "v3"),
        ])
        .headers(headers)
        .send()
        .await?;

    Ok(response.json().await?)
}
