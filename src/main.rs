use std::fs::File;
use std::env;
use std::io::Write;
use lagda2html::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    match get_args() {
        Err(err) => println!("{}", err),
        Ok(mut args) => {
            let br = BufReader::new(args.source);
            match args.target.write_all(&mk_html(br).as_bytes()) {
                Ok(_) => println!("HTML generation successfull."),
                Err(_) => println!("Some problem when generating HTML")
            }
        }
    }
    Ok(())
}
    

#[derive(Debug)]
struct Args {
    pub source : File,
    pub target : File,
}

fn get_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        Err("Please provide a source and a target as arguments")
    } else {
        Ok(Args {
            source : File::open(&args[1]).unwrap(),
            target : File::create(&args[2]).unwrap(),
        })
    }
}
    
            
