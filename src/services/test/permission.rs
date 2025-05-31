use crate::config::load_config;
use crate::models::request::Permission as PermissionRequest;
use crate::services::permissions::Permissions;
use anyhow::Error;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test()]
async fn test_add_list_delete_permission(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let permission_service = Permissions::new(pool.clone(), config);

    //delete non exist permission
    let delete_result = permission_service.delete(87).await;
    assert!(delete_result.is_err(), "Service will return an error");


    // Add permission
    let perm = PermissionRequest {
        resource: "permission".to_string(),
        action: "action".to_string(),
        description: "example data".to_string(),
    };

    let add_result = permission_service.add(perm).await;
    assert!(add_result.is_ok(), "Permission should be added successfully");

    // List permissions
    let list = permission_service.list().await;
    assert!(list.is_ok(), "Should be able to list permissions");
    let list = list?;
    assert_eq!(list.len(), 1, "There should be one permission");
    assert_eq!(list[0].name, "permission:action");
    assert_eq!(list[0].description, "example data");
    assert_eq!(list[0].resource, "permission");
    assert_eq!(list[0].action, "action");

    let perm = PermissionRequest {
        resource: "permission".to_string(),
        action: "action".to_string(),
        description: "data".to_string(),
    };
    let add_result = permission_service.add(perm).await;
    assert!(add_result.is_err(), "Should be able error");

    // Delete permission
    let delete_result = permission_service.delete(list[0].id).await;
    assert!(delete_result.is_ok(), "Permission should be deleted");

    Ok(())
}