pub fn run(){

    let number: i32 = 30;

    //println!()

    match number{

        5 | 10 => println!("Class A"),
        30 => println!("Class B"),
        21..=40 => println!("Class C"),
        _ => println!("Class not registered")
    }
}