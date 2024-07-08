use apply::Apply;
use std::collections::HashMap;

pub struct Code {
	nodes: HashMap<NodeId, PlacedNode>,
	root: NodeId,
	id_counter: NodeId,
}

pub struct PlacedNode {
	id: NodeId,
	parent: Option<NodeId>,
	pub node: Node,
}

pub enum Node {
	Program(Vec<NodeId>),
	Pixel {
		position: (usize, usize),
		color: Color,
	},
}

pub enum Color {
	Red,
	Green,
	Blue,
}

pub type NodeId = usize;

impl Code {
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new().apply(|mut ns| {
				ns.insert(
					0,
					PlacedNode {
						node: Node::Group(Vec::new()),
						parent: None,
					},
				);
				ns
			}),
			root: 0,
			id_counter: 1,
		}
	}

	pub fn get(&self, node_id: NodeId) -> Option<&PlacedNode> {
		self.nodes.get(&node_id)
	}

	/// Insert a node into a particular node group
	pub fn insert(&mut self, node: Node, group: NodeId) -> Option<NodeId> {
		let id = self.id_counter;
		self.nodes.insert(
			id,
			PlacedNode {
				node,
				parent: Some(group),
			},
		);
		self.nodes.get_mut(&group)?.node.group_mut()?.push(id);
		self.id_counter += 1;
		Some(id)
	}

	pub fn root(&self) -> NodeId {
		self.root
	}
}

impl PlacedNode {
	pub fn parent(&self) -> Option<NodeId> {
		self.parent
	}
}

impl Node {
	pub fn group_mut(&mut self) -> Option<&mut Group> {
		match self {
			Node::Group(ref mut group) => Some(group),
			_ => None,
		}
	}
}
