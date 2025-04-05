use super::errors::CriticalErrorKind;

pub struct Config {
    pub dsn: String,
    pub gel: gel_tokio::Client,
    pub dry: bool,
    pub no_gel: bool,
    pub retries: u16,
}

impl Config {
    pub fn new(dsn: String, dry: bool, no_gel: bool) -> Result<Self, CriticalErrorKind> {
        let config = gel_tokio::Builder::new()
            .dsn(&dsn)
            // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
            .build()?;
        let gel = gel_tokio::Client::new(&config);
        Ok(Self {
            dry,
            dsn,
            gel,
            no_gel,
            retries: 0,
        })
    }
}
