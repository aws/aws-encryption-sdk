// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Runnable entry point for the Dafny-Rust Language_Server.
//!
//! Binds an rpcv2Cbor HTTP endpoint on a port taken from (in order) the first
//! CLI argument, the `ESDK_TESTSERVER_PORT` env var, or the default 8098.

use std::net::SocketAddr;
use std::sync::Arc;

use aws_esdk_dafny_test_server::handlers::AppState;
use aws_esdk_dafny_test_server::wire::app;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("ESDK_TESTSERVER_PORT").ok())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8098);

    let state = Arc::new(AppState::new());
    let router = app(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    eprintln!("listening at http://127.0.0.1:{port}");
    axum::serve(listener, router).await.expect("server error");
}
