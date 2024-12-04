mod first;
mod model;
mod second;
mod third;

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
}

#[test]
fn test_module() {
    let user = model::User {
        first_name: String::from("Jaka"),
        last_name: String::from("Kelana"),
        username: String::from("Jaka"),
        email: String::from("jaka@demo.com"),
        age: 20,
    };

    user.say_hello("Agus");
}

fn main() {
    println!("Hello, world!");
    println!("Hello Rudi!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Rudi S";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Rudi S";
    println!("Hello {}", name);

    name = "Budi Santosa";
    println!("Hello {}", name);

    name = "Joko";
    println!("Hello {}", name);
}

#[test]
fn test_static_type() {
    let mut name = "Rudi S";
    println!("Hello {}", name);

    name = "Budi Santosa";
    println!("Hello {}", name);

    // name = 212;
    // println!("Hello {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Rudi S";
    println!("Hello {}", name);

    let name = 212;
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let age: i32 = 27;
    println!("Umur: {}", age);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("Umur: {}", a);

    let b: i16 = a as i16;
    println!("Umur: {}", b);

    let c: i32 = a as i32;
    println!("Umur: {}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 20;
    println!("{}", a);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result);
}

#[test]
fn tuple() {
    let data = (6, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, _b, c) = data;

    println!("{} {}", a, c);
}

#[test]
fn mutable_tuple() {
    let mut data = (6, 10.5, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 200;
    data.1 = 200.5;
    data.2 = false;
    println!("{:#?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
}

#[test]
fn array() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]

fn two_dim_array() {
    let matrix = [[1, 2, 3], [4, 5, 6]];

    println!("{:#?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 1;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn string() {
    let name = " Rudi Supra ";
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name = String::from("Rudi Supra");
    println!("{}", name);

    name.push_str(" Khannedy");
    println!("{}", name);

    // name.replace("Rudi", "Bejo");
    let name2 = name.replace("Rudi", "Bejo");
    println!("{}", name2);
}

#[test]
fn data_copy() {
    let a = "Super";
    let b = "Bejo";

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Rudi");
    println!("{}", name1);
    let name2 = name1;

    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone() {
    let name1 = String::from("Rudi");
    println!("{}", name1);
    let name2 = name1.clone();

    println!("{}", name2);
    println!("{}", name1);
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 8 {
        println!("Good");
    }
}

#[test]
fn else_expression() {
    let value = 7;

    if value >= 8 {
        println!("Good");
    } else {
        println!("Not Good");
    }
}

#[test]
fn else_if_expression() {
    let value = 1;

    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("Not Good");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Too Bad");
    }
}

#[test]
fn let_statement() {
    let value = 1;
    let result: &str;

    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Not Good";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Too Bad";
    }
    println!("{}", result);
}

#[test]
fn if_let_statement() {
    let value = 1;
    let result = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Good"
    } else if value >= 3 {
        "Bad"
    } else {
        "Too Bad"
    };
    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 20 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 20 {
            break counter * 2;
        }
    };

    println!("Counter: {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 12 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn for_array_iteration() {
    let array = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array = ["A", "B", "C", "D", "E"];
    println!("Array Start : {}", array[0]);
    println!("Array End : {}", array[4]);

    for i in range {
        println!("{}", array[i]);
        println!("{}", i);
    }
}

#[test]
fn range2() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array = ["A", "B", "C", "D", "E"];
    println!("Array Start : {}", array[0]);
    println!("Array End : {}", array[4]);

    for i in range {
        println!("Index: {}, Value: {}", i, array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    let array = ["A", "B", "C", "D", "E"];

    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    for i in range {
        println!(" Value {}", array[i]);
    }
}

// fn say_hello() {
//     println!("Hello");
// }

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_parameter() {
    say_goodbye("Jaka", "Kembara");
    say_goodbye("Rudi", "Supratman");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_loop(-10);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Rudi"), 5);
}
#[test]
fn test_print_text2() {
    print_text(String::from("Rudi"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("Result: {}", result);
}

//---------------------------------

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Rudi");
    hi(name);
    // println!("{}", name);
}
//--------------------------------

fn full_name(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Jaka");
    let last_name = String::from("Kelana");

    let (first_name, last_name, name) = full_name(first_name, last_name);
    // let full_name = full_name(first_name, last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}
//---------------------------------

fn full_name2(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name2() {
    let first_name = String::from("Jaka");
    let last_name = String::from("Kelana");

    let name = full_name2(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}
//-------------------------

fn change_value(value: &mut String) {
    value.push_str("Test");
}

// #[test]
// fn test_change_value() {
//     let mut value = String::from("Rudi");
//     change_value(&mut value);
//     println!("{}", value);
// }

// #[test]
// fn test_change_value() {
//     let mut value = String::from("Rudi");
//     change_value(&mut value);
//     change_value(&mut value);
//     change_value(&mut value);
//     println!("{}", value);
// }

#[test]
fn test_change_value() {
    let mut value = String::from("Rudi");

    let value_borrow1 = &mut value;
    // let valueBorrow2 = &mut value;

    change_value(value_borrow1);
    // change_value(valueBorrow2);

    println!("{}", value);
}
//-------------------------------

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Jaka");
    let last_name = String::from("Kelana");

    let name = get_full_name(&first_name, &last_name);
    println!("{}", name);
}
//----------------------------------------------

#[test]
fn slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Jaka Kelana Umbara");
    let first_name = &name[0..4];
    println!("{}", first_name);

    let last_name = &name[5..];
    println!("{}", last_name);
}
//-------------------------------

// struct Person {
//     first_name: String,
//     middle_name: String,
//     last_name: String,
//     age: u8,
// }

// impl Person {
//     fn say_hello(&self, name: &str) {
//         println!("Hello {}, my name is {}", name, self.first_name);
//     }
// }

// fn print_person(person: &Person) {
//     println!("First Name: {}", person.first_name);
//     println!("Middle Name: {}", person.middle_name);
//     println!("Last Name: {}", person.last_name);
//     println!("Age: {}", person.age);
// }

// #[test]
// fn struct_person() {
//     let person = Person {
//         first_name: String::from("Jaka"),
//         middle_name: String::from("Kelana"),
//         last_name: String::from("Umbara"),
//         age: 20,
//     };

//     print_person(&person);
// }

// Init Shorthand -- Struct Update Syntax
#[test]
fn struct_person() {
    let first_name = String::from("Jaka");
    let last_name = String::from("Umbara");

    let person = Person {
        first_name,
        middle_name: String::from("Kelana"),
        last_name,
        age: 20,
    };

    print_person(&person);
    // println!("{}", last_name);

    let person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);

    // println!("{}", person.first_name);
}

// Tuple Struct ------
struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(10.11, 12.22);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.2123456, 106.816666);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}

// Struct Kosong
struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing {};
}

