use std::fs::File;
pub fn run(){

    //use std::fs::File;
    //let f: u32 = File::open("hello.txt");

    /*let v = vec![1, 2, 3];
    v[99];*/

    let result = foo(11).unwrap();
    println!("result is {}", result);
    println!("end of main");

    let f = File::open("test.jpg");
    match f{
        Ok(f)=>{
            println!("file found {:?}", f);
        },
        Err(e)=>{
            println!("Not found \n{:?}", e);
        }
    }
    println!("End of main")
}

fn foo(x: i32)->Result<bool,String>{
    if x % 2 != 0{
        return Ok(true);
    } else{
        return Err("NOT_VALID".to_string());
    }
}