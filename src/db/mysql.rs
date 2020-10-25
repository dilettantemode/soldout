use dotenv::dotenv;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
lazy_static! {
    pub static ref POOL: Pool = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}
//
