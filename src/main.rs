mod handlers;
mod routes;
use self::{handlers::user_handler, routes::user_route};
use console::Style;
use warp::{self, Filter};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;

mod db;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new().blue();

    let list_posts = user_route::list().and_then(user_handler::list);

    let get_post = user_route::get().and_then(user_handler::get);

    let create_post = user_route::create().and_then(user_handler::create);

    let update_post = user_route::update().and_then(user_handler::update);

    let delete_post = user_route::delete().and_then(user_handler::delete);

    let post_api = list_posts
        .or(get_post)
        .or(create_post)
        .or(update_post)
        .or(delete_post);

    let end = post_api.with(warp::log("post_api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl http://0.0.0.0:8000/api/post/v1 to test the end point.");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
