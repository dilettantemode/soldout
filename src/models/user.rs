use crate::schema::user;

use crate::schema::user::dsl;
use crate::schema::user::dsl::*;
use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn create(&self, &connection: &MysqlConnection) {
        diesel::insert_into(user::table)
            .values(self)
            .execute(&connection)
            .unwrap();
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn find(user_id: &u64, &connection: &MysqlConnection) {
        user::table.find(user_id).execute(&connection).unwrap();
    }

    pub fn update(user_id: &u64, new_user: &NewUser, connection: &MysqlConnection) {
        diesel::update(dsl::user.find(user_id))
            .set(new_user)
            .execute(connection)?;
        Ok(())
    }

    pub fn delete(user_id: &i32, connection: &MysqlConnection) {
        diesel::delete(dsl::user.find(user_id)).execute(connection)?;
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
