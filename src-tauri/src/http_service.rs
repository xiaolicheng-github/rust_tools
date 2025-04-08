use std::sync::Arc;
use axum::{Router, routing::get, response::Html};
use std::net::SocketAddr;
use tokio::sync::oneshot::Sender;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

pub struct HttpService {
    port: u16,
    shutdown_tx: Option<Sender<()>>,
}

impl HttpService {
    pub fn new(port: u16) -> Self {
        HttpService {
            port,
            shutdown_tx: None,
        }
    }

    pub async fn start(&mut self) -> String {
        // 使用 rust-embed 获取文件内容
        let html_content = match Asset::get("index.html") {
            Some(file) => {
                match std::str::from_utf8(&file.data) {
                    Ok(content) => content.to_string(),
                    Err(e) => {
                        eprintln!("文件解码失败: {}", e);
                        return format!("HTTP服务启动失败: 无法解码模板文件 ({})", e);
                    }
                }
            },
            None => {
                eprintln!("文件加载失败: 未找到 index.html");
                return format!("HTTP服务启动失败: 未找到模板文件");
            }
        }.replace("<p id=\"port-info\">", &format!("<p id=\"port-info\">端口: {}", self.port));
        
        if self.shutdown_tx.is_some() {
            return format!("HTTP服务已在端口 {} 运行", self.port);
        }

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);
        
        // let port = self.port;
        // let html_content = fs::read_to_string("static/index.html")
        //     .expect("无法读取HTML文件")
        //     .replace("<p id=\"port-info\">当前服务运行正常，端口: </p>", 
        //         &format!("<p id=\"port-info\">当前服务运行正常，端口: {}</p>", port));

        let html_content = Arc::new(html_content);
        let app = Router::new().route("/", get(move || {
            let html = html_content.clone();
            async move { Html(html.to_string()) }
        }));

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        let server = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            });

        tokio::spawn(async move {
            server.await.expect("HTTP服务器运行失败");
        });

        format!("HTTP服务已启动，监听端口: {}", self.port)
    }

    pub async fn stop(&mut self) -> String {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            shutdown_tx.send(()).map_err(|_| "发送关闭信号失败").ok();
            "HTTP服务正在停止...".into()
        } else {
            "服务未运行".into()
        }
    }
}