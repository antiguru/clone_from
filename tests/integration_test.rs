use clone_from::CloneFrom;

#[derive(CloneFrom)]
struct MyTest {
    _name: String,
    _whatnot: usize,
}

#[derive(CloneFrom)]
struct MyTestGeneric<'a, T> {
    _inner: T,
    _inner_ref: &'a T,
}

#[derive(CloneFrom)]
struct UnnamedStruct<'a, T>(T, &'a T, String, u64);

// Fails to compile:
// #[derive(CloneFrom)]
// enum Enum {}
// #[derive(CloneFrom)]
// union Union { f1: usize }
// #[derive(CloneFrom)]
// struct Unit;
