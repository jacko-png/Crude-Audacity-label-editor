use std::fs;
// use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // Grabbing the paths from argv is scope creep
    let tracklist = file_ctns("tracklist.txt").unwrap();
    let stamps = file_ctns("stamps.txt").unwrap();
    
    let mut titles: Vec<&str> = vec![];

    let tracklist_re = Regex::new(
    /*
    */
    r#"^\d+\t(.*?)\t"#
    ).unwrap();
    
    // let mut i = 0;
    for (_, [title]) in tracklist_re.captures_iter(&tracklist).map(|c| c.extract()) {
        titles.push(title);
        // println!("{}th :  '{}'", i, title);
        // i+=1;
    }
    
    // println!("{:?}", titles);
    
    let stamps_re = Regex::new(
    /*
    prevent matches of trailing digits in label name
        |   * used to produce zero-length matches
        |       |  Trailing tab is excluded (lookahead assertions are not in this crate)
        v       v  v                          */
    r#"^([\d.\s]+)\t"#
    ).unwrap();
    
    // change to i=0 if i is declared earlier
    let mut i = 0;
    for (_, [stamp]) in stamps_re.captures_iter(&stamps).map(|c| c.extract()) {
        let title = titles.get(i);
        let title = match title {
            Some(s) => s,
            None => panic!("There are more stamps than there are track titles."),
        };
        
        let s = format!("{stamp}\t{title}");
        println!("s: {}", s);
        
        i+=1
    }
    
    Ok(())
}




fn file_ctns(f: &str) -> Result<String, std::io::Error> {
    let mut f = fs::File::open(f)?;
    let mut ctns = String::new();
    let _ = f.read_to_string(&mut ctns); // std::io::Read trait (prelude)
    Ok(ctns)
}