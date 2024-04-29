use core::hash;
use std::{thread::JoinHandle, vec};

fn main() {
    //unsigned integer u8,u16,u32,u64,u128
    let unsigned: u32 = 32;

    //signed integer i8,i16,i32,i64,i128
    let signed: i32 = -3;

    //floating point f32,f64
    let float: f64 = 3.14;

    let sum: i32 = unsigned as i32 + signed;

    println!("The value of sum is: {}", sum);

    //char
    let chars: char = 'a';

    //boolean
    let boolean: bool = true;

    //array
    let array: [i32; 4] = [1, 2, 34, 32];
    println!("The value of array is: {:?}", array);

    //string
    let name: &str = "Hello, World!";

    let name1 = String::from("Hello, World!");

    print!("name1 is {} and name2 is {}",name,name1);

    //mutability

    let mut num1: i32 = 10;
    let num2: i32 = 20;

    num1 = 16;
    println!("The value of num1 is: {}", num1);

    // if we write num2=25; it will give error because num2 is immutable

    println!("The value of num1 is: {}", num1);

    //hashmap

    let mut hashmap = std::collections::HashMap::new();

    hashmap.insert(1001, "John Doe");
    hashmap.insert(1002, "jake");

    let mut hash = std::collections::HashMap::new();

    hash.insert("blue", "KKr");
    hash.insert("yellow", "csk");

    println!("the value of hashmap is: {:?}", hashmap);

    //struct - we can make our own data type

    struct point {
        x: i32,
        y: i32,
    }

    //enums - complex data type

    #[derive(Debug)]
    enum Color {
        red,
        green,
        blue,
    }

    let color = Color::red;
    println!("The value of color is: {:?}", color);

    //loop

    let mut i = 0;
    loop {
        println!("The value of i is: {}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }

    //while loop

    let mut j = 0;
    while j < 8 {
        println!("The value of j is: {}", j);
        j += 2;
    }

    //for loop

    for i in 0..10 {
        println!("The value of i is: {}", i);
    }

    //vector

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of v is: {:?} and the array is {:?}", v,array);

    //owner ship concept example

    let name1: String = String::from("Alen");
        transfer_ownership( name1);
        
}


fn transfer_ownership(name: String) {
    println!("The value of name is: {:?}", name);
}