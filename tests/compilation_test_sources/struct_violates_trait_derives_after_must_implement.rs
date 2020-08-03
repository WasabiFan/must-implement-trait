use must_implement_trait::must_implement_trait;
use std::rc::Rc;

fn main() {
    let instance = MySendStruct { value: 7 };
    let _other = instance.clone();
}

#[must_implement_trait(Send)]
#[derive(Clone)]
struct MySendStruct {
    value: Rc<i32>
}