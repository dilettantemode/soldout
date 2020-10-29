use crate::schema::user;

use diesel::mysql::MysqlConnection;
use crate::models::user::{NewUser, User};

impl NewUser {
    pub fn create(&self, &connection: &MysqlConnection) {
        diesel::insert_into(user::table)
            .values(self)
            .execute(&connection)
            .unwrap();
        Self::last()
    }
}

impl User {
    pub fn all() -> Box<Vec<Self>> {
        use schema::tickets::dsl::*;

        let connection = database::establish_connection();
        let results = tickets.load::<Self>(&connection).expect(
            "Error loading tickets",
        );

        Box::new(results)
    }

    pub fn find(record: i32) -> Self {
        let connection = database::establish_connection();
        tickets::table
            .find(record)
            .first::<Self>(&connection)
       

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
