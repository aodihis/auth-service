use crate::config::load_config;
use crate::error::authorization::AuthorizationError;
use crate::models::request::Role;
use crate::services::roles::Roles;
use anyhow::Error;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test()]
async fn test_add_role_success(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let new_role = Role {
        name: "admin".to_string(),
        description: "Administrator".to_string(),
        permission_ids: vec![],
    };

    let result = roles_service.add(new_role).await;

    assert!(result.is_ok());
    let model = result?;
    assert_eq!(model.name, "admin".to_string());
    assert_eq!(model.description, "Administrator".to_string());

    Ok(())
}

#[sqlx::test()]
async fn test_add_duplicate_role(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let role = Role {
        name: "admin".to_string(),
        description: "Administrator".to_string(),
        permission_ids: vec![],
    };

    roles_service.add(role).await?;

    let role = Role {
        name: "admin".to_string(),
        description: "Administrator".to_string(),
        permission_ids: vec![],
    };

    let result = roles_service.add(role).await;

    assert!(
        matches!(result, Err(AuthorizationError::RoleAlreadyExist)),
        "Should return an error"
    );

    Ok(())
}

#[sqlx::test()]
async fn test_get_role_success(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let role = Role {
        name: "editor".to_string(),
        description: "Editor".to_string(),
        permission_ids: vec![],
    };

    let inserted = roles_service.add(role).await?;

    let result = roles_service.get(inserted.id).await;

    assert!(result.is_ok());
    assert_eq!(result?.name, "editor");

    Ok(())
}

#[sqlx::test()]
async fn test_get_role_not_found(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let result = roles_service.get(99999).await;

    assert!(matches!(result, Err(AuthorizationError::NotFound)));

    Ok(())
}

#[sqlx::test()]
async fn test_update_role_success(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let role = Role {
        name: "moderator".to_string(),
        description: "Mod".to_string(),
        permission_ids: vec![],
    };

    let inserted = roles_service.add(role).await?;

    let updated = Role {
        name: "supermod".to_string(),
        description: "Super Moderator".to_string(),
        permission_ids: vec![],
    };

    let result = roles_service.update(inserted.id, updated).await;

    assert!(result.is_ok());
    let model = result?;
    assert_eq!(model.name, "supermod".to_string());
    assert_eq!(model.description, "Super Moderator".to_string());

    Ok(())
}

#[sqlx::test()]
async fn test_update_role_not_found(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let update = Role {
        name: "ghost".to_string(),
        description: "".to_string(),
        permission_ids: vec![],
    };

    let result = roles_service.update(99999, update).await;

    assert!(matches!(result, Err(AuthorizationError::NotFound)));
    Ok(())
}

#[sqlx::test]
async fn test_delete_role_success(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let role = Role {
        name: "to-delete".to_string(),
        description: "".to_string(),
        permission_ids: vec![],
    };

    let inserted = roles_service.add(role).await?;

    let result = roles_service.delete(inserted.id).await;

    assert!(result.is_ok());

    Ok(())
}

#[sqlx::test]
async fn test_delete_role_not_found(pool: PgPool) -> Result<(), Error> {
    let config = Arc::new(load_config()?);
    let roles_service = Roles::new(pool.clone(), config);

    let result = roles_service.delete(99999).await;

    assert!(matches!(result, Err(AuthorizationError::NotFound)));
    Ok(())
}
