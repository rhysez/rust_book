fn main() {
    let number: u32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let string: &str = if number == 3 {
        "Number is 3"
    } else {
        "Number is not 3"
    };

    println!("{string}");

    loops_example();
    while_example();
    for_example();
}

fn loops_example() {
    let mut counter: u32 = 0;

    let result: u32 = loop {
        counter += 1;

        if counter == 5 {
            println!("We're halfway there!");
            continue;
        }

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_example() {
    let mut number: u32 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Lift off!!!");
}

fn for_example() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
