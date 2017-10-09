/// quicksort-rs
/// A little tool to sort string given in an input file and writes the sorted string back

use std::fs::{File};
use std::io::{Read, BufReader, Write};
use std::env;

fn main() {


    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);


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