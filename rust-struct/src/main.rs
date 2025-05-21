use std::io::Read;
use std::fs::File;
use std::error::Error;

fn main() {
    let user_name = read_username();
    match user_name {
        Ok(name)=> println!("{}",name),
        Err(_) => panic!("Error occured")
    }

}

fn read_username() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("./hello.txt");
    let user_name = match &mut file {
        Ok(file) => {
            let mut file_username = String::new();
            match file.read_to_string(&mut file_username) {
                Ok(_) => file_username,
                Err(error) => panic!("Problem reading the contents: {error:?}"),
            }
        },
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    Ok(user_name)
}
