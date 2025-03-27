
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    code: u16,
    message: String,
    data: Option<T>,
    error: Option<String>,
}
impl <T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        
        Self {
            code: 200,
            message: "OK".to_string(),
            data: Some(data),
            error: None,
        }
    }
    pub fn error(code: u16, message: &str, error: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
            error: Some(error.to_string()),
        }
    }
    
}
// 实现 From<Result<T, String>>
impl<T> From<Result<T, String>> for ApiResponse<T> {
    fn from(result: Result<T, String>) -> Self {
        match result {
            Ok(data) => ApiResponse::success(data),
            Err(e) => {
                // 根据错误内容细化状态码（示例逻辑）
                let (code, msg) = if e.contains("connection") {
                    (503, "Service Unavailable")
                } else {
                    (500, "Internal Server Error")
                };
                ApiResponse::error(code, msg, &e)
            }
        }
    }
}