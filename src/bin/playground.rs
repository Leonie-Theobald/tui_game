trait MyTrait {
    fn value(&self) -> u8;
    fn test_self_default(&self) -> u8 {
        self.value()
    }
}

struct MyStruct {
    value: u8,
}

impl MyTrait for MyStruct {
    fn value(&self) -> u8 {
        self.value
    }
}
fn main() {}
