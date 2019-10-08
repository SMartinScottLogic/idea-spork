trait A {
    fn name(&self);
}

trait B {
    fn name(&self);
}

trait C {
    fn name(&self);
}

#[derive(Debug)]
struct D {
    value: u8,
    name: String
}

impl D {
    fn new() -> D {
        D { value: 1, name: "D".to_string() }
    }
}

impl A for D {
    fn name(&self) {
        println!("A for D {:?}", self);
    }
}

impl B for D {
    fn name(&self) {
        println!("B for D {:?}", self);
    }
}

impl C for D {
    fn name(&self) {
        println!("B for D {:?}", self);
    }
}

fn main() {
    let o = D::new();
    //o.name();
    A::name(&o);
    B::name(&o);
    C::name(&o);
}