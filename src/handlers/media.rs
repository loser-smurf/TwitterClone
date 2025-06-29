use actix_web::{web, HttpResponse, Error};
use crate::jwt::AuthenticatedUser;
use uuid::Uuid;
use mime_guess::from_path;
use crate::storage::S3Storage;
use std::path::Path;

/// Uploads a media file
pub async fn upload_media(
    storage: web::Data<S3Storage>,
    user: AuthenticatedUser,
    req: actix_web::HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {

    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;
    
    // Get the original filename from the request headers
    let original_name = req.headers().get("X-Filename").unwrap().to_str().unwrap();

    // Check file extension
    let allowed_exts = ["png", "jpg", "jpeg", "gif", "mp4", "mov", "webm"];
    let ext = Path::new(original_name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    if !allowed_exts.contains(&ext.as_str()) {
        return Err(actix_web::error::ErrorBadRequest(
            "Only .png, .jpg, .jpeg, .gif, .mp4, .mov, .webm files are allowed",
        ));
    }

    // MIME type check (additional to extension)
    let mime_type = from_path(original_name)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());
    if !mime_type.starts_with("image/") && !mime_type.starts_with("video/") {
        return Err(actix_web::error::ErrorBadRequest("Only image and video files are allowed"));
    }

    // Save file to S3
    let (_orig, key, _size, _mime) = storage.save_file(&req, payload, user_id   ).await?;

    let s3_url = format!("https://{}.s3.amazonaws.com/{}", storage.bucket_name, key);

    Ok(HttpResponse::Ok().json(s3_url))
}

