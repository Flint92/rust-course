use crate::cli::http_opts::HttpServeOpts;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode, header::CONTENT_TYPE};
use axum::routing::get;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

struct HttpServerState {
    path: PathBuf,
}

pub async fn serve(opts: HttpServeOpts) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], opts.port));
    let state = HttpServerState { path: opts.dir };

    let router = Router::new()
        .route("/", get(root_path_handler))
        .route("/{*path}", get(path_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server staring at :{}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}

async fn root_path_handler(
    State(state): State<Arc<HttpServerState>>,
) -> (StatusCode, HeaderMap, String) {
    path_handler(State(state), Path(String::from(""))).await
}

async fn path_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    info!("Path: {}", path);
    let full_path = state.path.join(&path);
    info!("{:?}", full_path);

    if !full_path.exists() {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
        return (
            StatusCode::NOT_FOUND,
            headers,
            "<h1>Not found</h1>".to_string(),
        );
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());

    // 检查路径是否为目录
    if full_path.is_dir() {
        match tokio::fs::read_dir(&full_path).await {
            Ok(dir_entries) => {
                let mut html_content = String::new();
                html_content.push_str(&format!(
                    "<html><head><title>Directory listing for {}</title></head><body>",
                    path
                ));
                html_content.push_str(&format!("<h1>Directory listing for {}</h1>", path));
                html_content.push_str("<ul>");

                // 遍历目录条目
                let mut entries = Vec::new();
                tokio::pin!(dir_entries);
                while let Some(entry) = dir_entries.next_entry().await.transpose() {
                    match entry {
                        Ok(entry) => {
                            let entry_name = entry.file_name().to_string_lossy().to_string();
                            let is_dir = entry.path().is_dir();
                            entries.push((entry_name, is_dir));
                        }
                        Err(err) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                headers,
                                format!("Failed to read directory entry: {}", err),
                            );
                        }
                    }
                }

                // 按名称排序并生成链接
                entries.sort_by(|a, b| a.0.cmp(&b.0));
                for (entry_name, is_dir) in entries {
                    let link_path = if path.is_empty() {
                        entry_name.clone()
                    } else if path.ends_with('/') {
                        format!("{}{}", path, entry_name)
                    } else {
                        format!("{}/{}", path, entry_name)
                    };

                    // 目录需要在路径末尾添加斜杠
                    let display_name = if is_dir && !entry_name.ends_with('/') {
                        format!("{}/", entry_name)
                    } else {
                        entry_name.clone()
                    };

                    html_content.push_str(&format!(
                        "<li><a href='/{link_path}'>{display_name}</a></li>",
                        link_path = link_path,
                        display_name = display_name
                    ));
                }

                html_content.push_str("</ul></body></html>");
                (StatusCode::OK, headers, html_content)
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                format!("Failed to read directory: {}", err),
            ),
        }
    } else {
        // 处理文件路径
        match tokio::fs::read_to_string(&full_path).await {
            Ok(content) => {
                // 设置适合的内容类型
                if let Some(ext) = full_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    match ext_str.as_str() {
                        "html" | "htm" => headers
                            .insert(CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap()),
                        "css" => headers.insert(CONTENT_TYPE, "text/css".parse().unwrap()),
                        "js" => {
                            headers.insert(CONTENT_TYPE, "application/javascript".parse().unwrap())
                        }
                        "txt" => headers
                            .insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap()),
                        "json" => headers.insert(CONTENT_TYPE, "application/json".parse().unwrap()),
                        _ => headers
                            .insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap()),
                    };
                }
                (StatusCode::OK, headers, content)
            }
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                "Internal server error".to_string(),
            ),
        }
    }
}
