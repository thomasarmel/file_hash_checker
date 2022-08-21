use file_hash_checker::file_hash::FileHash;

fn main() {
    if std::env::args().count() < 2 {
        print_usage_stderr();
        std::process::exit(1);
    }

    let filename = match std::env::args().nth(1) {
        Some(filename) => filename,
        None => {
            print_usage_stderr();
            std::process::exit(1);
        }
    };

    let mut file_hasher = match FileHash::new(filename.as_str()) {
        Ok(file_hasher) => file_hasher,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(2);
        }
    };

    match file_hasher.get_hash() {
        Ok(hash) => {
            println!("{}", hash);
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(3);
        }
    }
}

fn print_usage_stderr() {
    eprintln!("Usage: file_hash_checker_cli <file_to_check>");
}