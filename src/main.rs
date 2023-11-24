

use std::fs;
use std::io::prelude::*;
use regex::Regex;
use std::fmt;

struct ReHayPair {
    re: Regex,
    hay: String,
}

#[derive(Debug)]
struct Range(f64, f64);

fn get_padding() -> f64 {
    0.2
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (earlier, later) = (self.0, self.1);
        
        let range = format!("{earlier}\t{later}");
        write!(f, "{}", range)
    }
}

fn main() -> std::io::Result<()> {
    // let mut i: u8 = 0;  // no more than 255 outputs. Hopefully no one ever needs more than 255 outputs…
// unstable for like the next week or so
/*     let out = loop {
        if i == 0 {
            match fs::File::create_new("out.txt") {
                Ok(f) => break f,
                Err(_) => {
                    i += 1;
                    continue
                } 
            }
        } else if i <= 255 {
            let path = format!("out{i}.txt");
            match fs::File::create_new(path) {
                Ok(f) => break f,
                Err(_) => {
                    i += 1;
                    continue
                }
            }
        } else {
            panic!("Arbitrary limit of 255 outputs reached. Change the i in this code block to a u16, etc.\n    --your friendly neighbourhood spiderman")
        }
    }; */
    let mut out = fs::File::create("out.txt")?;
    
    let tracklist = ReHayPair {
        re: Regex::new(
            // r#"(?m)^\d+\t(.*?)\t"#
            r#"(?m)^\d+\t(.+?)$"#
            ).unwrap(),
        hay: file_ctns("tracklist.txt").unwrap(),
    };
    let stamps = ReHayPair {
        re: Regex::new(
            r#"(?m)^([\d.]+)\s([\d.]+)\t"#
            ).unwrap(),
        hay: file_ctns("stamps.txt").unwrap(),
    };
    
    let mut titles: Vec<&str> = vec![];
    
    for (_, [title]) in tracklist.re.captures_iter(&tracklist.hay).map(|c| c.extract()) {
        titles.push(title);
    }
    
    println!("{:#?}", titles);
    
    let mut i = 0;
    for (_, [earlier, later]) in stamps.re.captures_iter(&stamps.hay).map(|c| c.extract()) {
        let title = titles.get(i)
        .expect(format!("There are more labels than there are titles.\n{i} tracks were found.").as_str());
        
        let earlier: f64 = earlier.parse().unwrap();
        let later: f64 = later.parse().unwrap();
        
        let stamp = apply_padding(earlier, later);
        
        let _ = out.write_fmt(format_args!("{stamp}\t{title}\n"));
        i+=1
    }
    println!("{i} tracks were found.");
    
    Ok(())
}


fn apply_padding(mut e: f64, mut l: f64) -> Range {
    let padding = get_padding();
    
    e -= padding;
    if e < 0.0 {
        e=0.0
    }
    
    l += padding;
    
    Range(e, l)
}

fn file_ctns(f: &str) -> Result<String, std::io::Error> {
    let mut f = fs::File::open(f)?;
    let mut ctns = String::new();
    let _ = f.read_to_string(&mut ctns); // std::io::Read trait (prelude)
    Ok(ctns)
}

    /*
    enable multiline mode
        | prevent matches of trailing digits in label name
        |  |    * produces unwanted zero-length matches
        |  |        |  Trailing tab is excluded (lookahead assertions are not in this crate)
        v  v        v  v   
       (?m)^([\d.\s]+)\t  
    */