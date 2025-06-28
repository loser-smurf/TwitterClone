use crate::database::{DbPool, get_db_conn};
use crate::models::likes::{Like, NewLike};
use crate::schema::likes::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Creates a like
pub fn like_tweet_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
    tweet_id_val: &Uuid,
) -> Result<Like, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let new_like = NewLike {
        id: Uuid::new_v4(),
        user_id: *user_id_val,
        tweet_id: *tweet_id_val,
    };

    diesel::insert_into(likes)
        .values(&new_like)
        .get_result(&mut conn)
}

/// Deletes a like
pub fn delete_like_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
    tweet_id_val: &Uuid,
) -> Result<usize, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    diesel::delete(likes)
        .filter(user_id.eq(user_id_val))
        .filter(tweet_id.eq(tweet_id_val))
        .execute(&mut conn)
        .map_err(|e| {
            eprintln!("Database delete like error: {}", e);
            e
        })
}

/// Gets likes for a tweet
pub fn get_likes_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
    tweet_id_val: &Uuid,
) -> Result<Vec<Like>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let likes_result = likes
        .filter(user_id.eq(user_id_val))
        .filter(tweet_id.eq(tweet_id_val))
        .load::<Like>(&mut conn)?;

    Ok(likes_result)
}