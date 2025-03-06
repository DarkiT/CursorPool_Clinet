use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use std::env;

// 共享的 HTTP 客户端
#[derive(Clone)]
pub struct ApiClient(pub(crate) Arc<Client>);

impl Default for ApiClient {
    fn default() -> Self {
        Self(Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        ))
    }
}

// 从环境变量获取基础 URL
// pub fn get_base_url() -> String {
//     "https://cursor.v2.wvw.ink/api".to_string()
// }

// 从环境变量获取基础 URL（带默认值）
pub fn get_base_url() -> String {
    env::var("BASE_URL")  // 读取环境变量
        .unwrap_or_else(|_| "https://cursor.v2.wvw.ink/api".into()) // 设置默认值
}

// 或者强制要求必须设置环境变量（推荐生产环境使用）
// pub fn get_base_url() -> String {
//     env::var("BASE_URL")
//         .expect("BASE_URL environment variable must be set")
// }