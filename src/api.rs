use std::time::Duration;

use reqwest::{header, Client, Method, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    config::{AbbotikConfig, OutputFormat},
    error::{AbbotikError, ServerErrorEnvelope},
};

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: url::Url,
    token: Option<String>,
    output_format: OutputFormat,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiEnvelope<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub tenant: Option<String>,
    pub tenant_id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterRequest {
    pub tenant: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefreshRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub tenant: String,
    pub tenant_id: String,
    #[serde(default)]
    pub database: Option<String>,
    pub access: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginData {
    pub token: String,
    pub user: AuthUser,
    #[serde(default)]
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterData {
    pub tenant_id: String,
    pub tenant: String,
    pub username: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefreshData {
    pub token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvisionRequest {
    pub tenant: Option<String>,
    pub username: Option<String>,
    pub public_key: Option<String>,
    pub algorithm: Option<String>,
    pub key_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvisionUserData {
    pub id: String,
    pub username: String,
    pub access: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvisionKeyData {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    pub algorithm: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvisionChallengeData {
    pub challenge_id: String,
    pub nonce: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvisionData {
    pub tenant: String,
    pub tenant_id: String,
    pub user: ProvisionUserData,
    pub key: ProvisionKeyData,
    pub challenge: ProvisionChallengeData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChallengeRequest {
    pub tenant: Option<String>,
    pub key_id: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChallengeData {
    pub challenge_id: String,
    pub nonce: String,
    pub issued_at: String,
    pub expires_in: u64,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyRequest {
    pub tenant: Option<String>,
    pub challenge_id: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyData {
    pub token: String,
    pub expires_in: u64,
    pub tenant: String,
    pub tenant_id: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DissolveRequest {
    pub tenant: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DissolveData {
    pub confirmation_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DissolveConfirmRequest {
    pub confirmation_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DissolveConfirmData {
    pub tenant: String,
    pub username: String,
    pub dissolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SudoData {
    pub sudo_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub access_level: String,
    pub is_sudo: bool,
    pub warning: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenantSummary {
    pub name: String,
    pub description: Option<String>,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    pub status: String,
    pub timestamp: String,
    pub uptime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRequest {
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

impl ApiClient {
    pub fn new(config: AbbotikConfig) -> Result<Self, AbbotikError> {
        let base_url = config.base_url()?;
        Ok(Self::from_parts(
            base_url,
            config.token,
            config.output_format,
        ))
    }

    pub fn from_parts(
        base_url: url::Url,
        token: Option<String>,
        output_format: OutputFormat,
    ) -> Self {
        Self {
            base_url,
            token,
            output_format,
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("reqwest client should build"),
        }
    }

    pub fn base_url(&self) -> &url::Url {
        &self.base_url
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn with_output_format(mut self, output_format: OutputFormat) -> Self {
        self.output_format = output_format;
        self
    }

    pub fn endpoint(&self, path: &str) -> Result<url::Url, AbbotikError> {
        self.base_url
            .join(path)
            .map_err(|_| AbbotikError::InvalidBaseUrl(path.to_string()))
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, AbbotikError> {
        self.request_json(Method::GET, path, Option::<&()>::None)
            .await
    }

    pub async fn get_json_with_query<Q, T>(&self, path: &str, query: &Q) -> Result<T, AbbotikError>
    where
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(Method::GET, path, Some(query), Option::<&()>::None)
            .await
    }

    pub async fn get_text(&self, path: &str) -> Result<String, AbbotikError> {
        self.request_text(Method::GET, path, Option::<&()>::None)
            .await
    }

    pub async fn delete_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, AbbotikError> {
        self.request_json(Method::DELETE, path, Option::<&()>::None)
            .await
    }

    pub async fn delete_json_with_query<Q, T>(&self, path: &str, query: &Q) -> Result<T, AbbotikError>
    where
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(Method::DELETE, path, Some(query), Option::<&()>::None)
            .await
    }

    pub async fn post_json<B, T>(&self, path: &str, body: &B) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json(Method::POST, path, Some(body)).await
    }

    pub async fn post_json_without_auth<B, T>(&self, path: &str, body: &B) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_without_auth(Method::POST, path, Some(body))
            .await
    }

    pub async fn post_json_with_query<B, Q, T>(
        &self,
        path: &str,
        query: &Q,
        body: &B,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(Method::POST, path, Some(query), Some(body))
            .await
    }

    pub async fn put_json<B, T>(&self, path: &str, body: &B) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json(Method::PUT, path, Some(body)).await
    }

    pub async fn put_json_with_query<B, Q, T>(
        &self,
        path: &str,
        query: &Q,
        body: &B,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(Method::PUT, path, Some(query), Some(body))
            .await
    }

    pub async fn patch_json<B, T>(&self, path: &str, body: &B) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json(Method::PATCH, path, Some(body)).await
    }

    pub async fn patch_json_with_query<B, Q, T>(
        &self,
        path: &str,
        query: &Q,
        body: &B,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(Method::PATCH, path, Some(query), Some(body))
            .await
    }

    pub async fn request_json<B, T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query(method, path, Option::<&()>::None, body)
            .await
    }

    pub async fn request_json_without_auth<B, T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.request_json_with_query_without_auth(method, path, Option::<&()>::None, body)
            .await
    }

    pub async fn request_json_with_query<B, Q, T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = self.endpoint(path)?;
        let request = self.request_builder(method.clone(), url.clone())?;
        let request = if let Some(query) = query {
            request.query(query)
        } else {
            request
        };
        let request = if let Some(body) = body {
            request.json(body)
        } else {
            request
        };

        let response = request.send().await.map_err(|source| AbbotikError::Request {
            method: method.clone(),
            url: url.to_string(),
            source,
        })?;

        self.parse_response(method, url, response).await
    }

    pub async fn request_json_with_query_without_auth<B, Q, T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = self.endpoint(path)?;
        let request = self.request_builder_without_auth(method.clone(), url.clone())?;
        let request = if let Some(query) = query {
            request.query(query)
        } else {
            request
        };
        let request = if let Some(body) = body {
            request.json(body)
        } else {
            request
        };

        let response = request.send().await.map_err(|source| AbbotikError::Request {
            method: method.clone(),
            url: url.to_string(),
            source,
        })?;

        self.parse_response(method, url, response).await
    }

    pub async fn request_text<B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<String, AbbotikError>
    where
        B: Serialize + ?Sized,
    {
        self.request_text_with_query(method, path, Option::<&()>::None, body)
            .await
    }

    pub async fn request_text_with_query<B, Q>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<String, AbbotikError>
    where
        B: Serialize + ?Sized,
        Q: Serialize + ?Sized,
    {
        let url = self.endpoint(path)?;
        let request = self.request_builder(method.clone(), url.clone())?;
        let request = if let Some(query) = query {
            request.query(query)
        } else {
            request
        };
        let request = if let Some(body) = body {
            request.json(body)
        } else {
            request
        };

        let response = request.send().await.map_err(|source| AbbotikError::Request {
            method: method.clone(),
            url: url.to_string(),
            source,
        })?;

        let status = response.status();
        let text = response.text().await.map_err(|source| AbbotikError::Request {
            method: method.clone(),
            url: url.to_string(),
            source,
        })?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(AbbotikError::Http {
                status,
                method,
                url: url.to_string(),
                message: text,
            })
        }
    }

    pub async fn auth_login(
        &self,
        request: &LoginRequest,
    ) -> Result<ApiEnvelope<LoginData>, AbbotikError> {
        self.post_json("/auth/login", request).await
    }

    pub async fn auth_register(
        &self,
        request: &RegisterRequest,
    ) -> Result<ApiEnvelope<RegisterData>, AbbotikError> {
        self.post_json_without_auth("/auth/register", request).await
    }

    pub async fn auth_refresh(
        &self,
        request: &RefreshRequest,
    ) -> Result<ApiEnvelope<RefreshData>, AbbotikError> {
        self.post_json("/auth/refresh", request).await
    }

    pub async fn auth_provision(
        &self,
        request: &ProvisionRequest,
    ) -> Result<ApiEnvelope<ProvisionData>, AbbotikError> {
        self.post_json_without_auth("/auth/provision", request).await
    }

    pub async fn auth_challenge(
        &self,
        request: &ChallengeRequest,
    ) -> Result<ApiEnvelope<ChallengeData>, AbbotikError> {
        self.post_json_without_auth("/auth/challenge", request).await
    }

    pub async fn auth_verify(
        &self,
        request: &VerifyRequest,
    ) -> Result<ApiEnvelope<VerifyData>, AbbotikError> {
        self.post_json_without_auth("/auth/verify", request).await
    }

    pub async fn auth_dissolve(
        &self,
        request: &DissolveRequest,
    ) -> Result<ApiEnvelope<DissolveData>, AbbotikError> {
        self.post_json_without_auth("/auth/dissolve", request).await
    }

    pub async fn auth_dissolve_confirm(
        &self,
        request: &DissolveConfirmRequest,
    ) -> Result<ApiEnvelope<DissolveConfirmData>, AbbotikError> {
        self.post_json_without_auth("/auth/dissolve/confirm", request).await
    }

    pub async fn auth_sudo(
        &self,
        reason: Option<&str>,
    ) -> Result<ApiEnvelope<SudoData>, AbbotikError> {
        #[derive(Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<&'a str>,
        }

        self.post_json("/api/user/sudo", &Body { reason }).await
    }

    pub async fn auth_tenants(&self) -> Result<ApiEnvelope<Vec<TenantSummary>>, AbbotikError> {
        self.get_json("/auth/tenants").await
    }

    pub async fn health(&self) -> Result<ApiEnvelope<HealthData>, AbbotikError> {
        self.get_json("/health").await
    }

    fn request_builder(
        &self,
        method: Method,
        url: url::Url,
    ) -> Result<reqwest::RequestBuilder, AbbotikError> {
        let mut builder = self.client.request(method, url);

        if let Some(token) = &self.token {
            builder = builder.bearer_auth(token);
        }

        builder = builder.header(header::ACCEPT, self.accept_header_value());
        Ok(builder)
    }

    fn request_builder_without_auth(
        &self,
        method: Method,
        url: url::Url,
    ) -> Result<reqwest::RequestBuilder, AbbotikError> {
        let builder = self.client.request(method, url);
        Ok(builder.header(header::ACCEPT, self.accept_header_value()))
    }

    fn accept_header_value(&self) -> &'static str {
        match self.output_format {
            OutputFormat::Json => "application/json",
            OutputFormat::Toon => "application/toon",
            OutputFormat::Yaml => "application/yaml",
        }
    }

    async fn parse_response<T: DeserializeOwned>(
        &self,
        method: Method,
        url: url::Url,
        response: Response,
    ) -> Result<T, AbbotikError> {
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|source| AbbotikError::Request {
                method: method.clone(),
                url: url.to_string(),
                source,
            })?;

        if status.is_success() {
            serde_json::from_slice(&bytes).map_err(|source| AbbotikError::InvalidResponse {
                method,
                url: url.to_string(),
                source,
            })
        } else {
            let message =
                if let Ok(envelope) = serde_json::from_slice::<ServerErrorEnvelope>(&bytes) {
                    envelope.message()
                } else if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    value
                        .get("message")
                        .and_then(|value| value.as_str())
                        .or_else(|| value.get("error").and_then(|value| value.as_str()))
                        .map(str::to_string)
                        .unwrap_or_else(|| String::from_utf8_lossy(&bytes).to_string())
                } else {
                    String::from_utf8_lossy(&bytes).to_string()
                };

            Err(AbbotikError::Http {
                status,
                method,
                url: url.to_string(),
                message,
            })
        }
    }
}

impl From<AbbotikConfig> for ApiClient {
    fn from(config: AbbotikConfig) -> Self {
        Self::new(config).expect("valid AbbotikConfig base URL")
    }
}
