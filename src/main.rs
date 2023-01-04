use std::path::Path;
use clap::Parser;
use walkdir::WalkDir;
use std::process::exit;
use sha2::{Sha256, Digest};
use std::{fs, io};

/*
TODO: save output to csv file
*/
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the target directory
    #[arg(short, long)]
    path: String,
    /// Filter by file limit
    #[arg(short, long, default_value_t = 0)]
    limit: u64,
}

fn get_hash(path: &Path) -> String {
    let mut file = fs::File::open(path)
        .expect("Err");
    let mut hasher = Sha256::new();
    let n = io::copy(&mut file, &mut hasher)
        .expect("Err");
    let hash = hasher.finalize();
    return hex::encode(hash);
}

fn main() {
    let args = Args::parse();
    let p = Path::new(&args.path);
    let limit = args.limit;

    if p.is_dir() {

        for x in WalkDir::new(p) {
            let x = x.unwrap();
            let _filename = x.file_name().to_string_lossy();

            let size_bytes = x.metadata().unwrap().len();
            let f_path = x.path().display();
            let _created = x.metadata()
                .unwrap()
                .created()
                .expect("Cannot unwrap");

            if !x.path().is_dir() {
                if size_bytes > limit {
                    println!("{} {} {}",
                        f_path,
                        size_bytes,
                        get_hash(x.path())
                    );
                }
            }
        }
    }
    exit(0);
}
