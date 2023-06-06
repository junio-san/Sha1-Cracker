use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 { println!
        ("Use:"); println!
        ("sha1-cracker: <wordlist.txt> <sha1_hash>");       
        return;
    }
}
