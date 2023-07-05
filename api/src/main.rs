#[macro_use]
extern crate rbatis;
extern crate rbdc;

pub mod configurator;
pub mod router_creator;
pub mod endpoints;
pub mod misc;
pub use configurator::Configurator;

#[tokio::main]
async fn main() {
    let conf = Configurator::init();
    conf.enable_logging();
    conf.server().await;
}
