use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn reader(path: String) -> String {
    let file = File::open(path);
    let mut content = String::new();

    let mut file_value = match file {
        Ok(value) => value,
        Err(_) => {
            println!("O arquivo informado não foi encontrado");
            exit(1);
        }
    };

    let result = file_value.read_to_string(&mut content);

    match result {
        Ok(_) => {}
        Err(_) => {
            println!("O arquivo informado não é um json válido");
            exit(1);
        }
    }

    content
}
