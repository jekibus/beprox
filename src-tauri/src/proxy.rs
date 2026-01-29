use axum::{
    body::Body,
    extract::{Request, State},
    response::Response,
    Router,
};
use http_body_util::BodyExt;
use reqwest::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::store::StoreState;

#[derive(Clone)]
struct AppState {
    client: Client,
    store: StoreState,
}

pub fn start_proxy(store: StoreState) {
    tauri::async_runtime::spawn(async move {
        // Disable redirects - the proxy should pass 3xx responses back to the client
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let state = AppState { client, store };

        let app = Router::new().fallback(handler).with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));
        println!("Proxy listening on {}", addr);

        match TcpListener::bind(addr).await {
            Ok(listener) => {
                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("Server error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to bind port 80: {}", e);
                eprintln!("Ensure you run the app with sudo or Administrator privileges to bind port 80.");
            }
        }
    });
}

async fn handler(State(state): State<AppState>, req: Request) -> Response {
    let host = req.headers().get("host")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.split(':').next().unwrap_or(h)) // remove port if present
        .unwrap_or("unknown");

    let target_port = if let Some(site) = state.store.get_site_by_domain(host) {
        if !site.enabled {
             return Response::builder().status(503).body(Body::from("Site is disabled")).unwrap();
        }
        site.port
    } else {
         return Response::builder().status(404).body(Body::from(format!("Site {} not found", host))).unwrap();
    };

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    println!("Incoming Request: {} {} for host {}", req.method(), path_query, host);

    let target_url = format!("http://localhost:{}{}", target_port, path_query);
    println!("Proxying to: {}", target_url);

    // Filter out hop-by-hop headers
    let (parts, body) = req.into_parts();
    let mut request_builder = state.client.request(parts.method.clone(), &target_url);

    for (key, value) in parts.headers.iter() {
        // Skip headers that are hop-by-hop or problematic for proxying
        if key.as_str() != "host" 
           && key.as_str() != "connection" 
           && key.as_str() != "transfer-encoding" 
           && key.as_str() != "upgrade" {
            request_builder = request_builder.header(key, value);
        }
    }
    // Add Host header manually to match target
    request_builder = request_builder.header("Host", format!("localhost:{}", target_port));

    // Convert body to bytes (simplest for MVP)
    // In production, use streaming
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => {
            eprintln!("Failed to read body: {}", e);
            return Response::builder().status(500).body(Body::from("Body error")).unwrap();
        }
    };
    
    request_builder = request_builder.body(bytes);

    match request_builder.send().await {
        Ok(resp) => {
            let status = resp.status();
            println!("Response Status: {}", status);
            let headers = resp.headers().clone();
            
            // Convert stream to axum Body
            let stream = resp.bytes_stream();
            let body = Body::from_stream(stream);

            let mut response_builder = Response::builder().status(status);
            for (key, value) in headers.iter() {
                response_builder = response_builder.header(key, value);
            }

            response_builder
                .body(body)
                .unwrap_or_else(|_| Response::new(Body::from("Error creating response")))
        }
        Err(e) => {
            eprintln!("Request Failed: {}", e);
            Response::builder()
                .status(502)
                .body(Body::from(format!("Proxy Error: {}", e)))
                .unwrap()
        },
    }
}
