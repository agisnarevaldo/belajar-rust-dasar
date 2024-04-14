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

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b)
}

#[test]
fn comparison() {
    let result = 10 >= 20;
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 800;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("lulus = {}", lulus_final);
}

#[test]
fn char() {
    let char1: char = 'a';
    let char2 = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let data = (10, 10.5, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{}, {}, {}", a, b, c);
}

#[test]
fn destructuring_tuple() {
    let data = (10, 10.5, true);

    let (a, b, c) = data;
    println!("{}, {}, {}", a, b, c);
}

#[test]
fn mutable_tuple() {
    let mut data = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{}, {}, {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

#[test]
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}
