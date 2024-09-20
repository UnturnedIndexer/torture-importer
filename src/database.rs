use surrealdb::{
    engine::any::{self, Any},
    opt::auth::Root,
    Surreal,
};

pub struct Database;

pub struct ConnectionOptions<'a> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub credentials: Root<'a>,
}

impl Database {
    pub async fn connect<'a>(
        endpoint: &'a str,
        options: &ConnectionOptions<'a>,
    ) -> anyhow::Result<Surreal<Any>> {
        let db = any::connect(endpoint).await?;

        db.signin(options.credentials).await?;

        db.use_ns(options.namespace)
            .use_db(options.database)
            .await?;

        Ok(db)
    }
}
