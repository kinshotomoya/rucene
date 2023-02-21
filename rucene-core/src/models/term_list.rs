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

    // TODO: 同じkeyが出てきたら、valueをPostingsListWrapperのpostings_listのlinked listの先頭に追加する
    fn add_posting_list() {}

    fn insert(&mut self, key: String, value: PostingsListWrapper) {
        let entry = Entry::new(Some(key), Some(value), None);
        if self.num_of_children == 0 {
            if self.children.len() != ORDER - 1 {
                // リーフノード&&childrenの数上限では（order - 1）ではない場合
                match self.children.binary_search(&entry) {
                    //  参考：https://stackoverflow.com/questions/36249693/whats-the-most-efficient-way-to-insert-an-element-into-a-sorted-vector
                    Ok(exist_index) => Node::add_posting_list(),
                    Err(new_index) => {
                        self.children.insert(new_index, entry);
                        self.num_of_children = self.children.len() as i32
                    }
                }
            } else {
                // リーフノード&&childrenの数が上限（order）の場合
                // => 分割してbottom upでnodeを作成する必要がある
            }
        } else {
            // TODO:
            //  for文回してkeyの順序を計算しながらleaf nodeまで深く潜る
            self.children.iter_mut().for_each(|a| ());
        }
    }
}
