use must_implement_trait::must_implement_trait;

fn main() {
    let _instance = MySendStruct { value: 7 };
}

#[must_implement_trait(Send)]
struct MySendStruct<T: Send + Sync> {
    value: T
}
