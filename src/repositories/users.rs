use crate::database::{DbPool, get_db_conn};
use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

/// Inserts a new user and returns the created user
pub fn insert_user(pool: &DbPool, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    diesel::insert_into(users)
        .values(new_user)
        .get_result(&mut conn)
}

/// Checks if a user exists with the given username or email.
/// Returns `true` if such user exists, otherwise `false`.
pub fn find_user_by_username_or_email(
    pool: &DbPool,
    username_to_check: &str,
    email_to_check: &str,
) -> Result<bool, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let existing_user = users
        .filter(username.eq(username_to_check))
        .or_filter(email.eq(email_to_check))
        .first::<User>(&mut conn)
        .optional()?;

    Ok(existing_user.is_some())
}
