struct MyStruct(u8);

fn outer_mut_struct(my_struct: &mut MyStruct) {
    inner_struct(my_struct);
    inner_mut_struct(my_struct);
    inner_struct(my_struct);
}

fn inner_mut_struct(my_inner_mut_struct: &mut MyStruct) {
    my_inner_mut_struct.0 = 4;
}

fn inner_struct(my_inner_struct: &MyStruct) {
    println!("{:?}", my_inner_struct.0);
}

fn main() {
    let mut my_struct = MyStruct(1);
    outer_mut_struct(&mut my_struct);
}
