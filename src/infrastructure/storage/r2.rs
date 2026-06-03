use crate::error::AppError;
use worker::Bucket;

pub async fn upload(bucket: &Bucket, key: &str, data: Vec<u8>, _content_type: &str) -> Result<(), AppError> {
    bucket
        .put(key, data)
        .execute()
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(())
}
