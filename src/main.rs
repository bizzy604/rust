fn main() {
    //let a = 99;
    /*
    if a > 200 {
        println!("Huge number");
    } else if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }
    
    if a > 99 {
        println!("Big number");
    } else if a > 200 {
        println!("Huge number");
    } else {
        println!("Small number");
    }
    */

    // use this when you have only a few conditions
    let some_bool = true;
    match some_bool {
        true => println!("It's true!"),
        false => println!("It's false!"),
    }
    // use this when you have a lot of conditions
    let a = 1000;
    match a {
        0..=99 => println!("Small number"),
        100..=199 => println!("Big number"),
        _ => println!("Huge number"),
    }
}
