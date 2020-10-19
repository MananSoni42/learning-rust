// read numbers from a file and find their mean, median and mode

use std::vec::Vec;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read(path: &str) -> Result<Vec<i32>, Error> {
    let mut data: Vec<i32> = Vec::new();

    //let file = File::open(path).expect("File not found"); // panics if file not found
    let file = File::open(path)?; // returns error to the caller instead
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?; // returns an error if there is an I/O error
        let n:i32 = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // need to explicitly convert to
        data.push(n);
    }
    Ok(data) // return vector
}

fn mean(num: &Vec<i32>) -> f32 {
    num.iter().sum::<i32>() as f32 / num.len() as f32
}
fn median(num: &mut Vec<i32>) -> f32 {
    num.sort();
    if num.len()%2 == 0 {
        0.5*num[num.len()/2] as f32 + 0.5*num[num.len()/2 - 1] as f32
    }
    else {
        num[(num.len()-1)/2] as f32
    }
}

fn mode(num: &Vec<i32>) -> i32 {
    let mut count: HashMap<i32,i32> = HashMap::new();
    for &elem in num {
        *count.entry(elem).or_insert(0) += 1;
    }

    let mut mode = num[0];
    for (&key, &val) in count.iter() {
        if val > count[&mode] {
            mode = key;
        }
    }
    mode
}

fn main() {
    let v = read("./data.txt"); // read the text file into v
    match v {
        Ok(mut x) => {
            println!("data: {:?}", x);
            println!("mean: {}", mean(&x));
            println!("median: {}", median(&mut x));
            println!("mode: {}", mode(&x));
        }
        Err(e) => {
            println!("An error occured while reading the file: {}",e);
        }

    }
}
