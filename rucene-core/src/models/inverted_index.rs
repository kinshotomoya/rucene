use crate::models::postings_list::PostingsList;
use std::collections::HashMap;

// ハッシュマップでドキュメントとポスティングリストのマッピングを実現している
// [重要] タームリスト自体はRDBで管理している。なので自前でタームリストのb+treeデータ構造を作成・持つ必要はない
pub struct InvertedIndex<'a> {
    inverted_index: HashMap<DocumentId, PostingsListWrapper<'a>>,
}

pub struct DocumentId(i32);

pub struct PostingsListWrapper<'a> {
    document_count: i32, // 対象単語が出現するドキュメントの総数（linked listで紐付くPostingsList自体の総数）
    positions_count: i32, // 全ドキュメント内での対象の単語が出現する回数（PostingsListのpostings_countの総数）
    postings_list: Box<PostingsList<'a>>, // 対象のポスティングリストへのポインタ
}
