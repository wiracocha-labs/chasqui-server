use actix_crud::models::entities::user::User;
use uuid::Uuid;
use mockall::predicate::*;
use actix_crud::models::entities::role::roles;
#[path = "../common/mocks/user_repository.rs"]
mod user_repository;
use user_repository::{UserRepository, MockUserRepository};

#[tokio::test]
async fn test_user_role_persistence() {
    // 1. Configurar el mock del repositorio
    let mut mock_repo = MockUserRepository::new();
    
    // 2. Crear datos de prueba
    let username = format!("test_user_{}", Uuid::new_v4());
    let username_clone = username.clone();  // Clonar aquí
    let email = format!("{}@example.com", &username);

    // 3. Configurar expectativas del mock
    mock_repo
        .expect_find_by_username()
        .with(eq(username.clone()))  // Usar el clone para el matcher
        .returning(move |_| {
            // Usar la variable clonada dentro del closure
            let mut user = User::new(
                username_clone.clone(),  // Usar la variable clonada
                format!("{}@example.com", username_clone),  // Usar la variable clonada
                "test_password".to_string()
            ).expect("Failed to create test user");
            
            // Agregar roles al usuario
            user.add_role(roles::admin());
            user.add_role(roles::moderator());
            
            Ok(Some(user))
        });

    // 4. Ejecutar la prueba - ahora username sigue siendo válido aquí
    let result = mock_repo.find_by_username(&username).await;
    
    // 5. Verificar resultados
    match result {
        Ok(Some(user)) => {
            assert_eq!(user.username, username);
            assert_eq!(user.email, Some(email));
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
    let username_clone = username.clone();  // Clonar aquí
    let email = format!("{}@example.com", &username);
    
    // 3. Configurar expectativas del mock
    mock_repo
        .expect_find_by_username()
        .with(eq(username.clone()))
        .returning(move |_| {
            // Crear un nuevo User con múltiples roles
            let mut user = User::new(
                username_clone.clone(),
                format!("{}@example.com", username_clone),
                "test_password".to_string()
            ).expect("Failed to create test user");
            
            // Agregar roles al usuario
            user.add_role(roles::admin());
            user.add_role(roles::moderator());
            
            Ok(Some(user))
        });
    
    // 4. Ejecutar la prueba
    let result = mock_repo.find_by_username(&username).await;
    
    // 5. Verificar resultados
    match result {
        Ok(Some(user)) => {
            assert_eq!(user.username, username);
            assert_eq!(user.email, Some(email));
            assert!(user.has_role("user"), "User should have default 'user' role");
            assert!(user.has_role("admin"), "User should have 'admin' role");
            assert!(user.has_role("moderator"), "User should have 'moderator' role");
        }
        _ => panic!("User should be found"),
    }
}