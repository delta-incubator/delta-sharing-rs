use anyhow::Context;
use anyhow::Result;
use async_session::async_trait;
use async_session::serde_json;
use async_session::Result as AsyncResult;
use async_session::Session;
use async_session::SessionStore;
use redis::aio::Connection;
use redis::AsyncCommands;
pub use redis::Client;
use redis::IntoConnectionInfo;
use redis::RedisResult;
use tracing::info;

pub fn connect(url: &str) -> Result<Client> {
    info!("connecting to kvs");
    let client = Client::open(url).context("failed to acquire redis connection")?;
    info!("connected to kvs");
    Ok(client)
}

#[derive(Clone, Debug)]
pub struct RedisSessionStore {
    client: Client,
    prefix: Option<String>,
}

impl RedisSessionStore {
    pub fn from_client(client: Client) -> Self {
        Self {
            client,
            prefix: None,
        }
    }

    pub fn new(info: impl IntoConnectionInfo) -> RedisResult<Self> {
        Ok(Self::from_client(Client::open(info)?))
    }

    pub fn with_prefix(mut self, prefix: impl AsRef<str>) -> Self {
        self.prefix = Some(prefix.as_ref().to_owned());
        self
    }

    async fn ids(&self) -> AsyncResult<Vec<String>> {
        Ok(self.connection().await?.keys(self.prefix_key("*")).await?)
    }

    pub async fn count(&self) -> AsyncResult<usize> {
        if self.prefix.is_none() {
            let mut connection = self.connection().await?;
            Ok(redis::cmd("DBSIZE").query_async(&mut connection).await?)
        } else {
            Ok(self.ids().await?.len())
        }
    }

    fn prefix_key(&self, key: impl AsRef<str>) -> String {
        if let Some(ref prefix) = self.prefix {
            format!("{}{}", prefix, key.as_ref())
        } else {
            key.as_ref().into()
        }
    }

    async fn connection(&self) -> RedisResult<Connection> {
        self.client.get_async_std_connection().await
    }

