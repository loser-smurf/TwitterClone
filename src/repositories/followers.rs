use crate::database::{DbPool, get_db_conn};
use crate::models::follows::{Follow, NewFollow};
use crate::models::users::{User, UserPublic};
use crate::schema::follows::dsl::*;
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

/// Gets followers of a user
pub fn get_followers_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
) -> Result<Vec<UserPublic>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let followers_list = follows
        .inner_join(users::table.on(users::id.eq(follower_id)))
        .filter(followed_id.eq(user_id_val))
        .select(users::all_columns)
        .load::<User>(&mut conn)?;

    Ok(followers_list.into_iter().map(UserPublic::from).collect())
}

/// Gets followings of a user
pub fn get_followings_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
) -> Result<Vec<UserPublic>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let followings_list = follows
        .inner_join(users::table.on(users::id.eq(followed_id)))
        .filter(follower_id.eq(user_id_val))
        .select(users::all_columns)
        .load::<User>(&mut conn)?;

    Ok(followings_list.into_iter().map(UserPublic::from).collect())
}

/// Follows a user
pub fn follow_user_repo(
    pool: &DbPool,
    follower_id_val: &Uuid,
    followed_id_val: &Uuid,
) -> Result<Follow, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let new_follow = NewFollow {
        follower_id: *follower_id_val,
        followed_id: *followed_id_val,
    };

    diesel::insert_into(follows)
        .values(&new_follow)
        .get_result(&mut conn)
}

/// Unfollows a user
pub fn unfollow_user_repo(
    pool: &DbPool,
    follower_id_val: &Uuid,
    followed_id_val: &Uuid,
) -> Result<bool, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    diesel::delete(
        follows.filter(
            follower_id
                .eq(follower_id_val)
                .and(followed_id.eq(followed_id_val)),
        ),
    )
    .execute(&mut conn)?;

    Ok(true)
}

/// Checks if a user is followed by another user
pub fn is_followed_repo(
    pool: &DbPool,
    follower_id_val: &Uuid,
    followed_id_val: &Uuid,
) -> Result<bool, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let is_followed = follows
        .filter(
            follower_id
                .eq(follower_id_val)
                .and(followed_id.eq(followed_id_val)),
        )
        .first::<Follow>(&mut conn)
        .optional()?;

    Ok(is_followed.is_some())
}
