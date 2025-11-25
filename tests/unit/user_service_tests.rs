use crate::common::test_utils::{create_mock_user_repository, create_test_user};
use actix_crud::models::entities::user::User;

#[tokio::test]
async fn test_create_user() {
    let mut mock = create_mock_user_repository();
    let test_user = create_test_user();

    mock.expect_create_user()
        .returning(move |_| Ok(User::clone(&test_user)));

    // Aquí irían tus pruebas usando el mock
    let result = mock.create_user(test_user).await;
    assert!(result.is_ok());
}
