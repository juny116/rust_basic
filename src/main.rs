// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
mod assign2;

#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    // Printing with `{:?}` prints "" marks!.
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

    // Assignment 2
    let x1: f64 = -40.0;
    let y1: f64 = assign2::fahrenheit_to_celsius(x1);
    println!("{:.1}", y1); // -40

    let x5: String = String::from("hello, world!");
    let y5: String = assign2::capitalize(x5);
    println!("{}", y5);  // Hello, world!

    let x2: [u64; 5] = [1, 2, 3, 4, 5];
    let y2: u64 = assign2::sum_array(&x2);
    println!("{}", y2);  // 15

    let x3: u64 = 10;
    let y3: u64 = assign2::up3(x3);
    println!("{}", y3);  // 27


    let x4a: u64 = 24;  
    let x4b: u64 = 60;
    let y4: u64 = assign2::gcd(x4a, x4b);
    println!("{}", y4);  // 12

    let x6: u64 = 4;
    let y6: Vec<u64> = assign2::chooses(x6);
    for (i, c) in y6.iter().enumerate() {
        println!("* {}C{} = {}", x6, i, c)
    }

    let x7a = vec![1, 2, 3];
    let x7b = vec![4, 5];
    let y = assign2::zip(x7a, x7b);
    for (a, b) in y.iter() {
        println!("* ({}, {})", a, b);
    }
    
    println!("{}", assign2::fibonacci(10));
    println!("{}", assign2::twelve_days_of_christmas_lyrics());
}
