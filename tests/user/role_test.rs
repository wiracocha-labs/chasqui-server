use actix_crud::models::entities::role::Role;
use actix_crud::models::entities::role::Permission;

#[test]
fn test_role_creation() {
    let role = Role::new("admin", "Administrator");
    assert_eq!(role.name, "admin");
    assert_eq!(role.description, "Administrator");
    assert!(role.permissions.is_empty());
}

#[test]
fn test_has_permission() {
    let role = Role::new("admin", "Administrator")
        .with_permissions(&[Permission::AdminAll]);
    
    assert!(role.has_permission(Permission::AdminAll));
    assert!(role.has_permission(Permission::UserBan));
}