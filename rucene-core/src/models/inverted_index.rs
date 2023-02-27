use std::collections::HashMap;
use crate::models::postings_list::PostingsList;
use crate::models::term_list::TermList;

// ハッシュマップでドキュメントとポスティングリストのマッピングを実現している
// [重要] 本ではタームリスト自体はRDBで管理している。なので自前でタームリストのb+treeデータ構造を作成・持つ必要はない

// RDBに保存したトークンのID（連番）
type TokenId = i32;

pub struct InvertedIndex {
    index: HashMap<TokenId, PostingsList>, // tokenIdとポスティングリストのハッシュマップ
    total_doc_count: i32 // ドキュメントの総数
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex{
            index: HashMap::new(),
            total_doc_count: 0
        }
    }
}