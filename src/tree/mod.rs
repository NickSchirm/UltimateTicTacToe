//! # Contains the [Tree] and [Node] structs
//! 
//! The [Tree] struct represents a tree data structure.
//! 
//! The [Node] struct represents a node in a tree data structure.
use std::cell::RefCell;
use std::rc::Rc;

/// # Struct representing a tree data structure
/// 
/// The tree might have a root node.
/// The root node might have children.
/// The number of children is not limited.
pub struct Tree<T: Clone> {
	root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Tree<T> {
	/// Creates a new tree
	/// 
	/// The root of this tree is set to the given node.
	/// 
	/// # Arguments
	/// * `root` - The root node of the tree
	pub fn new(root: Node<T>) -> Self {
		Tree { root: Some(Rc::new(RefCell::new(root))) }
	}

	/// Gets the root of the tree
	/// 
	/// # Returns
	/// The root of the tree
	pub fn get_root(&self) -> Option<Rc<RefCell<Node<T>>>> {
		self.root.clone()
	}
}

/// # Struct representing a node in a tree data structure
/// 
/// The node might have children.
/// The number of children is not limited.
/// 
/// The data that is stored in the node is of type T and must implement the Clone trait.
pub struct Node<T: Clone> {
	data: T,
	children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Node<T> {
	/// Creates a new node
	/// 
	/// # Arguments
	/// * `data` - The data of the node
	pub fn new(data: T) -> Self {
		Node {
			data,
			children: Vec::new(),
		}
	}

	/// Appends a child to the node
	/// 
	/// # Arguments
	/// * `child` - The child to append
	pub fn append(&mut self, child: Node<T>) {
		self.children.push(Rc::new(RefCell::new(child)));
	}

	/// Gets the children of the node
	/// 
	/// # Returns
	/// The children of the node
	pub fn get_children(&self) -> &Vec<Rc<RefCell<Node<T>>>> {
		&self.children
	}

	/// Gets the data of the node
	/// 
	/// # Returns
	/// The data of the node
	pub fn get_data(&self) -> T {
		self.data.clone()
	}

	/// Checks if the node is a leaf
	/// 
	/// # Returns
	/// True if the node is a leaf, false otherwise
	pub fn is_leaf(&self) -> bool {
		self.children.is_empty()
	}

	/// Maps a function over the data of the node
	/// 
	/// # Arguments
	/// * `f` - The function to map over the data
	pub fn map<F>(&mut self, f: F) where F: Fn(T) -> T {
		self.data = f(self.data.clone());
	}
}