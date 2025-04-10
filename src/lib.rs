pub mod schema;
pub mod models;

use diesel::prelude::*;
use self::models::{User, NewUser};

pub fn create_user(conn: &mut MysqlConnection, discord_id: i32, anilist_username: &str) {
    use crate::schema::users;

    let new_user = NewUser { discord_id, anilist_username };

    conn.transaction(|conn| {
        diesel::insert_into(users::table).values(&new_user).execute(conn)?;

        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first(conn)
    }).expect("Error while saving user");
}