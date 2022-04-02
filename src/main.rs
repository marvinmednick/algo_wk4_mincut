use std::env;
use std::process;
use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};



#[derive(Debug)]
struct Edge {
	edge_id: String,
	vertex1: u32,
	vertex2: u32,	
	count: u32,
}


impl Edge {

	pub fn new(id : &String, v1: u32, v2: u32) -> Edge {
		Edge {
			edge_id: id.clone(),
			vertex1: v1,
			vertex2: v2,
			count: 1,
		}
	}

	pub fn incr_cnt(&mut self) {
		self.count += 1;
	}

	pub fn count(&self) -> u32 {
		self.count
	}
}

#[derive(Debug, Clone)]
struct Vertex {
	vertex_id: u32,
	adjacent: Vec<u32>
}

impl Vertex {

	pub fn new(id : &u32) -> Vertex {
		let adjacent = Vec::<u32>::new();
		Vertex {vertex_id: id.clone(), adjacent: adjacent}
	}
	
	pub fn add_adjacent(&mut self, vertex_id: u32) {
		self.adjacent.push(vertex_id);
	}
	
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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
	//edge_index: u32,
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
		}
	}

	pub fn print_vertexes(&self) {
		for (key, value) in &self.vertex_map {
			let adj_list : String = value.adjacent.iter().map(|x| format!("{} ",x)).collect();
			println!("Vertex {} ({}) :  {}",key,value.vertex_id,adj_list);
		}
					
	}

	pub fn print_edges(&self) {
		for (key, value) in &self.edge_map {
			println!("Edge {} : id {}  v1 {} v2 {} cnt {}",key, value.edge_id, value.vertex1,value.vertex2,value.count);
		}
					
	}

	pub fn create_vertex(&mut self,id: &u32) -> Option<usize> {

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

	pub fn edgename(&mut self, v1: u32, v2: u32) -> String {
		if self.graph_type == GraphType::Directed {
			format!("{}_{}",v1,v2).to_string()
		}
		else {
			let start = cmp::min(v1,v2);
			let end = cmp::max(v1,v2);
			format!("{}_{}",start,end).to_string()
		}
	}

	pub fn get_edge(&mut self, v1 : u32, v2: u32) -> Option<&Edge> {
		let edge_name = self.edgename(v1,v2);
		self.edge_map.get(&edge_name)
	}

	pub fn edge_exists(&mut self, edge_name : &String) -> bool {
		self.edge_map.contains_key(edge_name)
	}

	pub fn create_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {
		let edge_name = self.edgename(v1,v2);
		if self.edge_exists(&edge_name) {
			None
		}
		else {
			self.add_edge(v1,v2)
		}

	}

	pub fn add_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {

		//get the edgename
		let edge_name = self.edgename(v1,v2);

		//create the vertexes, if the don't exist
		self.create_vertex(&v1);
		self.create_vertex(&v2);

		if self.edge_exists(&edge_name) {
			let e_map = &mut self.edge_map;
			// know what edge exists, since we just checked
			let edge = e_map.get_mut(&edge_name).unwrap();
			edge.incr_cnt();
			None
		}
		// edge doesn't already exists, so create it
		else {


			// create the edge data
			let e = Edge::new(&edge_name,v1,v2);

			// insert the edge into the map by name
			self.edge_map.insert(edge_name.clone(),e);

			// add the edge to the edge list
			self.edge_list.push(edge_name.clone());

			let v_map = &mut self.vertex_map;

			// add the edge to the first vertex's adjanceny list
			let mut vert = v_map.get_mut(&v1); 
			vert.unwrap().add_adjacent(v2);

			// add the edge to the second vertex adjacentcy list
			vert = v_map.get_mut(&v2); 
			vert.unwrap().add_adjacent(v1);

//			let mut v_map2 = v_map.clone();
//			let vert2 = v_map.get_mut(&v2); 

			Some(self.edge_list.len())

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

		g.create_vertex(&vertex);
		for other_v in &adjacent {
			let _num_edges = g.create_edge(vertex,*other_v);
		}
		if _count < 10 {
			println!("{} - Vertex: {} {:?}",_count,vertex,adjacent);
		}
    }
	g.print_vertexes();
	g.print_edges();
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
    fn basic() {
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.create_edge(1,2),Some(1));
		assert_eq!(g.vertex_list,vec!(1,2));
		assert_eq!(g.edge_list,vec!("1_2".to_string()));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.create_edge(1,3),Some(2));
		assert_eq!(g.create_edge(2,3),Some(3));
		assert_eq!(g.vertex_list,vec!(1,2,3));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),"1_3".to_string(),"2_3".to_string()));
		assert_eq!(g.create_edge(1,4),Some(4));
		assert_eq!(g.vertex_list,vec!(1,2,3,4));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),"1_3".to_string(),"2_3".to_string(),"1_4".to_string()));
		println!("{:?}",g);

    }

	#[test]
	fn name() {
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.edgename(1,2),"1_2".to_string()); 
		assert_eq!(g.edgename(3,2),"2_3".to_string()); 
		assert_eq!(g.edgename(10,10),"10_10".to_string()); 
	}

	#[test]
	fn test_add() {
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.create_edge(1,2),Some(1));
		assert!(g.get_edge(2,3).is_none());
		assert_eq!(g.add_edge(1,2),None);
		assert_eq!(g.get_edge(1,2).unwrap().count(),2);
	}



}
