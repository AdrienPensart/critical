use anyhow::Result;
use clap::{AppSettings, Clap};
use postgres::{Client, NoTls};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Database options")]
pub struct Database {
    /// Database admin user
    #[clap(long, default_value = "postgres")]
    pub admin_user: String,

    /// Database admin password
    #[clap(short, long, default_value = "musicbot")]
    pub admin_password: String,

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
    pub fn admin_client(&self) -> Result<Client> {
        let dsn = format!(
            "postgresql://{admin_user}:{admin_password}@{host}:{port}/{name}",
            host=self.host,
            port=self.port,
            name=self.name,
            admin_user=self.admin_user,
            admin_password=self.admin_password,
        );
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn admin_no_db_client(&self) -> Result<Client> {
        let dsn = format!(
            "postgresql://{admin_user}:{admin_password}@{host}:{port}/",
            host=self.host,
            port=self.port,
            admin_user=self.admin_user,
            admin_password=self.admin_password,
        );
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn no_db_client(&self) -> Result<Client> {
        let dsn = format!("postgresql://{user}:{password}@{host}:{port}", host=self.host, port=self.port, user=self.user, password=self.password);
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn client(&self) -> Result<Client> {
        let dsn = format!("postgresql://{user}:{password}@{host}:{port}/{name}", host=self.host, port=self.port, user=self.user, password=self.password, name=self.name);
        Ok(Client::connect(&dsn, NoTls)?)
    }

    pub fn kill_connections(&self, client: &mut Client) -> Result<()> {
        let query = format!(r#"
            select pg_terminate_backend(pg_stat_activity.pid)
            from pg_stat_activity
            where pg_stat_activity.datname = '{name}' and pid <> pg_backend_pid()
            "#,
            name=&self.name,
        );
        client.batch_execute(&query)?;
        Ok(())
    }

    pub fn fill_schemas(&self, client: &mut Client) -> Result<()> {
        let aggregates = include_str!("schema/user/aggregates.sql");
        client.batch_execute(aggregates)?;

        let user = include_str!("schema/user/user.sql");
        client.batch_execute(user)?;

        let music = include_str!("schema/user/music.sql");
        client.batch_execute(music)?;

        let filter = include_str!("schema/user/filter.sql");
        client.batch_execute(filter)?;

        let views = include_str!("schema/user/views.sql");
        client.batch_execute(views)?;

        let playlist = include_str!("schema/user/playlist.sql");
        client.batch_execute(playlist)?;

        let stat = include_str!("schema/user/stat.sql");
        client.batch_execute(stat)?;

        let grants = include_str!("schema/user/grants.sql");
        client.batch_execute(grants)?;

        Ok(())
    }

    pub fn drop_functions(&self, client: &mut Client) -> Result<()> {
        let query = include_str!("schema/user/drop_functions.sql");
        client.batch_execute(query)?;
        Ok(())
    }

    pub fn drop_schemas(&self, client: &mut Client) -> Result<()> {
        let query = include_str!("schema/user/drop_schemas.sql");
        client.batch_execute(query)?;
        Ok(())
    }

    pub fn create_role(&self, client: &mut Client) -> Result<()> {
        let create_role_query = format!(
            include_str!("schema/admin/create_role.sql"),
            user=self.user,
            password=self.password,
        );
        client.batch_execute(&create_role_query)?;
        Ok(())
    }

    pub fn create_schemas(&self, client: &mut Client) -> Result<()> {
        let create_schemas_query = format!(
            include_str!("schema/admin/create_schemas.sql"),
            user=self.user,
        );
        client.batch_execute(&create_schemas_query)?;
        Ok(())
    }

    pub fn create_extensions(&self, client: &mut Client) -> Result<()> {
        let create_extensions_query = include_str!("schema/admin/create_extensions.sql");
        client.batch_execute(&create_extensions_query)?;
        Ok(())
    }

    pub fn drop_extensions(&self, client: &mut Client) -> Result<()> {
        let drop_extensions_query = include_str!("schema/admin/drop_extensions.sql");
        client.batch_execute(&drop_extensions_query)?;
        Ok(())
    }

    pub fn create_database(&self, client: &mut Client) -> Result<()> {
        let create_database_query = format!(
            include_str!("schema/admin/create_database.sql"),
            host=self.host,
            name=self.name,
            user=self.user,
            admin_user=self.admin_user,
            admin_password=self.admin_password,
        );
        client.batch_execute(&create_database_query)?;
        Ok(())
    }

    pub fn drop_database(&self, client: &mut Client) -> Result<()> {
        let drop_database_query = format!(
            include_str!("schema/admin/drop_database.sql"),
            host=self.host,
            name=self.name,
            admin_user=self.admin_user,
            admin_password=self.admin_password,
        );
        client.batch_execute(&drop_database_query)?;
        Ok(())
    }

    pub fn drop_role(&self, client: &mut Client) -> Result<()> {
        let drop_role_query = format!(
            include_str!("schema/admin/drop_role.sql"),
            host=self.host,
            user=self.user,
            admin_user=self.admin_user,
            admin_password=self.admin_password,
        );
        client.batch_execute(&drop_role_query)?;
        Ok(())
    }
}
