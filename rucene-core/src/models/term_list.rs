use crate::models::inverted_index::PostingsListWrapper;

// TODO: b+treeで実装
pub struct TermList {
    root: Node
}

pub struct Node {
    term: String,
    postings_list: PostingsListWrapper,
    next: Box<Node>,
    previous: Box<Node>
}