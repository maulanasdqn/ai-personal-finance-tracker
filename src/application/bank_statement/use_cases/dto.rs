pub struct UploadInput {
    pub workspace_id: String,
    pub file_name: String,
    pub file_data: Vec<u8>,
    pub content_type: String,
    pub created_by: String,
}
