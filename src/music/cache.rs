use std::collections::HashMap;

#[derive(Default)]
pub struct UpsertCache {
    pub folders: HashMap<String, uuid::Uuid>,
    pub artists: HashMap<String, uuid::Uuid>,
    pub albums: HashMap<uuid::Uuid, HashMap<String, uuid::Uuid>>,
    pub genres: HashMap<String, uuid::Uuid>,
    pub keywords: HashMap<String, uuid::Uuid>,
    pub errors: u64,
}
