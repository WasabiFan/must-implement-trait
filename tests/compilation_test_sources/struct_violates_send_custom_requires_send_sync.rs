use must_implement_trait::must_implement_trait;

fn main() {
    let _instance = MySendSyncStruct { value: 7 };
}

trait NotSend: Sync {}

#[must_implement_trait(Send, Sync)]
struct MySendSyncStruct {
    value: Box<dyn NotSend>
}