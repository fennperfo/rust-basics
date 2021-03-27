fn main() {
    
    let mut s = String::from("hello");

    // Rule #1

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
 
    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &mut s;
    // let r2 = &mut s1;

    // println!("{} {}", r1, r2);
 


    // Rule #2
    let r1 = &s;
    // let r2 = &mut s1;
    // println!("{} {}", r1, r2);

    // Dangling References
    // let r = dangle();
}

// fn dangle() -> &String{
//     let s = String::from("ok");
//     &s
// }
