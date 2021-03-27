fn main() {
    // IF
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // IF in let statement
    let condition = false;
    let y = if condition { // IF in let
        5
    }else{ // must have else
        1
    };
    // return type must be same

    println!("y = {:?}", y);


    // LOOPS

    // loop {
    // - runs forever!
    // - can interrupt with 'break'
    //}

    let mut counter = 1;
    let l = loop {
        counter = counter + 1;
        if counter > 10 {
            break counter
        }
    };
    println!("counter = {}", counter);
    println!("l = {:?}", l);

    while counter > 0 {
        counter -= 1;
    };
    println!("counter after while = {:?}", counter);

    let arr = [1, 2, 3];
    for e in arr.iter(){
        println!("{}", e);
    }

    for e in (1..4){
        println!("{}", e);
    }
}