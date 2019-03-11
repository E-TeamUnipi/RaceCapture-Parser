// Copyright (C) 2019  Giuseppe Fabiano

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Header<'a> {
    name: &'a str,
    units: &'a str,
    min: &'a str,
    max: &'a str,
    freq: &'a str
}

#[derive(Debug)]
struct Point(String, String);

fn main() -> std::io::Result<()> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let mut buf_reader = BufReader::new(file);

    let mut header_str = String::new();
    buf_reader.read_line(&mut header_str)?;
    
    let headers: Vec<Header> = header_str.split(",").map(|x| {
        let mut x = x.split("|");
        Header {
            name: x.next().unwrap().trim_matches('"'),
            units: x.next().unwrap().trim_matches('"'),
            min: x.next().unwrap().trim(),
            max: x.next().unwrap().trim(),
            freq: x.next().unwrap().trim()
        } 
    }).collect();

    let mut points: Vec<Vec<Point>> = Vec::with_capacity(headers.len());

    for _ in 0..headers.len() {
        points.push(Vec::new());
    }

    for line in buf_reader.lines() {
        let line = line?;
        let mut line_split = line.split(',');
        let istant = line_split.next().unwrap();
        for (i, val) in line_split.enumerate() {
            if i > 0 && val != "" {
                points[i].push(
                    Point(
                        istant.to_string(),
                        val.to_string()
                    )
                );
            }
        }
    }
    
    for (i, variable) in headers.iter().enumerate() {
        if i > 1 {
            println!("{} = [", variable.name);
            for point in points[i].iter() {
                println!("  {} {}", point.0, point.1);
            }
            println!("];");
        }
    } 

    Ok(())
}
