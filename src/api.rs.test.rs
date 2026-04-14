use crate::api::{
    ChallengeData, DissolveConfirmRequest, DissolveData, DissolveRequest, ProvisionData,
    ProvisionKeyData, ProvisionUserData, RefreshData, RegisterData, VerifyData,
};

#[test]
fn dissolve_request_and_confirm_types_are_structural() {
    let request = DissolveRequest {
        tenant: Some("acme".to_string()),
        username: Some("alice".to_string()),
        password: Some("secret".to_string()),
    };
    let confirm = DissolveConfirmRequest {
        confirmation_token: "token".to_string(),
    };

    assert_eq!(request.tenant.as_deref(), Some("acme"));
    assert_eq!(request.username.as_deref(), Some("alice"));
    assert_eq!(request.password.as_deref(), Some("secret"));
    assert_eq!(confirm.confirmation_token, "token");
}

#[test]
fn dissolve_and_refresh_payloads_have_expected_fields() {
    let dissolve = DissolveData {
        confirmation_token: "token".to_string(),
        expires_in: 300,
    };
    let refresh = RefreshData {
        token: "jwt".to_string(),
        expires_in: 604800,
    };

    assert_eq!(dissolve.expires_in, 300);
    assert_eq!(refresh.expires_in, 604800);
}

#[test]
fn register_and_machine_auth_payloads_have_expected_fields() {
    let register = RegisterData {
        tenant_id: "tenant-1".to_string(),
        tenant: "acme".to_string(),
        username: "alice".to_string(),
        status: "pending".to_string(),
    };
    let provision = ProvisionData {
        tenant: "acme".to_string(),
        tenant_id: "tenant-1".to_string(),
        user: ProvisionUserData {
            id: "user-1".to_string(),
            username: "machine_root".to_string(),
            access: "root".to_string(),
        },
        key: ProvisionKeyData {
            id: "key-1".to_string(),
            name: Some("ci-runner".to_string()),
            algorithm: "ed25519".to_string(),
            fingerprint: "fp_1234".to_string(),
        },
        challenge: crate::api::ProvisionChallengeData {
            challenge_id: "challenge-1".to_string(),
            nonce: "nonce".to_string(),
            expires_in: 300,
        },
    };
    let challenge = ChallengeData {
        challenge_id: "challenge-1".to_string(),
        nonce: "nonce".to_string(),
        issued_at: "2026-04-14T00:00:00Z".to_string(),
        expires_in: 300,
        algorithm: "ed25519".to_string(),
    };
    let verify = VerifyData {
        token: "jwt".to_string(),
        expires_in: 900,
        tenant: "acme".to_string(),
        tenant_id: "tenant-1".to_string(),
        key_id: "key-1".to_string(),
    };

    assert_eq!(register.status, "pending");
    assert_eq!(provision.key.fingerprint, "fp_1234");
    assert_eq!(challenge.algorithm, "ed25519");
    assert_eq!(verify.key_id, "key-1");
}
