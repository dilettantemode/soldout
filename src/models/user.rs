use crate::schema::user;

use crate::schema::user::dsl;
use crate::schema::user::dsl::*;
use diesel::mysql::MysqlConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use std::time::SystemTime;

use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

impl NewUser {
    pub fn create(&self, connection: &MysqlConnection) -> Result<User, diesel::result::Error> {
        diesel::insert_into(user::table)
            .values(self)
            .get_result(connection)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn find(
        user_id: &i32,
        connection: &MysqlConnection,
    ) -> Result<User, diesel::result::Error> {
        user::table.find(user_id).first(connection)
    }

    pub fn delete(
        user_id: &i32,
        connection: &MysqlConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::user.find(user_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        user_id: &i32,
        new_post: &NewUser,
        connection: &MysqlConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(dsl::user.find(user_id))
            .set(new_post)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn list(connection: &MysqlConnection) -> Self {
        let result = user
            .limit(10)
            .load::<User>(connection)
            .expect("Error loading posts");
        UserList(result)
    }
}
