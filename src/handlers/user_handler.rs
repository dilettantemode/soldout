use warp;

use crate::{
    db::mysql::POOL,
    repositories::user::{NewUser, User},
};

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::find(&id, &conn);

    let reply = match response {
        Ok(post) => {
            println!("{:#?}", &post);
            post
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_user: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_user.create(&conn);

    let reply = match response {
        Ok(new_user) => {
            println!("{:#?}", &new_user);
            new_user
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, update_post: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::update(&id, &update_post, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}
