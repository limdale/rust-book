fn main() {

    // if else
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }


    // assignment if
    let condition = true;
    let number_two = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number_two);


    // while
    let mut number_three = 3;
    while number_three != 0 {
        println!("{}!", number_three);
        number_three -= 1;
    }
    println!("when: LIFTOFF!!!");

    // for
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("for: LIFTOFF!!!");

    // for w/ array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}
