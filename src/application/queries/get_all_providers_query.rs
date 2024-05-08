use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

pub async fn get_all_providers_query() -> impl IntoResponse {
    let repository:ProviderRepository = ProviderRepository::new(PROVIDER);

    let mut query: Vec<Provider> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        query = result;
    }

    let json_response:Value = serde_json::json!({
        "status": "success",
        "results": query.len(),
        "query": query,
    });

    Json(json_response)
}