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
    let bytes = store
        .get_range(&location, 0..5)
        .await
        .expect("Failed to get object");
    println!("Retrieved range [0-5]: {}", String::from_utf8_lossy(&bytes));
}