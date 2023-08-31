use warp::Filter;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use warp::http::Response;
use futures_util::future::Either;

async fn list_directory(path: PathBuf) -> Result<impl warp::Reply, warp::Rejection> {
    let mut response = String::from("<html><body>");

    let base_path = Path::new("./wwwroot");

    fn get_entries(base: &Path, path: &Path) -> String {
        let mut result = String::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let full_path = entry.path();
                    let rel_path = full_path.strip_prefix(base).unwrap_or(&full_path);
                    let display_name = rel_path.to_string_lossy();

                    if full_path.is_dir() {
                        result.push_str(&format!("<a href='{}'>{}/</a><br/>", display_name, display_name));
                    } else {
                        result.push_str(&format!("<a href='{}'>{}</a><br/>", display_name, display_name));
                    }
                }
            }
        }

        result
    }

    response.push_str(&get_entries(&base_path, &path));
    response.push_str("</body></html>");
    
    Ok(Response::builder().body(response))
}

#[tokio::main]
async fn main() {
    let web_dir = warp::path::tail()
    .and_then(|tail: warp::path::Tail| {
        let path = Path::new("./wwwroot").join(tail.as_str());
        if path.is_dir() {
            // Use Either::Left to wrap our future
            Either::Left(list_directory(path))
        } else {
            // Use Either::Right to wrap our immediate rejection into a future
            Either::Right(async { Err(warp::reject()) })
        }
    });

    let files = warp::fs::dir("wwwroot");
    let routes = web_dir.or(files).with(warp::cors().allow_any_origin());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Starting server at http://{}", addr);

    warp::serve(routes).run(addr).await;
}