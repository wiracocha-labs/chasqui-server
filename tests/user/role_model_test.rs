// tests/role_model_test.rs
#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!role.has_permission(Permission::ManageUsers));
    }
}

// tests/user_model_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_role() {
        let mut user = User::new("testuser", "test@example.com", "password").unwrap();
        let role = Role::new("admin", "Administrator");
        
        assert!(user.add_role(role.clone()));
        assert!(user.has_role("admin"));
        assert!(!user.add_role(role)); // No debería duplicar
    }

    #[test]
    fn test_has_permission() {
        let mut user = User::new("testuser", "test@example.com", "password").unwrap();
        let role = Role::new("admin", "Administrator")
            .with_permissions(&[Permission::AdminAll]);
        
        user.add_role(role);
        assert!(user.has_permission(Permission::AdminAll));
    }
}