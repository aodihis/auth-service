use crate::config::load_config;
use crate::models::authorization::Role;
use crate::models::request::Permission;
use crate::services::permissions::Permissions;
use crate::services::role_permissions::RolePermissions;
use crate::services::roles::Roles;
use anyhow::Error;
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_role_permissions_success(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup service
    let config = Arc::new(load_config()?);
    let role_permissions = Arc::new(RolePermissions::new(pool.clone(), config));

    add_roles(&pool, "Role 1".to_string(), "Role 1 desc".to_string()).await?;
    add_permission(
        &pool,
        "resource 1".to_string(),
        "action 1".to_string(),
        "desc".to_string(),
    )
    .await?;
    add_role_permission(&pool, 1, 1).await?;

    let result = role_permissions.get_role_permissions(1).await;

    // Assert
    assert!(result.is_ok());
    let permissions = result?;

    assert_eq!(1, permissions.len());
    assert_eq!(permissions[0].resource, "resource 1".to_string());
    assert_eq!(permissions[0].action, "action 1".to_string());
    assert_eq!(permissions[0].description, "desc".to_string());

    Ok(())
}

#[sqlx::test]
async fn test_get_role_permissions_empty(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup service
    let config = Arc::new(load_config()?);
    let role_permissions = Arc::new(RolePermissions::new(pool.clone(), config));

    let result = role_permissions.get_role_permissions(1).await;

    // Assert
    assert!(result.is_ok());
    let permissions = result?;

    assert_eq!(0, permissions.len());
    Ok(())
}

#[sqlx::test]
async fn test_add_role_permissions(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup service
    let config = Arc::new(load_config()?);
    let role_permissions = Arc::new(RolePermissions::new(pool.clone(), config));

    add_roles(&pool, "Role 1".to_string(), "Role 1 desc".to_string()).await?;
    add_permission(
        &pool,
        "resource 1".to_string(),
        "action 1".to_string(),
        "desc".to_string(),
    )
    .await?;

    let result = role_permissions.add_permissions_for_role(1, vec![1]).await;
    assert!(result.is_ok());

    // Do it twice, should be no problem
    let result = role_permissions.add_permissions_for_role(1, vec![1]).await;
    assert!(result.is_ok());
    let result = role_permissions.get_role_permissions(1).await;

    // Assert
    assert!(result.is_ok());
    let permissions = result?;

    assert_eq!(1, permissions.len());
    assert_eq!(permissions[0].resource, "resource 1".to_string());
    assert_eq!(permissions[0].action, "action 1".to_string());
    assert_eq!(permissions[0].description, "desc".to_string());

    Ok(())
}

#[sqlx::test]
async fn test_add_role_permissions_failed(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup service
    let config = Arc::new(load_config()?);
    let role_permissions = Arc::new(RolePermissions::new(pool.clone(), config));

    add_roles(&pool, "Role 1".to_string(), "Role 1 desc".to_string()).await?;
    add_permission(
        &pool,
        "resource 1".to_string(),
        "action 1".to_string(),
        "desc".to_string(),
    )
    .await?;

    let res = role_permissions.add_permissions_for_role(1, vec![2]).await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Permission not found");

    let res = role_permissions.add_permissions_for_role(2, vec![1]).await;
    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Role not found");
    Ok(())
}

#[sqlx::test]
async fn test_delete_permissions_for_role(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup service
    let config = Arc::new(load_config()?);
    let role_permissions = Arc::new(RolePermissions::new(pool.clone(), config));

    add_roles(&pool, "Role 1".to_string(), "Role 1 desc".to_string()).await?;
    add_permission(
        &pool,
        "resource 1".to_string(),
        "action 1".to_string(),
        "desc".to_string(),
    )
    .await?;
    add_role_permission(&pool, 1, 1).await?;

    let res = role_permissions.delete_permissions_for_role(1).await;
    assert!(res.is_ok());

    // Should be fine if delete unknown role
    let res = role_permissions.delete_permissions_for_role(2).await;
    assert!(res.is_ok());

    Ok(())
}

async fn add_roles(pool: &PgPool, name: String, desc: String) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO roles (name, description) VALUES ($1, $2)")
        .bind(name)
        .bind(desc)
        .execute(pool)
        .await?;
    Ok(())
}

async fn add_permission(
    pool: &PgPool,
    resource: String,
    action: String,
    description: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO permissions (name, description, resource, action) VALUES($1, $2, $3, $4)",
    )
    .bind(format!("{}:{}", resource, action))
    .bind(description)
    .bind(resource)
    .bind(action)
    .execute(pool)
    .await?;
    Ok(())
}

async fn add_role_permission(
    pool: &PgPool,
    role_id: i32,
    permission_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)")
        .bind(role_id)
        .bind(permission_id)
        .execute(pool)
        .await?;
    Ok(())
}
