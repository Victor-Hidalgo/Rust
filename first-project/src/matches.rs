pub fn run(){

    let number: i32 = 30;

    //println!()

    match number{

        5 | 10 => println!("Class A"),
        30 => println!("Class B"),
        21..=40 => println!("Class C"),
        _ => println!("Class not registered")
    }

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32{
        match coin{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("Value of quarter is: {}", value_in_cents(Coin::Quarter));
}
