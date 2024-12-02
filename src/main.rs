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