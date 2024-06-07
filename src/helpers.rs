use walkdir::DirEntry;

use crate::errors::CriticalErrorKind;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub async fn public_ip() -> Result<String, CriticalErrorKind> {
    let client = reqwest::Client::new();
    let response = client.head("https://www.wikipedia.org").send().await?;
    let header = "X-Client-IP";
    if let Some(ip) = response.headers().get(header) {
        match ip.to_str() {
            Ok(ip) => Ok(ip.to_string()),
            Err(e) => Err(CriticalErrorKind::HeaderError(e)),
        }
    } else {
        Err(CriticalErrorKind::NoPublicIp)
    }
}

pub fn vec_option_to_vec(v: Vec<Option<String>>) -> Vec<String> {
    v.into_iter().flatten().collect::<Vec<_>>()
}
