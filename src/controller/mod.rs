use axum::Router;

use crate::{
    error::Result,
    model::ModelManager,
    view::{self, notfound::handler_404},
};

#[derive(Debug)]
pub struct Controller {
    pub view: axum::Router,
    pub data: axum::Router,
}

impl Controller {
    pub fn new(mc: ModelManager) -> Self {
        Self {
            view: view::routes(mc.clone()),
            data: Router::new().fallback(handler_404),
        }
    }

    pub async fn run_server(self) -> Result<()> {
        let router = Router::new()
            .nest("/", self.view)
            .nest("/api", self.data)
            .fallback(handler_404);

        let port = 8000_u16;

        tracing::info!("router initialized, now listening on port {}", port);

        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
            .await
            .unwrap();

        axum::serve(listener, router).await.unwrap();

        Ok(())
    }
}
