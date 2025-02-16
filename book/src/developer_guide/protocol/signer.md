# Signer

The signer is a component of the protocol router that is responsible for creating signed URLs to the data files that contain the relevant table data. The signer is responsible for ensuring that the client has the necessary permissions to access the data files and that the URLs are only valid for a limited time.

## How is signing handled?

The signer is defined by the following trait:

```rust
trait Signer: Send + Sync {
   fn sign(&self, uri: &str, expires_in: Duration) -> Result<SignedUrl, SignerError>;
}
```

Implementing this type allows users to customize the signing process to their needs. The `sign` method takes a URI which is typically cloud specfic (e.g. `s3://my-data-bucket/my-table/part1-0000.snappy.parquet`) and a `Duration` for how long the signed URL should be valid. The signer should return a `SignedUrl` that contains the signed URL and the expiration time.

### Example

// TODO: create good example

### Configuring multiple signers

It is possible that tables that are shared using Delta Sharing are stored in different cloud storage services. In this case, the Delta Sharing server can be configured with multiple signers, each responsible for signing URLs for a specific cloud storage service. To make sure that the correct signer is used, one could implement a simple registry and use it to look up the correct signer based on the URI.

```rust
struct SignerRegistry {
    HashMap<String, Box<dyn Signer>>,
}

impl SignerRegistry {
    fn new() -> Self {
        let s3_signer = todo!();
        let gcs_signer = todo!();

        let mut registry = HashMap::new();
        registry.insert("s3".to_string(), Box::new(s3_signer));
        registry.insert("gs".to_string(), Box::new(gcs_signer));
        Self { registry }
    }

    fn get_signer(&self, uri: &str) -> Option<&Box<dyn Signer>> {
        // logic to determine which signer to use
        todo!()
    }
}

impl Signer for SignerRegistry {
    fn sign(&self, uri: &str, expires_in: Duration) -> Result<SignedUrl, SignerError> {
        let signer = self.get_signer(uri).unwrap();
        signer.sign(uri, expires_in)
    }
}
```

## What's in the box?

The Delta Sharing library comes with pre-built signers for common cloud storage services like S3, GCS, and Azure Blob Storage. These signers are implemented using the `Signer` trait and can be direcly used in the Delta Sharing server configuration.
