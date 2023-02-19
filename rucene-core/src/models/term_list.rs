use crate::models::inverted_index::PostingsListWrapper;
use std::collections::HashMap;
// btreeで実装（完成後、b+treeでも作ってみる）
// 参考：https://www.javatips.net/api/algs4-master/src/main/java/edu/princeton/cs/algs4/BTree.java

// 一旦orderは4で作成してみる
// 本来ならディスクのブロックサイズを考慮したorder数にすべきだがまたの機会に調整
static ORDER: usize = 4;

pub struct TermList {
    root: Node,
    height: i32,
}

impl TermList {
    fn init() -> TermList {
        TermList {
            root: Node::new(),
            height: 0,
        }
    }

    fn insert(&mut self, term_list: HashMap<String, PostingsListWrapper>) -> TermList {
        // self.root.insert()
        todo!()
    }
}

// key value childrenが存在しないEntryを作成することもあるので、Optionで定義
pub struct Entry {
    key: Option<String>,
    value: Option<PostingsListWrapper>,
    next: Option<Box<Node>>,
}

impl Entry {
    fn new(
        key: Option<String>,
        value: Option<PostingsListWrapper>,
        next: Option<Box<Node>>,
    ) -> Entry {
        Entry { key, value, next }
    }
}

pub struct Node {
    num_of_children: i32,
    children: Vec<Entry>,
}

impl Node {
    fn new() -> Node {
        Node {
            num_of_children: 0,
            children: Vec::with_capacity(ORDER),
        }
    }

    fn insert(&mut self, key: String, value: PostingsListWrapper) {
        let entry = Entry::new(Some(key), Some(value), None);
        if self.num_of_children == 0 {
            self.children.push(entry)
        } else {
            // TODO:
            //  for文回してkeyの順序を計算しながらleaf nodeまで深く潜る

            self.children.iter_mut().for_each(|a| ());
        }
    }
}
