use futures::future::BoxFuture;
use futures::FutureExt;
use gpui_http_client::{AsyncBody, HttpClient, Response};
use http::HeaderValue;
use std::sync::Arc;

pub struct SurfClient {
    client: surf::Client,
}

impl SurfClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            client: surf::Client::new(),
        })
    }
}

impl HttpClient for SurfClient {
    fn type_name(&self) -> &'static str {
        "SurfClient"
    }

    fn user_agent(&self) -> Option<&HeaderValue> {
        None
    }

    fn send(
        &self,
        req: http::Request<AsyncBody>,
    ) -> BoxFuture<'static, anyhow::Result<Response<AsyncBody>>> {
        let client = self.client.clone();

        async move {
            let (parts, body) = req.into_parts();

            let method = match parts.method.as_str() {
                "GET" => surf::http::Method::Get,
                "POST" => surf::http::Method::Post,
                "PUT" => surf::http::Method::Put,
                "DELETE" => surf::http::Method::Delete,
                "PATCH" => surf::http::Method::Patch,
                "HEAD" => surf::http::Method::Head,
                "OPTIONS" => surf::http::Method::Options,
                other => anyhow::bail!("Unsupported HTTP method: {}", other),
            };

            let url = surf::Url::parse(&parts.uri.to_string())?;

            let mut surf_req = surf::Request::new(method, url);

            for (name, value) in parts.headers.iter() {
                surf_req.insert_header(name.as_str(), value.to_str()?);
            }

            // 读取 AsyncBody 到 Vec<u8>
            let body_bytes = match body.0 {
                gpui_http_client::Inner::Empty => Vec::new(),
                gpui_http_client::Inner::Bytes(mut cursor) => {
                    use std::io::Read;
                    let mut buf = Vec::new();
                    cursor.read_to_end(&mut buf)?;
                    buf
                }
                gpui_http_client::Inner::AsyncReader(_) => {
                    anyhow::bail!("AsyncReader not supported in SurfClient")
                }
            };

            surf_req.set_body(body_bytes);

            let mut response = client
                .send(surf_req)
                .await
                .map_err(|e| anyhow::anyhow!("Surf request failed: {}", e))?;

            let status = response.status();
            let body_bytes = response
                .body_bytes()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to read response body: {}", e))?;

            let status_code = http::StatusCode::from_u16(status.into())?;
            let mut http_response = Response::builder().status(status_code);

            for (name, values) in response.iter() {
                for value in values.iter() {
                    http_response = http_response.header(name.as_str(), value.as_str());
                }
            }

            let response = http_response.body(AsyncBody::from(body_bytes))?;

            Ok(response)
        }
        .boxed()
    }

    fn proxy(&self) -> Option<&gpui_http_client::Url> {
        None
    }
}
