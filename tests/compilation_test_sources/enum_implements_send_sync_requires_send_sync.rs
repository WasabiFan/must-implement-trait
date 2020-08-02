use must_implement_trait::must_implement_trait;

fn main() {
    let _instance_one = MySendSyncEnum::VariantOne(7);
    let _instance_two = MySendSyncEnum::VariantTwo { foo: 13, bar: String::from("hello") };
}

#[must_implement_trait(Send, Sync)]
enum MySendSyncEnum {
    VariantOne(i32),
    VariantTwo { foo: i32, bar: String }
}