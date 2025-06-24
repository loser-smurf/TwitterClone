use crate::database::DbPool;
use crate::jwt::AuthenticatedUser;
use crate::models::users::User;
use crate::models::users::{UserPublic, UserUpdate};
use crate::repositories::followers::{get_followers_repo, get_followings_repo};
use crate::repositories::users::{delete_user_repo, find_user_by_id, get_users, update_user_repo};
use crate::requests::users::UsersQuery;
use actix_web::{Error, HttpResponse, web};
use uuid::Uuid;

/// Get paginated list of users with optional search
pub async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<UsersQuery>,
) -> Result<HttpResponse, Error> {
    let (users_list, total_count) =
        get_users(&pool, query.page, query.per_page, query.search.as_deref()).map_err(|e| {
            eprintln!("Database query error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    // Convert users to public format (without sensitive data)
    let public_users: Vec<UserPublic> = users_list
        .into_iter()
        .map(|user: User| user.into())
        .collect();

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

pub async fn get_user(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from string to UUID
    let user_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let user = find_user_by_id(&pool, &user_id)
        .map_err(|e| {
            eprintln!("Database query error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    // Convert to public format
    let public_user: UserPublic = user.into();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(public_user))
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    request: web::Json<UserUpdate>,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    // Update user in database
    let updated_user = update_user_repo(&pool, &user_id, &request.into_inner()).map_err(|e| {
        eprintln!("Database update error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    // Convert to public format
    let public_user: UserPublic = updated_user.into();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(public_user))
}

/// Delete a user
pub async fn delete_user(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    // Parse user_id from JWT token
    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Ivalid user ID"))?;

    match delete_user_repo(&pool, &user_id) {
        Ok(true) => Ok(HttpResponse::Ok().finish()),
        Ok(false) => Err(actix_web::error::ErrorNotFound("User not found")),
        Err(e) => {
            eprintln!("Database delete error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Database error"))
        }
    }
}

/// Get followers of a user
pub async fn get_followers(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let followers = get_followers_repo(&pool, &user_id).map_err(|e| {
        eprintln!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(followers))
}

/// Get followings of a user
pub async fn get_followings(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let followings = get_followings_repo(&pool, &user_id).map_err(|e| {
        eprintln!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(followings))
}
