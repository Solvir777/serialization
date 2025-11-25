use std::io::Read;
use serializeable::Serializeable;

fn main() {
    let data = TestEnum::C(Box::new(TestEnum::B{a: 18}));
    let a = data.serialize();
    println!("serialization finished: {:?}", a);
    let res = <TestEnum>::deserialize(&mut &a[..]);
    println!("{:?}", res);


    let data = TestStruct{
        a: 0,
        b: false,
    };
    let a = data.serialize();
    println!("serialization finished: {:?}", a);
    let res = <TestStruct>::deserialize(&mut &a[..]);
    println!("{:?}", res);

}

#[derive(Serializeable, Debug)]
enum TestEnum{
    A,
    B{a: i32},
    C(Box<TestEnum>)
}

#[derive(Serializeable, Debug)]
struct TestStruct{
    a: i32,
    b: bool,
}

