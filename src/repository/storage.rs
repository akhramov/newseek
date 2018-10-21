use domain::{
    Settings,
};

use errors::*;

use diesel::{
    self,
    Connection,
};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{ Pool, PooledConnection };

use dotenv::dotenv;
use std::env;

pub trait Store {
    fn create_settings(&self) -> Result<Settings>;
    fn get_settings(&self) -> Result<Settings>;
    fn update_settings(&self, settings : &Settings) -> Result<Settings>;
}

pub struct PgStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgStore {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15) // TODO: configurable by env
            .build(manager)
            .expect("Failed to create pool.");

        Self { pool: pool }
    }
}

impl Store for PgStore {
    fn create_settings(&self) -> Result<Settings> {
        use schema::settings;

        let ref connection = *self.pool.get().context(ErrorKind::DbPoolError)?;

        let result = diesel::insert_into(settings::table)
            .values(Settings::default())
            .get_result(connection)
            .with_context(|e| {
                ErrorKind::DbError(format!("Creating settings error: {:?}", e))
            })?;


        Ok(result)
    }

    fn get_settings(&self) -> Result<Settings> {
        use schema::settings::dsl::*;

        let ref connection = *self.pool.get().context(ErrorKind::DbPoolError)?;

        let existing_settings = settings.limit(1).first::<Settings>(connection);

        let result = match existing_settings {
            Ok(old_settings) => old_settings,
            Err(_) => self.create_settings()?,
        };

        Ok(result)
    }

    fn update_settings(&self, settings : &Settings) -> Result<Settings> {
        use schema::settings;

        let ref connection = *self.pool.get().context(ErrorKind::DbPoolError)?;

        let old_settings = self.get_settings()?;


        Ok(
            diesel::update(&old_settings)
                .set(settings).get_result::<Settings>(connection)
                .with_context(|e| {
                    ErrorKind::DbError(format!("Updating settings error: {:?}", e))
                })?
        )
    }
}
