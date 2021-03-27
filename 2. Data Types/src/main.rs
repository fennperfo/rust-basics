fn main() {

    // integer types
    let i : u8 = 255; 
    println!("u8: {}", i);


    // floating
    let x = 2.0; // f64 by default
    let y : f32 = 3.0;


    // boolean
    let b : bool = false;
    // Booleans are one byte in size


    // char 
    // 4 bytes

    let heart_eyed_cat = '😻';
    // allows us to do this ^
    println!("cat: {}", heart_eyed_cat);



    // compound types

    // tuple 
    // Elements can be of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x (0 pos) is: {}", tup.0);


    // arrays
    // every element has the same type
    // can't change size (use vectors)

    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];

    let a : [i32; 5] = [1, 2, 3, 4, 5];
    println!("Value of 0th pos of a: {}", a[0]);

    // to create an array that contains the same value for each element
    let a = [3;5];
    // a = [3, 3, 3, 3, 3];
}
