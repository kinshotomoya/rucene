use std::collections::{HashMap, LinkedList};
use crate::models::postings_list::Posting;
use crate::models::tokenizer::CustomTokenizer;
use crate::repositories::mysql_client::MysqlClient;

// ハッシュマップでドキュメントとポスティングリストのマッピングを実現している
// [重要] 本ではタームリスト自体はRDBで管理している。なので自前でタームリストのb+treeデータ構造を作成・持つ必要はない

// RDBに保存したトークンのID（連番）
type TokenId = i32;

pub struct InvertedIndex {
    index: HashMap<TokenId, LinkedList<Posting>>, // tokenIdとポスティングリストのハッシュマップ
    tokenizer: CustomTokenizer,
    total_doc_count: i32 // ドキュメントの総数
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex{
            index: HashMap::new(),
            tokenizer: CustomTokenizer::new(),
            total_doc_count: 0
        }
    }

    fn add_document(self, title: &str, content: &str) {
        // 日本語形態素解析で形態素分解する
        let titleTokens = self.tokenizer.tokenizer.tokenize(title)?;
        let contentTokens = self.tokenizer.tokenizer.tokenize(content)?;

        // TODO:
        // トークンをmysqlに保存する（すでに同じトークンがあればそのtokenIdを取得する）

        // ドキュメントをmysqlに保存する
        // tokenid、docidから転置インデックスを作成する

        todo!()
    }

}