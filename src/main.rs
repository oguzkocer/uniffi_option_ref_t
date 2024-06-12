uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
struct MyRecord {}

#[derive(uniffi::Enum)]
enum MyEnum {
    Foo,
    Bar,
}

#[uniffi::export]
fn process_data(_a: &MyRecord, _b: &MyEnum, _c: Option<&MyRecord>) {}

fn main() {
    println!("Hello, world!");
}
