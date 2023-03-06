// ポスティングリストはlinked listデータ構造で実現する
pub struct Posting {
    document_id: i32,        // ドキュメントID（ドキュメント自体もRDBで管理）
    postings: Vec<i32>,      // 対象の単語がドキュメントに現れるオフセットのリスト
    postings_count: i32,     // 対象の単語がドキュメントに何個現れるかの数（↑オフセットリストの数）
}

impl Posting {
    fn new(document_id: i32, postings: Vec<i32>, postings_count: i32) -> Posting {
        Posting {
            document_id,
            postings,
            postings_count
        }
    }
}