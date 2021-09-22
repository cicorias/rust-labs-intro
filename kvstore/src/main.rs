// use std::env::Args;
use std::{fs};

fn main() {
    let mut arguments= std::env::args().skip(1);
    // for arg in arguments {
    //     println!("arg: {}", arg);
    // }

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("key {}  value {}", key, value);

    let write_result = write_database(key, value);

    match write_result {
        Ok(()) => { // or Ok(the_ok) -- which is a unit
            println!("we are ok");
        }
        Err(err) => {
            println!("got an error {}", err);
        }

    }
}


fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let contents = format!("{} {}", key, value);
    fs::write("kv.db", contents)  //no semicolon is the alt to return result from this.
}

