fn f1() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn f2() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);
}

fn f3() {
    let c = 'z';
    let z = 'Ζ';
    let heart_eyed_cat = '🐱‍👤';
}

fn f4() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is ", y);
}

fn f5() {
    let number = 3;
    
    if number < 5 println!("condition was true");
    else println!("condition was false");
}