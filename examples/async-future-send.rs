use async_trait::async_trait;
use must_implement_trait::must_implement_trait;
use std::{rc::Rc, sync::Arc};

#[async_trait]
trait ResourceManager {
    async fn update_resources(resources: &Resources);
}

// TODO: comment out this attribute while using an Rc!
#[must_implement_trait(Send)]
struct Resources {
    // TODO: change this to an Rc; what happens?
    data: Arc<String>,
}

struct MyResourceManager {}

#[async_trait]
impl ResourceManager for MyResourceManager {
    async fn update_resources(resources: &Resources) {
        // Some code to update resources...
    }
}

fn main() {}
