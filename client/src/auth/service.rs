use super::model::{AuthError, AuthPayload, ErrorResponse, User};
use futures::AsyncReadExt;
use gpui_http_client::{AsyncBody, HttpClient, Method, Request};
use std::sync::Arc;

pub struct AuthService {
    client: Arc<dyn HttpClient>,
    base_url: String,
}

impl AuthService {
    pub fn new(client: Arc<dyn HttpClient>, base_url: String) -> Self {
        Self { client, base_url }
    }

    pub async fn authenticate(&self, endpoint: &str, payload: AuthPayload) -> Result<User, AuthError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let body = serde_json::to_vec(&payload)
            .map_err(|e| AuthError::Network(format!("序列化失败: {}", e)))?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(&url)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body))
            .map_err(|e| AuthError::Network(format!("构建请求失败: {}", e)))?;

        let mut response = self
            .client
            .send(request)
            .await
            .map_err(|e| AuthError::Network(format!("请求失败: {}", e)))?;

        let status = response.status();

        let mut body_bytes = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body_bytes)
            .await
            .map_err(|e| AuthError::Network(format!("读取响应失败: {}", e)))?;

        if status.is_success() {
            serde_json::from_slice::<User>(&body_bytes)
                .map_err(|_| AuthError::InvalidResponse)
        } else {
            let error_msg = serde_json::from_slice::<ErrorResponse>(&body_bytes)
                .ok()
                .map(|e| e.error)
                .unwrap_or_else(|| format!("服务器错误 ({})", status.as_u16()));

            Err(AuthError::Server(error_msg))
        }
    }
}
