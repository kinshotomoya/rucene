// ポスティングリストはlinked listデータ構造で実現する
pub struct PostingsList<'a> {
    document_id: i32,            // ドキュメントID
    postings: &'a [i32],         // 対象の単語がドキュメントに現れるオフセットのリスト
    postings_count: i32, // 対象の単語がドキュメントに何個現れるかの数（↑オフセットリストの数）
    next: Box<PostingsList<'a>>, // 次のポスティングリストへのポインタ
}
