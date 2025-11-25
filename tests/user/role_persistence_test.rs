use actix_crud::models::entities::user::User;
use uuid::Uuid;
use mockall::predicate::*;
use actix_crud::models::entities::role::roles;
mod common;
use common::mocks::user_repository::MockUserRepository;

// Función auxiliar para crear un usuario de prueba
fn create_test_user(username: &str, email: &str) -> User {
    User::new(
        username.to_string(),
        email.to_string(),
        "test_password".to_string()
    ).expect("Failed to create test user")
}

#[tokio::test]
async fn test_user_role_persistence() {
    // 1. Configurar el mock del repositorio
    let mut mock_repo = MockUserRepository::new();
    
    // 2. Crear datos de prueba
    let username = format!("test_user_{}", Uuid::new_v4());
    let email = format!("{}@example.com", &username);
    
    // 3. Configurar expectativas del mock sin clonar el User
    mock_repo
        .expect_find_by_username()
        .with(eq(username.clone()))
        .returning(move |_| {
            // Crear un nuevo User dentro del closure
            let user = User::new(
                username.clone(),
                format!("{}@example.com", username),
                "test_password".to_string()
            ).expect("Failed to create test user");
            Ok(Some(user))
        });
    
    // 4. Ejecutar la prueba
    let result = mock_repo.find_by_username(&username).await;
    
    // 5. Verificar resultados
    match result {
        Ok(Some(user)) => {
            assert_eq!(user.username, username);
            assert!(user.has_role("user"), "User should have default 'user' role");
        }
        _ => panic!("User should be found"),
    }
}

#[tokio::test]
async fn test_user_with_multiple_roles() {
    // 1. Configurar el mock del repositorio
    let mut mock_repo = MockUserRepository::new();
    
    // 2. Crear datos de prueba
    let username = format!("test_user_{}", Uuid::new_v4());
    let email = format!("{}@example.com", &username);
    
    // 3. Configurar expectativas del mock
    mock_repo
        .expect_find_by_username()
        .with(eq(username.clone()))
        .returning(move |_| {
            // Crear un nuevo User con múltiples roles
            let mut user = User::new(
                username.clone(),
                format!("{}@example.com", username),
                "test_password".to_string()
            ).expect("Failed to create test user");
            
            // Agregar roles al usuario
            user.add_role(roles::admin());
            user.add_role(roles::editor());
            
            Ok(Some(user))
        });
    
    // 4. Ejecutar la prueba
    let result = mock_repo.find_by_username(&username).await;
    
    // 5. Verificar resultados
    match result {
        Ok(Some(user)) => {
            assert_eq!(user.username, username);
            assert!(user.has_role("user"), "User should have default 'user' role");
            assert!(user.has_role("admin"), "User should have 'admin' role");
            assert!(user.has_role("editor"), "User should have 'editor' role");
        }
        _ => panic!("User should be found"),
    }
}