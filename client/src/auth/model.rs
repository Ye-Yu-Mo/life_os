use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMode {
    Login,
    Register,
}

impl AuthMode {
    pub fn endpoint(&self) -> &'static str {
        match self {
            Self::Login => "/login",
            Self::Register => "/register",
        }
    }

    pub fn button_text(&self) -> &'static str {
        match self {
            Self::Login => "登录",
            Self::Register => "注册",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Login => Self::Register,
            Self::Register => Self::Login,
        }
    }

    pub fn toggle_text(&self) -> &'static str {
        match self {
            Self::Login => "还没有账号？去注册",
            Self::Register => "已有账号？去登录",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum AuthError {
    Network(String),
    Server(String),
    InvalidResponse,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "网络错误: {}", msg),
            Self::Server(msg) => write!(f, "{}", msg),
            Self::InvalidResponse => write!(f, "服务器响应异常"),
        }
    }
}

impl std::error::Error for AuthError {}
