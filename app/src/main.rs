use actix_web::*;
use diesel_async::{
    pooled_connection::deadpool::Pool,
    pooled_connection::AsyncDieselConnectionManager as ConnectionManager, AsyncPgConnection as DB,
};
use dotenv::dotenv;

mod helpers;
mod routes;

pub type DbPool = Pool<DB>;
pub type DbConn = deadpool::managed::Object<ConnectionManager<DB>>;
pub type Result<T> = std::result::Result<T, crate::helpers::errors::Error>;

pub fn cfg(cfg: &mut web::ServiceConfig) {
    let pool = helpers::db::establish_connection();
    cfg.service(
        web::scope("")
            .app_data(web::Data::new(pool))
            .service(routes::index::get),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new().configure(cfg))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