// Reference Field di Struct --------------

// Membuat Method -----------------
// impl Person {
//     fn say_hello(&self, name: &str) {
//         println!("Hello, {} my name is {}", name, self.first_name)
//     }
// }

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("First Name: {}", person.first_name);
    println!("Middle Name: {}", person.middle_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Jaka"),
        middle_name: String::from("Kelana"),
        last_name: String::from("Umbara"),
        age: 20,
    };

    person.say_hello("Budi");

    println!("{}", person.first_name);
}

// Enum ----------------
enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

// #[test]
// fn test_enum() {
//     let _level = Level::Premium;
// }

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with ewallet {} {} amount {}",
                    wallet, number, amount
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("3245345434"));
    _payment1.pay(50000);
    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("3245345434"));
    _payment2.pay(60000);
    let _payment3 = Payment::EWallet(String::from("Gopay"), String::from("3245345434"));
    _payment3.pay(70000);
}

#[test]
fn test_match_value() {
    let name = "Adi";

    match name {
        "Rudi" | "Budi" => {
            println!("Hello Sir");
        }
        "Joko" | "Eko" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 900;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Bad")
        }
        0..=24 => {
            println!("Too Bad")
        }
        other => {
            println!("Invalid value {}", other)
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Jaka"),
        middle_name: String::from("Kelana"),
        last_name: String::from("Umbara"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(3.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("long: {}", long);
        }
    }
}

#[test]
fn test_range_ignoring() {
    let value = 900;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Bad")
        }
        0..=24 => {
            println!("Too Bad")
        }
        _ => {
            println!("Invalid value ")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;

    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "Invalid Data",
    };

    println!("{}", result);
}
