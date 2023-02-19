use crate::models::postings_list::PostingsList;
use crate::models::term_list::TermList;

// ハッシュマップでドキュメントとポスティングリストのマッピングを実現している
// [重要] 本ではタームリスト自体はRDBで管理している。なので自前でタームリストのb+treeデータ構造を作成・持つ必要はないが
// 今回は深堀りのためにタームリストもRDB使ってディスクで管理するのではなく、メモリ上でb+treeで持たせる！
pub struct InvertedIndex {
    term_list: TermList,
}

pub struct PostingsListWrapper {
    document_count: i32, // 対象単語が出現するドキュメントの総数（linked listで紐付くPostingsList自体の総数）
    positions_count: i32, // 全ドキュメント内での対象の単語が出現する回数（PostingsListのpostings_countの総数）
    postings_list: Box<PostingsList>, // 対象のポスティングリストへのポインタ
}
