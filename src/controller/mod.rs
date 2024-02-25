use axum::Router;

use crate::{
    error::Result,
    model::ModelManager,
    view::{self, notfound::handler_404},
};

pub struct Controller {
    pub view: axum::Router,
    pub data: axum::Router,
}

impl Controller {
    pub async fn new() -> Result<Self> {
        let mc = ModelManager::new().await?;

        Ok(Self {
            view: view::routes(mc.clone()),
            data: Router::new().fallback(handler_404),
        })
    }

    pub async fn get_routes(self) -> Result<axum::Router> {
        Ok(axum::Router::new()
            .nest("/", self.view)
            .nest("/api", self.data)
            .fallback(handler_404))
    }
}
