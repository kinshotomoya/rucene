// ポスティングリストはlinked listデータ構造で実現する
pub struct Posting {
    document_id: i32,        // ドキュメントID（ドキュメント自体もRDBで管理）
    postings: [i32],      // 対象の単語がドキュメントに現れるオフセットのリスト
    postings_count: i32,     // 対象の単語がドキュメントに何個現れるかの数（↑オフセットリストの数）
}

impl PostingsList {
    fn new(document_id: i32, postings: Box<[i32]>, postings_count: i32) -> PostingsList {
        Posting {
            document_id,
            postings: *postings,
            postings_count
        }
    }
}