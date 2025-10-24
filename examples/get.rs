use object_store::{local::LocalFileSystem, path::Path, ObjectStore};
use tempfile::tempdir;

#[tokio::main]
async fn main() {
    let tmp = tempdir().unwrap();
    let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
    let location = Path::from("example.txt");
    let content = b"Hello, Object Store!";

    // Put the object into the store
    store
        .put(&location, content.as_ref().into())
        .await
        .expect("Failed to put object");

    // Get the object from the store
    let get_result = store.get(&location).await.expect("Failed to get object");
    let bytes = get_result.bytes().await.expect("Failed to read bytes");
    println!("Retrieved content: {}", String::from_utf8_lossy(&bytes));
}
