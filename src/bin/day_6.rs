use std::collections::HashSet;
use std::env;
use std::process;

fn main() {
    // Read args

    let mut data_stream = env::args();

    data_stream.next();

    let data_stream = match data_stream.next() {
        Some(arg) => arg,
        None => {
            eprintln!("No Data Stream, first arg ought to be a data stream");
            process::exit(1);
        }
    };

    for i in 0..data_stream.len() - 4 {
        if unique(&data_stream[i..i + 3 + 1]) {
            println!("{}: {}", &data_stream[i..i + 3 + 1], i + 3 + 1);
            break;
        }
    }
}

fn unique(stream: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();

    let mut stream = stream.chars();

    while let Some(c) = stream.next() {
        if chars.contains(&c) {
            return false;
        }

        chars.insert(c);
    }

    true
}
