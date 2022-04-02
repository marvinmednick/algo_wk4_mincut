use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};



#[derive(Debug)]
struct Edge {
	edge_id: String,
	vertex1: u32,
	vertex2: u32,	
}


impl Edge {

	pub fn new(id : String, v1: u32, v2: u32) -> Edge {
		Edge {
			edge_id: id,
			vertex1: v1,
			vertex2: v2,
		}
	}
}

#[derive(Debug, Clone)]
struct Vertex {
	vertex_id: u32,
	adjacent_edges: Vec<String>
}

impl Vertex {

	pub fn new(id : &u32) -> Vertex {
		let adjacent = Vec::<String>::new();
		Vertex {vertex_id: id.clone(), adjacent_edges: adjacent}
	}
	
	pub fn add_edge(&mut self, edge_id: String) {
		self.adjacent_edges.push(edge_id);
	}
	
}

#[derive(Debug)]
enum GraphType {
	Undirected,
	Directed

}

#[derive(Debug)]
struct Graph {
	graph_type: GraphType,
	pub vertex_list: Vec<u32>,
	pub edge_list:Vec<String>,
	vertex_map:  HashMap::<u32, Vertex>,
	edge_map:   HashMap::<String, Edge>,
	edge_index: u32,
}


impl Graph {
	pub fn new(gtype: GraphType) -> Graph {
		let v_list = Vec::<u32>::new();
		let e_list = Vec::<String>::new();
		let v_map = HashMap::<u32, Vertex>::new();
		let e_map = HashMap::<String, Edge>::new();
		Graph {
				graph_type:  gtype,
				vertex_list : v_list,
				edge_list:  e_list,
				vertex_map: v_map,
				edge_map:  e_map,
				edge_index: 0
		}
	}

	pub fn add_vertex(&mut self,id: &u32) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
			let v = Vertex::new(&id);
			self.vertex_map.insert(id.clone(),v);
			self.vertex_list.push(id.clone());
			Some(self.vertex_list.len())
		}
		
	}

	pub fn add_edge(&mut self, v1: u32, v2: u32) -> Result<usize,&'static str> {

		let edge_name = format!("{}:{}_{}",self.edge_index+1,v1,v2).to_string();

		// I don't think this can happen since now the edge name is unique
		// based on an internal incrementing count. so two edges between the same 
		// vertexes would still have differrent names
		if self.edge_map.contains_key(&edge_name) {
			Err("Edge already exists")
		} 
		else {
			self.add_vertex(&v1);
			self.add_vertex(&v2);
			let v_map = &mut self.vertex_map;
			let mut v_map2 = v_map.clone();
			let vert1 = v_map.get_mut(&v1); 
			let vert2 = v_map2.get_mut(&v2); 
			let e = Edge::new(edge_name.clone(),v1,v2);
			self.edge_map.insert(edge_name.clone(),e);
			self.edge_list.push(edge_name.clone());
			vert1.unwrap().add_edge(edge_name.clone());
			vert2.unwrap().add_edge(edge_name.clone());
			self.edge_index += 1;
			Ok(self.edge_list.len())
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

	let mut g = Graph::new(GraphType::Undirected);

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
		let mut tokens = line_data.split_whitespace();
		let vertex = tokens.next().unwrap().parse::<u32>().unwrap();
		let adjacent : Vec<u32> = tokens.map(|x| x.to_string().parse::<u32>().unwrap()).collect();

		g.add_vertex(&vertex);
		for other_v in &adjacent {
			let num_edges = g.add_edge(vertex,*other_v);
		}
		if _count < 10 {
			println!("{} - Vertex: {} {:?}",_count,vertex,adjacent);
		}
    }
	println!("Read {} lines",_count);

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
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.add_vertex(&1),Some(1));
		assert_eq!(g.add_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Ok(1));
		assert_eq!(g.vertex_list,vec!(1,2));
		assert_eq!(g.edge_list,vec!("1:1_2".to_string()));
		assert_eq!(g.add_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Ok(2));
		assert_eq!(g.add_edge(2,3),Ok(3));
		assert_eq!(g.vertex_list,vec!(1,2,3));
		assert_eq!(g.edge_list,vec!("1:1_2".to_string(),"2:1_3".to_string(),"3:2_3".to_string()));
		assert_eq!(g.add_edge(1,4),Ok(4));
		assert_eq!(g.vertex_list,vec!(1,2,3,4));
		assert_eq!(g.edge_list,vec!("1:1_2".to_string(),"2:1_3".to_string(),"3:2_3".to_string(),"4:1_4".to_string()));
		println!("{:?}",g);

    }

}