    #[cfg(test)]
    async fn ttl_for_session(&self, session: &Session) -> AsyncResult<usize> {
        Ok(self
            .connection()
            .await?
            .ttl(self.prefix_key(session.id()))
            .await?)
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load_session(&self, cookie: String) -> AsyncResult<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie)?;
        let mut connection = self.connection().await?;
        let record: Option<String> = connection.get(self.prefix_key(id)).await?;
        match record {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(None),
        }
    }

    async fn store_session(&self, session: Session) -> AsyncResult<Option<String>> {
        let id = self.prefix_key(session.id());
        let string = serde_json::to_string(&session)?;
        let mut connection = self.connection().await?;
        match session.expires_in() {
            None => connection.set(id, string).await?,
            Some(expiry) => {
                connection
                    .set_ex(id, string, expiry.as_secs() as usize)
                    .await?
            }
        };
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> AsyncResult<()> {
        let mut connection = self.connection().await?;
        let key = self.prefix_key(session.id().to_string());
        connection.del(key).await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result<()> {
        let mut connection = self.connection().await?;
        if self.prefix.is_none() {
            let _: () = redis::cmd("FLUSHDB").query_async(&mut connection).await?;
        } else {
            let ids = self.ids().await?;
            if !ids.is_empty() {
                connection.del(ids).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use async_std::task;
    use std::time::Duration;
    use testcontainers::clients;
    use testcontainers::images::redis;

    #[derive(sqlx::FromRow)]
    struct Table {
        pub tablename: String,
    }

    async fn create_store() -> Result<RedisSessionStore> {
        dotenv::dotenv().ok();
        let docker = clients::Cli::default();
        docker.run(redis::Redis::default());
        let url = "redis://127.0.0.1:6379";
        let client = connect(&url).context("failed to create redis client")?;
        let store = RedisSessionStore::from_client(client);
        store
            .clear_store()
            .await
            .context("failed to clear redis session store")?;
        Ok(store)
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_session_with_no_expiry() {
        let store = create_store().await.expect("store should be created");
        let mut session = Session::new();
        let key = testutils::rand::string(10);
        let value = testutils::rand::string(10);
        session
            .insert(key.as_str(), value.as_str())
            .expect("should insert key/value properly");
        let cloned = session.clone();
        let cookie = store
            .store_session(session)
            .await
            .expect("should store session properly");
        assert!(matches!(cookie, Some(_)));
        let cookie = cookie.unwrap();
        let loaded = store
            .load_session(cookie)
            .await
            .expect("should load session properly");
        assert!(matches!(loaded, Some(_)));
        let loaded = loaded.unwrap();
        assert_eq!(cloned.id(), loaded.id());
        assert_eq!(value.as_str(), &loaded.get::<String>(key.as_str()).unwrap());
        assert!(!loaded.is_expired());
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_session() {
        let store = create_store().await.expect("store should be created");
        let mut session = Session::new();
        let key = testutils::rand::string(10);
        let value = testutils::rand::string(10);
        let other = testutils::rand::string(10);
        session
            .insert(key.as_str(), value.as_str())
            .expect("should insert key/value properly");
        let cookie = store
            .store_session(session)
            .await
            .expect("should store session properly");
        assert!(matches!(cookie, Some(_)));
        let cookie = cookie.unwrap();
        let session = store
            .load_session(cookie.clone())
            .await
            .expect("should load session properly");
        assert!(matches!(session, Some(_)));
        let mut session = session.unwrap();
        session
            .insert(key.as_str(), other.as_str())
            .expect("should insert key/value properly");
        let none = store
            .store_session(session)
            .await
            .expect("should store session properly");
        assert_eq!(none, None);
        let session = store
            .load_session(cookie.clone())
            .await
            .expect("should load session properly");
        assert!(matches!(session, Some(_)));
        let session = session.unwrap();
        assert_eq!(
            &session.get::<String>(key.as_str()).unwrap(),
            other.as_str()
        );
        assert_eq!(store.count().await.unwrap(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_session_with_extending_expiry() {
        let store = create_store().await.expect("store should be created");
        let mut session = Session::new();
        session.expire_in(Duration::from_secs(5));
        let original = session.expiry();
        assert!(matches!(original, Some(_)));
        let original = original.unwrap().clone();
        let cookie = store
            .store_session(session)
            .await
            .expect("should store session properly");
        assert!(matches!(cookie, Some(_)));
        let cookie = cookie.unwrap();

        let session = store
            .load_session(cookie.clone())
            .await
            .expect("should load session properly");
        assert!(matches!(session, Some(_)));
        let mut session = session.unwrap();
        let ttl = store.ttl_for_session(&session).await;
        assert!(matches!(ttl, Ok(_)));
        let ttl = ttl.unwrap();
        assert!(ttl > 3 && ttl < 5);
        let expiry = session.expiry();
        assert!(matches!(expiry, Some(_)));
        let expiry = expiry.unwrap();
        assert_eq!(expiry, &original);
        session.expire_in(Duration::from_secs(10));
        let expiry = session.expiry();
        assert!(matches!(expiry, Some(_)));
        store
            .store_session(session)
            .await
            .expect("should store session properly");
        let session = store
            .load_session(cookie.clone())
            .await
            .expect("should load session properly");
        assert!(matches!(session, Some(_)));
        let session = session.unwrap();
        let ttl = store.ttl_for_session(&session).await;
        assert!(matches!(ttl, Ok(_)));
        let ttl = ttl.unwrap();
        assert!(ttl > 8 && ttl < 10);
        assert_eq!(store.count().await.unwrap(), 1);
        task::sleep(Duration::from_secs(10)).await;
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_destroy_single_session() {
        let store = create_store().await.expect("store should be created");
        let attempt = testutils::rand::usize(10);
        for _ in 0..attempt {
            store
                .store_session(Session::new())
                .await
                .expect("should store session properly");
        }
        let cookie = store
            .store_session(Session::new())
            .await
            .expect("should store session properly");
        assert!(matches!(cookie, Some(_)));
        let cookie = cookie.unwrap();
        assert_eq!(store.count().await.unwrap(), attempt + 1);
        let session = store
            .load_session(cookie.clone())
            .await
            .expect("should load session properly");
        assert!(matches!(session, Some(_)));
        let session = session.unwrap();
        store
            .destroy_session(session.clone())
            .await
            .expect("should store session properly");
        let none = store
            .load_session(cookie)
            .await
            .expect("should store session properly");
        assert_eq!(none, None);
        assert_eq!(store.count().await.unwrap(), attempt);
        assert!(store.destroy_session(session).await.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_clear_whole_store() {
        let store = create_store().await.expect("store should be created");
        let attempt = testutils::rand::usize(10);
        for _ in 0..attempt {
            store
                .store_session(Session::new())
                .await
                .expect("should store session properly");
        }
        assert_eq!(store.count().await.unwrap(), attempt);
        store.clear_store().await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }
}
