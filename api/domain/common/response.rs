use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ApiListResponse<T> {
    pub message: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T: Serialize> ApiListResponse<T> {
    pub fn new(data: Vec<T>, meta: PaginationMeta, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            data,
            meta,
        }
    }
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub per_page: u64,
    pub total_page: u64,
    pub total_data: u64,
}

impl PaginationMeta {
    pub fn new(page: u64, per_page: u64, total_data: u64) -> Self {
        let denom = per_page.max(1);
        let total_page = total_data.div_ceil(denom).max(1);
        Self {
            page,
            per_page,
            total_page,
            total_data,
        }
    }

    pub fn unpaged(total_data: u64) -> Self {
        Self {
            page: 1,
            per_page: total_data,
            total_page: 1,
            total_data,
        }
    }
}
