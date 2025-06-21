use crate::database::DbPool;
use crate::repositories::users::get_users;
use crate::requests::users::UsersQuery;
use crate::models::users::UserPublic;
use crate::models::users::User;
use actix_web::{HttpResponse, web, Error};

/// Get paginated list of users with optional search
pub async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<UsersQuery>,
) -> Result<HttpResponse, Error> {
    let (users_list, total_count) = get_users(
        &pool,
        query.page,
        query.per_page,
        query.search.as_deref(),
    )
    .map_err(|e| {
        eprintln!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    // Convert users to public format (without sensitive data)
    let public_users: Vec<UserPublic> = users_list.into_iter().map(|user: User| user.into()).collect();

    // Calculate pagination info
    let total_pages = (total_count + query.per_page - 1) / query.per_page;
    let has_next = query.page < total_pages;
    let has_prev = query.page > 1;

    // Build response
    let response = serde_json::json!({
        "users": public_users,
        "pagination": {
            "page": query.page,
            "per_page": query.per_page,
            "total_count": total_count,
            "total_pages": total_pages,
            "has_next": has_next,
            "has_prev": has_prev
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}
