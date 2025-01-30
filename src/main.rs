use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_line: Vec<String> = args().collect();
    let line: Vec<&str> = raw_line.iter().map(|x| x.as_str()).collect();
    if line.len() < 2 {
        eprint!("yo need atleast 2 args");
        exit(1);
    }
    if let Some(word) = line.iter().find(|w| w.starts_with('-')) {
        match *word {
            "-f" => with_file(&line)?,
            "-i" => {}
            _ => piped(&line)?,
        }
    } else {
        let _ = piped(&line);
    }

    //
    //if line.contains(&"-f") {
    //    let line_f = &line[1..];
    //    if let Some(file) = line_f.iter().position(|&minus| minus == "-f") {
    //        let del_args = &line_f[file..file + 2];
    //        let argf: Vec<&str> = line_f
    //            .iter()
    //            .filter(|&&x| !del_args.contains(&x))
    //            .copied()
    //            .collect();
    //        let _ = with_file(line_f[file + 1], &argf);
    //    } else {
    //        eprintln!("bad use o -")
    //    }
    //} else {
    //    let _ = piped(&line[..]);
    //}

    Ok(())
}

fn with_file(line: &[&str]) -> Result<(), Box<dyn Error>> {
    let line_f = &line[1..];
    if let Some(filez) = line_f.iter().position(|&minus| minus == "-f") {
        let del_args = &line_f[filez..filez + 2];
        let args: Vec<&str> = line_f
            .iter()
            .filter(|&&x| !del_args.contains(&x))
            .copied()
            .collect();
        let filename = line_f[filez + 1];
        let filef = File::open(filename)?;
        let reader = BufReader::new(filef);
        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            if args.iter().all(|arg| line.contains(arg)) {
                println!("{}: {}", line_number + 1, &line);
            }
        }
    } else {
        eprintln!("bad use o -f")
    }
    //let filename = line_f[filez + 1];
    //let filef = File::open(filename)?;
    //let reader = BufReader::new(filef);
    //for (line_number, line) in reader.lines().enumerate() {
    //    let line = line?;
    //    if args.iter().all(|arg| line.contains(arg)) {
    //        println!("{}: {}", line_number + 1, &line);
    //    }
    //}

    Ok(())
}

fn piped(piped: &[&str]) -> Result<(), Box<dyn Error>> {
    let keywords = &piped[1..];
    for (line_num, line) in stdin().lock().lines().enumerate() {
        let line = line?;
        if keywords.iter().all(|kw| line.contains(kw)) {
            println!("{}: {}", line_num + 1, line);
        }
    }
    Ok(())
}
