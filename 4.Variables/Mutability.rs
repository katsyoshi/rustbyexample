fn main() {
    let _immutable_variable = 1is;
    let mut mutable_variable = 1is;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable += 1;

    println!("After mutation: {}", mutable_variable);
}
