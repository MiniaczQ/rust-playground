use std::collections::HashSet;

use crate::genarea::{Area, IdxGen};

#[derive(Debug)]
pub struct Node<T> {
    ingoing: HashSet<IdxGen>,
    outgoing: HashSet<IdxGen>,
    data: T,
}

#[derive(Debug, Default)]
pub struct DirectedGraph<T> {
    nodes: Area<Node<T>>,
}

impl<T> DirectedGraph<T> {
    pub fn add_node() {}
    pub fn add_edge() {}
}
