use super::dto::UploadInput;
use crate::domain::bank_statement::entity::{
    BankStatement, FileType, NewBankStatement, ProcessingStatus,
};
use crate::domain::bank_statement::repository::BankStatementRepository;
use crate::error::AppError;
use crate::infrastructure::{ai, storage::r2};
use base64::{engine::general_purpose::STANDARD, Engine};
use uuid::Uuid;
use worker::Bucket;

pub async fn execute(
    input: UploadInput,
    repo: &impl BankStatementRepository,
    bucket: &Bucket,
    api_key: &str,
) -> Result<BankStatement, AppError> {
    let file_type = detect_file_type(&input.content_type);
    let now = chrono::Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let file_key = format!("statements/{}/{id}", input.workspace_id);

    r2::upload(
        bucket,
        &file_key,
        input.file_data.clone(),
        &input.content_type,
    )
    .await?;

    repo.create(NewBankStatement {
        id: id.clone(),
        workspace_id: input.workspace_id,
        file_key,
        file_name: input.file_name,
        file_type,
        created_by: input.created_by,
        created_at: now.clone(),
        updated_at: now.clone(),
    })
    .await?;

    repo.update_status(&id, ProcessingStatus::Processing, None, None, &now)
        .await?;

    let b64 = STANDARD.encode(&input.file_data);
    let ai_result = ai::deepseek::analyze_image(
        &b64,
        &input.content_type,
        ai::BANK_STATEMENT_PROMPT,
        api_key,
    )
    .await;

    match ai_result {
        Ok(response) => {
            let parsed: serde_json::Value =
                serde_json::from_str(&response).unwrap_or(serde_json::Value::Array(vec![]));
            let summary_prompt =
                format!("In 2-3 sentences, summarize this bank statement data: {response}");
            let summary = ai::deepseek::analyze_text(&summary_prompt, api_key)
                .await
                .ok();
            repo.update_status(
                &id,
                ProcessingStatus::Processed,
                Some(parsed),
                summary,
                &now,
            )
            .await
        }
        Err(_) => {
            repo.update_status(&id, ProcessingStatus::Failed, None, None, &now)
                .await
        }
    }
}

fn detect_file_type(content_type: &str) -> FileType {
    if content_type.contains("pdf") {
        FileType::Pdf
    } else {
        FileType::Image
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn detect_jpeg_is_image() {
        assert_eq!(detect_file_type("image/jpeg"), FileType::Image);
    }

    #[wasm_bindgen_test]
    fn detect_png_is_image() {
        assert_eq!(detect_file_type("image/png"), FileType::Image);
    }

    #[wasm_bindgen_test]
    fn detect_webp_is_image() {
        assert_eq!(detect_file_type("image/webp"), FileType::Image);
    }

    #[wasm_bindgen_test]
    fn detect_pdf_is_pdf() {
        assert_eq!(detect_file_type("application/pdf"), FileType::Pdf);
    }
}
