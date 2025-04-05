use std::sync::Arc;
use axum::{Router, routing::get, response::Html};
use std::{net::SocketAddr, fs};
use tokio::sync::oneshot::Sender;  // 移除标准库的mpsc导入，保留tokio版本

// 移除 Clone 自动派生
pub struct HttpService {
    port: u16,
    shutdown_tx: Option<Sender<()>>,
}

// 手动实现 Clone
impl Clone for HttpService {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            shutdown_tx: None, // 明确不克隆通道
        }
    }
}

// 明确实现Send保证线程安全
unsafe impl Send for HttpService {}

impl HttpService {
    pub fn new(port: u16) -> Self {
        HttpService {
            port,
            shutdown_tx: None,
        }
    }

    pub async fn start(&mut self) -> String {
        // 修正资源路径获取逻辑
        let resource_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            // .join("src-tauri")
            .join("static\\index.html");

        // 添加路径调试日志
        println!("尝试加载模板路径: {}", resource_path.display());
        
        let html_content = fs::read_to_string(&resource_path)
            .unwrap_or_else(|e| {
                eprintln!("文件加载失败: {} ({})", resource_path.display(), e);
                format!(
                    "<p id='port-info'>服务运行中（端口: {}）</p>",
                    self.port
                )
            })
            .replace("<p id=\"port-info\">", &format!("<p id=\"port-info\">端口: {}", self.port));
        
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

        // 修复服务器启动逻辑（移除重复的axum::serve调用）
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