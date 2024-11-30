mod client;
mod error;
pub(crate) mod gen {
    pub mod v1 {
        include!("gen/openfga.v1.rs");
    }
}

pub use client::Client;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut client = Client::connect("http://[::1]:8081".into(), "sharing")
            .await
            .unwrap();
        let schemas = client.list_schemas("roeap").await.unwrap();
        println!("{schemas:?}");
    }
}
