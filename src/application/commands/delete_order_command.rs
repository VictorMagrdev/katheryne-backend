use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::application::commands::delete_command::delete_command;
use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::tables::tables_name::ORDER;

/// ## Descripción
/// Función que elimina una orden del repositorio utilizando su ID.
///
/// ## Precondición
/// - Se debe proporcionar un ID válido de la orden a eliminar.
/// - Debe existir una instancia de `OrderRepository`.
///
/// ## Poscondición
/// - Si la orden con el ID proporcionado existe, se eliminará del repositorio.
/// - Si la orden no existe, se devolverá un mensaje de error indicando que no se encontró la orden.


pub async fn delete_order_command( Path(id): Path<String>)
                                    -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    delete_command::<Order>(Path(id),ORDER).await
}