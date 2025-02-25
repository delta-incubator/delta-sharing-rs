use std::sync::Arc;

use self::credential::AwsCredential;
use crate::util::STRICT_ENCODE_SET;
use crate::CredentialProvider;

mod credential;

/// This struct is used to maintain the URI path encoding
const STRICT_PATH_ENCODE_SET: percent_encoding::AsciiSet = STRICT_ENCODE_SET.remove(b'/');

/// [`CredentialProvider`] for [`AmazonS3`]
pub type AwsCredentialProvider = Arc<dyn CredentialProvider<Credential = AwsCredential>>;
