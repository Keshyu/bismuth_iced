use std::collections::HashMap;

pub struct Code {
	nodes: HashMap<NodeId, Node>,
	root: NodeId,
	id_counter: NodeId,
}

pub enum Node {
	Word(String),
	Group(Group),
}

pub type Group = Vec<NodeId>;

pub type NodeId = usize;

impl Code {
	pub fn new() -> Self {
		let mut nodes = HashMap::new();
		nodes.insert(0, Node::Group(Vec::new()));
		Self {
			nodes,
			root: 0,
			id_counter: 1,
		}
	}

	pub fn get(&self, node_id: NodeId) -> Option<&Node> {
		self.nodes.get(&node_id)
	}

	/// Insert a node into a particular node group
	pub fn insert(&mut self, node: Node, group: NodeId) -> Option<NodeId> {
		let id = self.id_counter;
		self.nodes.insert(id, node);
		self.nodes.get_mut(&group)?.group_mut()?.push(id);
		self.id_counter += 1;
		Some(id)
	}

	pub fn root(&self) -> NodeId {
		self.root
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
