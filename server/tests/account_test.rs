mod common;

use server::services::account::{self, CreateAccountRequest, UpdateAccountRequest};

#[tokio::test]
async fn test_account_crud() {
    let db = common::setup_test_db().await;
    let user_id = common::create_test_user(&db).await;

    let req = CreateAccountRequest {
        name: "Test Account".to_string(),
        r#type: "bank_card".to_string(),
        currency_code: "USD".to_string(),
    };

    let created = account::create_account(&db, user_id, req)
        .await
        .expect("Failed to create account");

    assert_eq!(created.name, "Test Account");
    assert_eq!(created.r#type, "bank_card");
    assert_eq!(created.currency_code, "USD");

    let fetched = account::get_account(&db, user_id, created.id)
        .await
        .expect("Failed to get account");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, "Test Account");

    let update_req = UpdateAccountRequest {
        name: Some("Updated Account".to_string()),
        r#type: Some("cash".to_string()),
        currency_code: Some("EUR".to_string()),
    };

    let updated = account::update_account(&db, user_id, created.id, update_req)
        .await
        .expect("Failed to update account");

    assert_eq!(updated.name, "Updated Account");
    assert_eq!(updated.r#type, "cash");
    assert_eq!(updated.currency_code, "EUR");

    account::delete_account(&db, user_id, created.id)
        .await
        .expect("Failed to delete account");

    let result = account::get_account(&db, user_id, created.id).await;
    assert!(result.is_err(), "Should not be able to get deleted account");

    common::cleanup_test_user(&db, user_id).await;
}

#[tokio::test]
async fn test_account_user_isolation() {
    let db = common::setup_test_db().await;
    let user_a = common::create_test_user(&db).await;
    let user_b = common::create_test_user(&db).await;

    let req = CreateAccountRequest {
        name: "User A Account".to_string(),
        r#type: "cash".to_string(),
        currency_code: "USD".to_string(),
    };

    let account_a = account::create_account(&db, user_a, req)
        .await
        .expect("Failed to create account for user A");

    let result = account::get_account(&db, user_b, account_a.id).await;
    assert!(result.is_err(), "User B should not access User A's account");

    let result = account::update_account(
        &db,
        user_b,
        account_a.id,
        UpdateAccountRequest {
            name: Some("Hacked".to_string()),
            r#type: None,
            currency_code: None,
        },
    )
    .await;
    assert!(
        result.is_err(),
        "User B should not update User A's account"
    );

    let result = account::delete_account(&db, user_b, account_a.id).await;
    assert!(
        result.is_err(),
        "User B should not delete User A's account"
    );

    let list_b = account::list_accounts(&db, user_b)
        .await
        .expect("Failed to list accounts for user B");
    assert_eq!(list_b.len(), 0, "User B should see no accounts");

    let list_a = account::list_accounts(&db, user_a)
        .await
        .expect("Failed to list accounts for user A");
    assert_eq!(list_a.len(), 1, "User A should see 1 account");
    assert_eq!(list_a[0].id, account_a.id);

    common::cleanup_test_user(&db, user_a).await;
    common::cleanup_test_user(&db, user_b).await;
}

#[tokio::test]
async fn test_account_currency_validation() {
    let db = common::setup_test_db().await;
    let user_id = common::create_test_user(&db).await;

    let req = CreateAccountRequest {
        name: "Invalid Currency".to_string(),
        r#type: "bank_card".to_string(),
        currency_code: "INVALID".to_string(),
    };

    let result = account::create_account(&db, user_id, req).await;
    assert!(
        result.is_err(),
        "Should reject invalid currency code (too long)"
    );

    let req = CreateAccountRequest {
        name: "Short Currency".to_string(),
        r#type: "bank_card".to_string(),
        currency_code: "US".to_string(),
    };

    let result = account::create_account(&db, user_id, req).await;
    assert!(
        result.is_err(),
        "Should reject invalid currency code (too short)"
    );

    let req = CreateAccountRequest {
        name: "Valid Currency".to_string(),
        r#type: "bank_card".to_string(),
        currency_code: "CNY".to_string(),
    };

    let account = account::create_account(&db, user_id, req)
        .await
        .expect("Valid currency should be accepted");
    assert_eq!(account.currency_code, "CNY");

    common::cleanup_test_user(&db, user_id).await;
}

#[tokio::test]
async fn test_account_type_normalization() {
    let db = common::setup_test_db().await;
    let user_id = common::create_test_user(&db).await;

    let req = CreateAccountRequest {
        name: "Mixed Case Type".to_string(),
        r#type: "  BaNk_CaRd  ".to_string(),
        currency_code: "usd".to_string(),
    };

    let account = account::create_account(&db, user_id, req)
        .await
        .expect("Should normalize account type and currency");

    assert_eq!(account.r#type, "bank_card");
    assert_eq!(account.currency_code, "USD");

    common::cleanup_test_user(&db, user_id).await;
}
