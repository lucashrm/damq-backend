// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        discord_id -> Integer,
        #[max_length = 50]
        anilist_username -> Varchar,
    }
}
