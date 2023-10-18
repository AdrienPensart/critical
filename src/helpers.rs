use edgedb_protocol::value::Value as EValue;
use edgedb_protocol::codec::{ShapeElement, ObjectShape};
use edgedb_protocol::common::Cardinality;
use walkdir::DirEntry;

use crate::errors::CriticalErrorKind;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.'))
         .unwrap_or(false)
}

pub async fn public_ip() -> Result<String, CriticalErrorKind> {
    let client = reqwest::Client::new();
    let response = client.head("https://www.wikipedia.org")
        .send()
        .await?;
    let header = "X-Client-IP";
    if let Some(ip) = response.headers().get(header) {
        match ip.to_str() {
            Ok(ip) => Ok(ip.to_string()),
            Err(e) => Err(CriticalErrorKind::HeaderError(e))
        }
    } else {
        Err(CriticalErrorKind::NoPublicIp)
    }
}

pub fn vec_option_to_vec(v: Vec<Option<String>>) -> Vec<String> {
    v
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
}

pub fn edge_object_from_pairs<N, V>(iter: impl IntoIterator<Item = (N, (V, Cardinality))>) -> EValue
where
    N: ToString,
    V: Into<Option<EValue>>,
{
    let mut elements = Vec::new();
    let mut fields: Vec<Option<EValue>> = Vec::new();
    for (key, (val, cardinality)) in iter.into_iter() {
        elements.push(create_shape_element(key, cardinality));
        fields.push(val.into());
    }
    EValue::Object {
        shape: ObjectShape::new(elements),
        fields,
    }
}

pub fn create_shape_element<N: ToString>(name: N, cardinality: Cardinality) -> ShapeElement {
    ShapeElement {
        name: name.to_string(),
        cardinality: Some(cardinality),
        flag_link: false,
        flag_link_property: false,
        flag_implicit: false,
    }
}
