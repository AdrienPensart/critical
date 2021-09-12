use anyhow::Result;
use clap::{AppSettings, Clap};
use postgres::{Client, NoTls};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Database options")]
pub struct Database {
    /// Database host
    #[clap(long, default_value = "localhost")]
    pub host: String,

    /// Database port
    #[clap(long, default_value = "5432")]
    pub port: u16,

    /// Database user
    #[clap(short, long, default_value = "musicbot")]
    pub user: String,

    /// Database password
    #[clap(short, long, default_value = "musicbot")]
    pub password: String,

    /// Database name
    #[clap(short, long, default_value = "musicbot")]
    pub name: String,
}

impl Database {

    pub fn global_client(&self) -> Result<Client> {
        let dsn = format!("host={} port={} user={} password={}", self.host, self.port, self.user, self.password);
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn db_client(&self) -> Result<Client> {
        let dsn = format!("host={} port={} user={} password={} dbname={}", self.host, self.port, self.user, self.password, self.name);
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn create_role_and_db(&self, client: &mut Client) -> Result <()> {
        let query = r#"
do
$do$
begin
    if not exists (select from pg_catalog.pg_roles where rolname = '{user}') then
        create role {user} login password '{password}';
    end if;

    if not exists (select from pg_catalog.pg_database where datname = '{name}' then
        create database {name} with owner {user};
    end if;
end
$do$
        "#;
        client.batch_execute(query)?;
        Ok(())
    }

    pub fn create_schemas(&self, client: &mut Client) -> Result<()> {
        let schemas = include_str!("schema/schemas.sql");
        client.batch_execute(schemas)?;

        let extensions = include_str!("schema/extensions.sql");
        client.batch_execute(extensions)?;

        let user = include_str!("schema/user.sql");
        client.batch_execute(user)?;

        let music = include_str!("schema/music.sql");
        client.batch_execute(music)?;

        let filter = include_str!("schema/filter.sql");
        client.batch_execute(filter)?;

        let views = include_str!("schema/views.sql");
        client.batch_execute(views)?;

        let playlist = include_str!("schema/playlist.sql");
        client.batch_execute(playlist)?;

        let stat = include_str!("schema/stat.sql");
        client.batch_execute(stat)?;

        let grants = include_str!("schema/grants.sql");
        client.batch_execute(grants)?;

        Ok(())
    }

    pub fn kill_connections(&self, client: &mut Client) -> Result<()> {
        let query = format!(r#"
            select pg_terminate_backend(pg_stat_activity.pid)
            from pg_stat_activity
            where pg_stat_activity.datname = '{}' and pid <> pg_backend_pid()
        "#, &self.name);
        client.batch_execute(&query)?;
        Ok(())
    }

    pub fn drop_functions(&self, client: &mut Client) -> Result<()> {
        let query = include_str!("schema/drop_functions.sql");
        client.batch_execute(query)?;
        Ok(())
    }

    pub fn drop_db(&self, client: &mut Client) -> Result<()> {
        let query = format!("drop database if exists {}", self.name);
        client.batch_execute(&query)?;
        Ok(())
    }

    pub fn drop_schemas(&self, client: &mut Client) -> Result<()> {
        let query = include_str!("schema/drop_schemas.sql");
        client.batch_execute(query)?;
        Ok(())
    }

    pub fn clear_schemas(&self, client: &mut Client) -> Result<()> {
        self.drop_schemas(client)?;
        self.create_schemas(client)?;
        Ok(())
    }
}
