use std::time::{SystemTime, UNIX_EPOCH};

use abbotik_cli::{
    api::{ApiClient, DissolveConfirmRequest, DissolveRequest, LoginRequest, RegisterRequest},
    config::AbbotikConfig,
    error::AbbotikError,
};
use reqwest::StatusCode;

fn live_client() -> ApiClient {
    ApiClient::new(AbbotikConfig::new("https://api.abbotik.com"))
        .expect("valid Abbotik API base url should construct client")
}

fn live_identity() -> (String, String, String, String) {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let suffix = format!("{}", stamp % 100_000);
    let tenant = format!("dissolve_{suffix}");
    let username = format!("u{suffix}");
    let email = format!("{username}@example.com");
    let password = format!("Passw0rd!{suffix}");
    (tenant, username, email, password)
}

#[tokio::test]
#[ignore = "uses the live Abbotik API"]
async fn dissolve_flow_registers_confirms_and_blocks_login() {
    if std::env::var_os("ABBOTIK_LIVE_API").is_none() {
        eprintln!("set ABBOTIK_LIVE_API=1 to run this live test");
        return;
    }

    let client = live_client();
    let (tenant, username, email, password) = live_identity();

    let register = client
        .auth_register(&RegisterRequest {
            tenant: Some(tenant.clone()),
            username: Some(username.clone()),
            email: Some(email),
            password: Some(password.clone()),
        })
        .await
        .expect("register should succeed");

    assert_eq!(register.data.as_ref().map(|data| data.tenant.as_str()), Some(tenant.as_str()));
    assert_eq!(register.data.as_ref().map(|data| data.username.as_str()), Some(username.as_str()));

    let dissolve = client
        .auth_dissolve(&DissolveRequest {
            tenant: Some(tenant.clone()),
            username: Some(username.clone()),
            password: Some(password.clone()),
        })
        .await
        .expect("dissolve request should succeed");

    let confirmation_token = dissolve
        .data
        .as_ref()
        .map(|data| data.confirmation_token.clone())
        .expect("dissolve should return confirmation token");
    assert_eq!(dissolve.data.as_ref().map(|data| data.expires_in), Some(300));

    let confirm = client
        .auth_dissolve_confirm(&DissolveConfirmRequest {
            confirmation_token,
        })
        .await
        .expect("dissolve confirm should succeed");

    assert_eq!(confirm.data.as_ref().map(|data| data.tenant.as_str()), Some(tenant.as_str()));
    assert_eq!(confirm.data.as_ref().map(|data| data.username.as_str()), Some(username.as_str()));
    assert_eq!(confirm.data.as_ref().map(|data| data.dissolved), Some(true));

    let login_err = client
        .auth_login(&LoginRequest {
            tenant: Some(tenant),
            tenant_id: None,
            username: Some(username),
            password: Some(password),
            format: None,
        })
        .await
        .expect_err("login should fail after dissolution");

    match login_err {
        AbbotikError::Http { status, .. } => assert_eq!(status, StatusCode::UNAUTHORIZED),
        other => panic!("expected HTTP unauthorized error, got {other:?}"),
    }
}
