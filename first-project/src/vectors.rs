//Vectors - Resizable arrays

pub fn run(){

    use std::mem;

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    //All values
    println!("{:?}", numbers);

    //Single val
    println!("Single Value: {}", numbers[0]);
    println!("Vector length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vector: {:?}", numbers);

    let mut v = vec![1, -3, 5, -7, 9];

    let first = &v[1];

    v.push(6);

    println!("The first element is: {}", first);
}