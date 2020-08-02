use must_implement_trait::must_implement_trait;

fn main() {
    let _instance = MySendSyncStruct { value: 7 };
}

#[must_implement_trait(Send, Sync)]
struct MySendSyncStruct {
    value: i32
}