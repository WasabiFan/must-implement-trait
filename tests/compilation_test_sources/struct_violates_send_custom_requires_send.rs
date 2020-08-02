use must_implement_trait::must_implement_trait;

fn main() {
    let _instance = MySendStruct { value: 7 };
}

trait NotSend {}

#[must_implement_trait(Send)]
struct MySendStruct {
    value: Box<dyn NotSend>
}