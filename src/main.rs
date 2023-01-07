use std::path::Path;
use clap::Parser;
use walkdir::WalkDir;
use std::process::exit;
use sha2::{Sha256, Digest};
use std::{fs, io};
use serde::Serialize;
use std::error::Error;

/*
TODO: add Result to get_hash fn
TODO: add new arg for csv export, delimeter
TODO: save output to csv file
*/
#[derive(Parser, Debug)]
#[command(author, version = "0.2.1", about, long_about = None)]
struct Args {
    /// Path to the target directory
    #[arg(short, long)]
    path: String,
    /// Filter by file limit
    #[arg(short, long, default_value_t = 0)]
    limit: u64,
}

#[derive(Serialize)]
struct FileScan {
    filename: String,
    size: u64,
    path: String,
}

fn get_hash(path: &Path) -> String {
    let mut file = fs::File::open(path)
        .expect("Err");
    let mut hasher = Sha256::new();
    let _bytes = io::copy(&mut file, &mut hasher)
        .expect("Err");
    let hash = hasher.finalize();
    return hex::encode(hash);
}

fn to_csv(vs: &Vec<FileScan>) -> Result<(), Box<dyn Error>>{
    //let mut wtr = csv::Writer::from_path()
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for xs in vs {
        wtr.serialize(xs)?;
    }
    wtr.flush().expect("Cannot flush");
    Ok(())
}

fn main() {
    let args = Args::parse();
    let p = Path::new(&args.path);
    let limit = args.limit;

    if p.is_dir() {

        let mut xs :Vec<FileScan> = Vec::new();

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
                    let x = FileScan {
                        filename: f_path.to_string(),
                        size: size_bytes,
                        path: get_hash(x.path()).to_string(),
                    };
                    xs.push(x);
                }
            }
        }
        if let Err(err) = to_csv(&xs) {
            println!("{}", err);
            exit(1);
        }
    } else {
        println!("The path provided is not a directory");
        exit(1);
    }
    exit(0);
}
