
use crate::endpoints::phone_comments::*;
use salvo::prelude::*;
use salvo::logging::Logger;

fn phone_comments_router() -> Router {
    Router::with_path("phoneComment")
        .get(show_comments)
        .post(add_comment)
}

pub fn main_router() -> Router {
    Router::new()
        .hoop(Logger::new())
        .push(phone_comments_router())
} 
