use std::path::Path;
use clap::Parser;
use walkdir::WalkDir;

/*
TODO: Count hash for each file
TODO: save output to csv file
*/
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the target directory
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let p = Path::new(&args.path);

    if p.is_dir() {

        for x in WalkDir::new(p) {
            let x = x.unwrap();
            let filename = x.file_name().to_string_lossy();

            let size_bytes = x.metadata().unwrap().len();
            let f_path = x.path().display();
            let created = x.metadata()
                .unwrap()
                .created()
                .expect("Cannot unwrap");

            println!("{} {} {:?}",
                f_path,
                size_bytes,
                created
            )
        }
    }
}
