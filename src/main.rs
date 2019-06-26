fn inc(val: &mut i32) {
    (*val) += 1;
}

fn main() {
    let mut number = 13;

    inc(&mut number);

    println!("Tell me about {}", number);
}