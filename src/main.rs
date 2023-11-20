use std::fs::*;
// use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    // Grabbing the paths from argv is scope creep
    let stamps = match file_ctns("stamps.txt") {
        Ok(ctns) => ctns,
        Err(e) => panic!("{e}"),
    };
    
    let tracklist = match file_ctns("tracklist.txt") {
        Ok(ctns) => ctns,
        Err(e) => panic!("{e}"),
    };
    
    let mut titles = vec![];
    let re = Regex::new(r#"^((?P<track_number>\d*)\t"*+(?P<track_name>.*?)"*+\t)"#).unwrap();
    
    for (_, [_, _, trk_title]) in re.captures_iter(&tracklist).map(|c| c.extract()) {
        titles.push(trk_title);
    }
    
    println!("{:?}", titles);
    
    let re = Regex::new(r#"([\d.\s]*)"#).unwrap();
    
    let mut i = 0;
    for (_, [ stamp ]) in re.captures_iter(&stamps).map(|c| c.extract()) {
        let title = titles.get(i);
        let title = match title {
            Some(s) => s,
            None => panic!("out of bounds ig"),
        };
        
        let s = format!("{}{}", stamp, title);
        println!("{}", s);
        
        i+=1
    }
    
    Ok(())
}


// filenames are hardcoded by design (I'll learn argv later)
fn file_ctns(f: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(f)?;
    let mut ctns = String::new();
    let _ = f.read_to_string(&mut ctns); // std::io::Read trait (prelude)
    Ok(ctns)
} // by opening f in a function, the File will be killed and can't be accidentally modified for whatever horrific reason