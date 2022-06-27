/*Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or
own string data
*/
pub fn run(){

    //Growable type definition
    let mut hello = String::from("Hello ");
    
    println!("Length: {}", hello.len());
    
    //Push char
    hello.push('W');

    //Push string
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World': {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    /*for word in hello.split_whitespace(){
        println!("{}", word);
    }*/

    let mut s = String::from("Hello, ");

    s.push('W');
    s.push_str("orld!");
    s.replace("World", "There");
    println!("End: {}", s);

    /*//Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());*/

    let s2 = String::from("Hello,");
    let mut s1 = String::from("World!");
    s1.push_str("Hello,");

    //println!("{}", s1);
}