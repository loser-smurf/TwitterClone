use crate::database::{DbPool, get_db_conn};
use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Inserts a new user and returns the created user
pub fn insert_user(pool: &DbPool, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    diesel::insert_into(users)
        .values(new_user)
        .get_result(&mut conn)
}

/// Checks if a user exists with the given username or email.
/// Returns `true` if such user exists, otherwise `false`.
pub fn does_user_exist(
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

/// Finds a user by username or email.
/// Returns `Ok(Some(user))` if found, `Ok(None)` if not found.
pub fn find_user_by_username_or_email(
    pool: &DbPool,
    username_or_email: &str,
) -> Result<Option<User>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let user_found = users
        .filter(username.eq(username_or_email))
        .or_filter(email.eq(username_or_email))
        .first::<User>(&mut conn)
        .optional()?;

    Ok(user_found)
}

/// Finds a user by ID.
/// Returns 'Ok(Some(user))' if found, 'Ok(None)' if not found.
pub fn find_user_by_id(
    pool: &DbPool,
    user_id: &Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let user_found = users
        .filter(id.eq(user_id))
        .first::<User>(&mut conn)
        .optional()?;
    Ok(user_found)
}

/// Gets a paginated list of users with optional search
pub fn get_users(
    pool: &DbPool,
    page: i64,
    per_page: i64,
    search_query: Option<&str>,
) -> Result<(Vec<User>, i64), diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;
    
    let offset = (page - 1) * per_page;
    
    // Get total count first
    let total_count = if let Some(search) = search_query {
        let search_pattern = format!("%{}%", search);
        users
            .filter(
                username.ilike(&search_pattern)
                    .or(name.ilike(&search_pattern))
                    .or(bio.ilike(&search_pattern))
            )
            .count()
            .get_result(&mut conn)?
    } else {
        users.count().get_result(&mut conn)?
    };
    
    // Build the query for results
    let users_list = if let Some(search) = search_query {
        let search_pattern = format!("%{}%", search);
        users
            .filter(
                username.ilike(&search_pattern)
                    .or(name.ilike(&search_pattern))
                    .or(bio.ilike(&search_pattern))
            )
            .order(created_at.desc())
            .offset(offset)
            .limit(per_page)
            .load::<User>(&mut conn)?
    } else {
        users
            .order(created_at.desc())
            .offset(offset)
            .limit(per_page)
            .load::<User>(&mut conn)?
    };
    
    Ok((users_list, total_count))
}
