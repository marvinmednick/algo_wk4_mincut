use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};





#[derive(Debug)]
struct Edge {
	edge_id: u32,
	vertex1: u32,
	vertex2: u32,	
}


impl Edge {

	pub fn new(id : u32, v1: u32, v2: u32) -> Edge {
		Edge {
			edge_id: id,
			vertex1: v1,
			vertex2: v2,
		}
	}
}

#[derive(Debug)]
struct Vertex {
	vertex_id: u32,
	adjacent_edges: Vec<u32>
}

impl Vertex {

	pub fn new(id : u32) -> Vertex {
		let adjacent = Vec::<u32>::new();
		Vertex {vertex_id: id, adjacent_edges: adjacent}
	}
	
	pub fn add_edge(&mut self, edge_id: u32) {
		self.adjacent_edges.push(edge_id);
	}
	
}


#[derive(Debug)]
struct Graph {
	vertex_list: Vec<u32>,
	edge_list:Vec<u32>,
	vertex_map:  HashMap::<u32, Vertex>,
	edge_map:   HashMap::<u32, Edge>,
}


impl Graph {
	pub fn new() -> Graph {
		let v_list = Vec::<u32>::new();
		let e_list = Vec::<u32>::new();
		let v_map = HashMap::<u32, Vertex>::new();
		let e_map = HashMap::<u32, Edge>::new();
		Graph {
				vertex_list : v_list,
				edge_list:  e_list,
				vertex_map: v_map,
				edge_map:  e_map,
		}
	}
}



fn main() {

	let mut foo = vec![
		Vec::<u32>::new()
	];
	foo.push( Vec::<u32>::new());
	foo[0].push(1u32);


 	println!("foo {:?}", foo);


    let args: Vec<String> = env::args().collect();

	println!("Args {:?} {}",args,args.len());

	if args.len() < 2 {
        eprintln!("Usage: {} filename", args[0]);
        process::exit(1);
	}

  // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
	let mut digits = Vec::<i32>::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let x : i32 = line.unwrap().parse::<i32>().unwrap();
		digits.push(x);
    }
	println!("Read {} lines",digits.len());

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn check1() {
    }

}
