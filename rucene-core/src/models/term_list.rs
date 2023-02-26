use crate::models::inverted_index::PostingsListWrapper;
use std::arch::x86_64::_mm256_insert_epi16;
use std::cmp::Ordering;
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
#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    key: Option<String>,
    value: Option<PostingsListWrapper>,
    next: Option<Box<Node>>,
}

// sort()メソッド内で必要
// 自前のstructなど比較できないかもしれない型同士を比較する際に、このPartialOrd.cmp()メソッドが利用される
impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.key.is_none() {
            return Some(Ordering::Greater);
        }
        if other.key.is_none() {
            return Some(Ordering::Less);
        }
        Some(self.key.cmp(&other.key))
    }
}

// binary_searchメソッド内で必要
// sortされている前提なのでPartialOrdではなく、Ordのcpm()メソッドが使われる
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // key = Noneの場合は、Vecの最後尾に配置したいので↓のような実装にしている
        if self.key.is_none() {
            return Ordering::Greater;
        }
        if other.key.is_none() {
            return Ordering::Less;
        }
        self.key.cmp(&other.key)
    }
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

    fn add_posting_list() {}

    fn insert(&mut self, key: String, value: PostingsListWrapper) {
        fn inner_loop(insert_key: &str, node: &mut Node, new_entry: Entry) {
            for entry in node.children.iter_mut() {
                if node.num_of_children == 0 {
                    match node.children.binary_search(&new_entry) {
                        //  参考：https://stackoverflow.com/questions/36249693/whats-the-most-efficient-way-to-insert-an-element-into-a-sorted-vector
                        Ok(exist_index) => {
                            // TODO: 同じkeyが出てきたら、valueをPostingsListWrapperのpostings_listのlinked listの先頭に追加する
                            Node::add_posting_list()
                        },
                        Err(new_index) => {
                            // TODO: cloneやめたいがどうしようか
                            node.children.insert(new_index, new_entry.clone());
                            node.num_of_children = node.children.len() as i32;
                        }
                    }
                    break;
                } else {
                    // TODO: 続き、子ノードが存在する場合
                    match &entry.key {
                        Some(key_str) => {
                            if insert_key < key_str  {
                                match &entry.next {
                                    Some(mut node) => {
                                        // TODO: cloneやめたいがどうしようか
                                        inner_loop(insert_key,  &mut node, new_entry.clone())
                                    },
                                    None => {
                                        // リーフノードの場合
                                    }
                                }
                            } else {
                                continue
                            }
                        },
                        None => {
                            if entry.next.is_some() {
                                // こノードが存在する場合
                            } else {
                                // リーフノードの場合
                            }
                        }
                    }
                }
            }
        }
        let new_entry = Entry::new(Some(key), Some(value), None);
        inner_loop(key.as_str(), &mut self, new_entry);
    }
}
