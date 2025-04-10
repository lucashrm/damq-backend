use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub discord_id: i64,
    pub anilist_username: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub discord_id: i64,
    pub anilist_username: &'a str,
}

pub fn create_user(conn: &mut MysqlConnection, discord_id: i64, anilist_username: &str) {
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

pub fn get_user(conn: &mut MysqlConnection, id_value: i64) -> Option<User> {
    use crate::schema::users::dsl::*;

    let mut results = users
        .filter(discord_id.eq(&id_value))
        .select(User::as_select())
        .load(conn)
        .expect("Error loading user");

    if results.len() > 0 {
        let user = results.pop().unwrap();
        return Some(user);
    }

    None
}