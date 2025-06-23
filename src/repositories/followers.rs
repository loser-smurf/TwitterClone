use crate::database::{DbPool, get_db_conn};
use crate::models::users::{User, UserPublic};
use crate::schema::follows::dsl::*;
use crate::schema::users::dsl as users_dsl;
use diesel::prelude::*;
use uuid::Uuid;

/// Get followers of a user
pub fn get_followers_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
) -> Result<Vec<UserPublic>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let followers_list = follows
        .inner_join(users_dsl::users.on(users_dsl::id.eq(follower_id)))
        .filter(followed_id.eq(user_id_val))
        .select(users_dsl::users::all_columns())
        .load::<User>(&mut conn)?;

    Ok(followers_list.into_iter().map(UserPublic::from).collect())
}

/// Get followings of a user
pub fn get_followings_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
) -> Result<Vec<UserPublic>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let followings_list = follows
        .inner_join(users_dsl::users.on(users_dsl::id.eq(followed_id)))
        .filter(follower_id.eq(user_id_val))
        .select(users_dsl::users::all_columns())
        .load::<User>(&mut conn)?;

    Ok(followings_list.into_iter().map(UserPublic::from).collect())
}   