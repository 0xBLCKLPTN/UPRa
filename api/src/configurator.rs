use crate::router_creator::main_router;
use salvo::prelude::*;
use salvo::oapi::extract::*;

use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;


pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());


pub struct Configurator {
    server_url: String,
    database_url: String,
}


impl Configurator {
    pub fn init() -> Configurator {
        let server_url: String = "127.0.0.1:5800".to_string();
        let database_url = "postgres://root:root@127.0.0.1:5432/test".to_string();
        Configurator { server_url, database_url }
    }

    pub fn enable_logging(&self) {
        tracing_subscriber::fmt().init()
    }

    pub async fn server(&self) {

        RB.link(PgDriver {}, &self.database_url).await.unwrap();
        let acceptor = TcpListener::new(&self.server_url).bind().await;
        Server::new(acceptor).serve(main_router()).await;
    }
}
