# rucene

Rust製の自作検索エンジン

参考：https://gihyo.jp/book/2014/978-4-7741-6753-4

## 構成
基本的にインデックスはメモリ上で持つようにする

### インデックス
メモリ上に全て持たせる

### ドキュメント自体
RDBに保存する
RDBに保存したDocumentIDをインデックスのポスティングリストに持たせる


タームリストのデータ構造イメージ
![term list image](/images/btree.jpg)
