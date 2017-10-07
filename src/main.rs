use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader, Write};

fn main() {
    let mut data = String::new();
    let fin = File::open("test.txt").expect("Unable to open file");
    let mut br = BufReader::new(fin);
    br.read_to_string(&mut data)
        .expect("Unable to read string");
    //for control, print data
    println!("\nUnsortierter String:");
    println!("{}", data);

    //split data into single strings using defined separator
    let mut strings: Vec<&str> = data.split(",").collect();

    //sort strings
    strings.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    println!("\nSortierter String:");
    for i in strings.iter() {
        print!("{},", i);
    }

    let mut fout = File::create("out.txt").unwrap();
    for j in strings.iter() {
        fout.write(j.as_bytes()).unwrap();
        fout.write(",".as_bytes()).unwrap();
    }

}