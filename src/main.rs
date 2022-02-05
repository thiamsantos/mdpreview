use clap::Parser;
use std::path::Path;

/// Preview markdown files rendered using Github API
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Markdown file to render
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.file);

    let path = Path::new(&args.file);

    println!("{:?}", path.extension().unwrap())

    // validate file extension
    //
    // validate file exists
    //
    // start server
    //
    // open in browser
    //
    // req: read file
    // req: post github api
    // req: render template
}
