use crate::server::entities::signed_url::SignedUrl;
use anyhow::Result;
use url::Url;

pub trait SignedUrlService {
    fn sign(&self, signed_url: &SignedUrl) -> Result<Url>;
}

pub enum Provider {
    AWS,
    GCP,
}
