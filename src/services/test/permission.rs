use crate::config::load_config;
use crate::models::request::Permission as PermissionRequest;
use crate::services::permissions::Permissions;
use anyhow::Error;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;
use crate::error::authorization::AuthorizationError;

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

#[sqlx::test]
async fn test_update_permission_success(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let permission_service = Permissions::new(pool.clone(), config);

    let perm = PermissionRequest {
        resource: "user".to_string(),
        action: "view".to_string(),
        description: "Some desc".to_string(),
    };

    let inserted = permission_service.add(perm).await?;

    let updated_perm = PermissionRequest {
        description: "Can edit users".to_string(),
        resource: "user".to_string(),
        action: "edit".to_string(),
    };

    let result = permission_service.update(inserted.id, updated_perm).await;

    assert!(result.is_ok());
    let model = result?;
    assert_eq!(model.resource, "user");
    assert_eq!(model.action, "edit");
    assert_eq!(model.description, "Can edit users".to_string());

    Ok(())
}


#[sqlx::test]
async fn test_update_permission_not_found(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let permission_service = Permissions::new(pool.clone(), config);

    let update_data = PermissionRequest {
        description: "Non-existent update".to_string(),
        resource: "ghost".to_string(),
        action: "speak".to_string(),
    };

    let result = permission_service.update(9999, update_data).await;

    assert!(matches!(result, Err(AuthorizationError::NotFound)));

    Ok(())
}


#[sqlx::test]
async fn test_update_permission_duplicate(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let permission_service = Permissions::new(pool.clone(), config);

    let perm1 = PermissionRequest {
        description: "View".to_string(),
        resource: "product".to_string(),
        action: "view".to_string(),
    };

    let perm2 = PermissionRequest {
        description: "Edit".to_string(),
        resource: "product".to_string(),
        action: "edit".to_string(),
    };

    let inserted1 = permission_service.add(perm1).await?;
    let _inserted2 = permission_service.add(perm2).await?;

    let perm2 = PermissionRequest {
        description: "Edit".to_string(),
        resource: "product".to_string(),
        action: "edit".to_string(),
    };

    // Attempt to update perm1 to have same name as perm2
    let result = permission_service.update(inserted1.id, perm2).await;

    assert!(matches!(result, Err(AuthorizationError::PermissionAlreadyExist)));
    Ok(())
}
