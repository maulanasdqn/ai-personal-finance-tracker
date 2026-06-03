use crate::error::AppError;
use worker::Bucket;

pub async fn upload(bucket: &Bucket, key: &str, data: Vec<u8>, _content_type: &str) -> Result<(), AppError> {
    bucket
        .put(key, data)
        .execute()
        .await
        .map_err(|e| AppError::Internal(format!("R2 upload error: {}", e)))?;
    Ok(())
}

pub async fn download(bucket: &Bucket, key: &str) -> Result<Vec<u8>, AppError> {
    let obj = bucket
        .get(key)
        .execute()
        .await
        .map_err(|e| AppError::Internal(format!("R2 get error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("file not found".into()))?;

    let bytes = obj
        .body()
        .ok_or_else(|| AppError::NotFound("file has no body".into()))?
        .bytes()
        .await
        .map_err(|e| AppError::Internal(format!("R2 read error: {}", e)))?;
    Ok(bytes)
}

pub async fn delete(bucket: &Bucket, key: &str) -> Result<(), AppError> {
    bucket
        .delete(key)
        .await
        .map_err(|e| AppError::Internal(format!("R2 delete error: {}", e)))?;
    Ok(())
}
