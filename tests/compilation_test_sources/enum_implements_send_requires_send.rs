use must_implement_trait::must_implement_trait;

fn main() {
    let _instance_one = MySendEnum::VariantOne(7);
    let _instance_two = MySendEnum::VariantTwo { foo: 13, bar: String::from("hello") };
}

#[must_implement_trait(Send)]
enum MySendEnum {
    VariantOne(i32),
    VariantTwo { foo: i32, bar: String }
}