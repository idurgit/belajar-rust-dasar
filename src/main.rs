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