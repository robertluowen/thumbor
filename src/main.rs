use axum::{extract::Path, http::StatusCode, routing::get, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;
// 引入protobuf生成的代码
mod pb;

use pb::*;

// 序列化数据
#[warn(dead_code)]
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化一个使用格式化输出的追踪订阅者
    // 1.创建一个格式化订阅者：这个订阅者会将追踪事件格式化为文本，并输出到标准输出（通常是终端或控制台）。
    // 2.全局设置：通过调用 init() 方法，这个订阅者被设置为全局的默认订阅者。这意味着，任何使用 tracing 库生成的追踪事件都将被这个订阅者捕获并处理。
    // 3.配置：虽然 tracing_subscriber::fmt::init(); 使用了一些默认配置，但 tracing_subscriber::fmt 模块还提供了更多的配置选项，允许你自定义输出的格式、级别、过滤规则等。
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/image/:spec/:url", get(generate));

    // 运行web服务器
    let addr: String = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 解析参数
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
