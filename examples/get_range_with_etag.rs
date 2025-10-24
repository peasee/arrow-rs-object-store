#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use object_store::{local::LocalFileSystem, path::Path, GetOptions, ObjectStore};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use tempfile::tempdir;

fn main() {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        println!("This example is not supported on wasm32-unknown.");
        return;
    }

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        use tokio::runtime::Runtime;

        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let tmp = tempdir().unwrap();
            let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
            let location = Path::from("example.txt");
            let content = b"Hello, Object Store!";

            // Put the object into the store
            store
                .put(&location, content.as_ref().into())
                .await
                .expect("Failed to put object");

            // Get the object from the store to figure out the right etag
            let result: object_store::GetResult = store.get(&location).await.expect("Failed to get object");

            let etag = result.meta.e_tag.expect("ETag should be present");

            // Get the object from the store with range and etag
            let bytes = store
                .get_opts(
                    &location,
                    GetOptions::new()
                        .with_range(0..5)
                        .with_if_match(etag.clone()),
                )
                .await
                .expect("Failed to get object with range and etag")
                .bytes()
                .await
                .expect("Failed to read bytes");

            println!(
                "Retrieved range [0-5] with ETag {}: {}",
                etag,
                String::from_utf8_lossy(&bytes)
            );

            // Show that if the etag does not match, we get an error
            let wrong_etag = "wrong-etag".to_string();
            match store
                .get_opts(
                    &location,
                    GetOptions::new().with_range(0..5).with_if_match(wrong_etag)
                )
                .await
            {
                Ok(_) => println!("Unexpectedly succeeded with wrong ETag"),
                Err(e) => println!("On a non-versioned object store, getting an invalid ETag ('wrong-etag') results in an error as expected: {}", e),
            }
        });
    }
}
