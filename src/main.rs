// Copyright (C) 2019  Giuseppe Fabiano

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point(String, String);

#[derive(Debug)]
struct Header<'a> {
    name: &'a str,
    units: &'a str,
    min: &'a str,
    max: &'a str,
    freq: &'a str,
    points: Vec<Point>
}



fn main() -> std::io::Result<()> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let mut buf_reader = BufReader::new(file);

    let mut header_str = String::new();
    buf_reader.read_line(&mut header_str)?;
    
    let mut headers: Vec<Header> = header_str.split(",").map(|x| {
        let mut x = x.split("|");
        Header {
            name: x.next().unwrap().trim_matches('"'),
            units: x.next().unwrap().trim_matches('"'),
            min: x.next().unwrap().trim(),
            max: x.next().unwrap().trim(),
            freq: x.next().unwrap().trim(),
            points: Vec::new()
        } 
    }).collect();

    for line in buf_reader.lines() {
        let line = line?;
        let mut line_split = line.split(',');
        let interval = line_split.next().unwrap();
        for (i, val) in line_split.skip(1).enumerate() {
            if !val.trim().is_empty() { 
                headers[i+2].points.push(
                    Point(
                        interval.to_string(),
                        val.to_string()
                    )
                );
            }
        }
    }
    
    for header in headers.iter().skip(2) {
        println!("{} = [", header.name);
        for point in header.points.iter() {
            println!("  {} {}", point.0, point.1);
        }
        println!("];");
    } 

    Ok(())
}
