fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Agis";
    println!("Hello, {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Agis";
    println!("{}", name);
    name = "Agisna Revaldo";
    println!("{}", name);
}

// #[test]
// fn static_typing() {
//     let mut name = "agis";
//     print!("{}", name);
//     name = 10;
//     print!("{}", name);
// }

#[test]
fn shadowing() {
    let name = "Agisna Revaldo";
    println!("{}", name);
    let name = 21;
    println!("{}", name);
}

#[test]
fn explicit() {
    let age: i8 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);
    let b: f64 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversasion() {
    let a: i32 = 1000000;
    println!("{}", a);

    let b: i8 = a as i8;
    println!("{}", b);

    let c: i16 = a as i16;
    println!("{}", c);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 20;
    let c = a * b;
    println!("{}", c);
}
