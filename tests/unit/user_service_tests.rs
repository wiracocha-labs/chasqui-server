use crate::common::test_utils::{create_test_user, create_mock_user_repository};

#[tokio::test]
async fn test_create_user() {
    let mut mock = create_mock_user_repository();
    let test_user = create_test_user();

    mock.expect_create_user()
        .returning(move |_| Ok(test_user.clone()));

    // Aquí irían tus pruebas usando el mock
    let result = mock.create_user(test_user).await;
    assert!(result.is_ok());
}