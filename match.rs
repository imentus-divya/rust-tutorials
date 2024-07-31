// Rust provides pattern matching via the match keyword, which can be used like a C switch. The first matching arm is evaluated and all possible values must be covered.
fn main() {
    let number: &str = "one";
    match number {
        "one" => println!("one"),
        "two" => println!("two"),
        _ => println!("anything else"),
    }

    // boolean
    let boolean: bool = false;
    match boolean {
        true => println!("its {}", boolean),
        _ => println!("its {}", boolean),
    }
}
