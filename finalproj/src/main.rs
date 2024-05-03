use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::Rng;
use rand::prelude::IteratorRandom;

fn read_vec(filename: &str) -> io::Result<HashMap<usize, Vec<usize>>> {
    //creates an empty hash map top store the vertex data
    let mut vec = HashMap::new();
    //opens the file that was passed to the function
    let file = File::open(filename)?;
    //creates a buffered reader for the file
    let reader = BufReader::new(file);
}
