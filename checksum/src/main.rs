use std::{
    fs::File,
    io,
    path::Path,
};

use sha2::{Digest, Sha256};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: checksum <file>");
        return;
    }

    // Iterate through the provided file paths
    for path in args.iter().skip(1) {
        let path = Path::new(path);

        if !path.exists() {
            eprintln!("Error: File not found: {}", path.display());
            continue;
        }

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                continue;
            }
        };

        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher).unwrap();

        let checksum = hasher.finalize();
        println!("{}  {}", path.display(), hex::encode(checksum));
    }
}
