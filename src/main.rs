use std::fs::*;
// use std::io;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // todo: path of tracklist and stamps are from argv.
    // -t path\to\tracklist.note -s path\to\stamps
    // or to make it easier to run, look for tracklist.txt or stamps.txt
    // in current directory (File::open, ErrorKind::NotFound => File::create)
    // wait for file to be overwritten
    //          ^scope creep
    // just look for tracklist.txt and stamps.txt then make out.txt
    
    let stamps = match file_ctns("stamps.txt") {
        Ok(ctns) => ctns,
        Err(e) => panic!("{e}"),
    };
    
    let tracklist = match file_ctns("tracklist.txt") {
        Ok(ctns) => ctns,
        Err(e) => panic!("{e}"),
    };
    
    println!("{}", tracklist);
    Ok(())
}

//filenames are hardcoded by design (I'll learn argv later)
fn file_ctns(f: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(f)?;
    let mut ctns = String::new();
    let _ = f.read_to_string(&mut ctns); // std::io::Read trait (prelude)
    Ok(ctns)
} // by opening f in a function, the File will be killed and can't be accidentally modified for whatever horrific reason