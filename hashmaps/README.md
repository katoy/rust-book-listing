# HashMaps

RustにおけるHashMap（ハッシュマップ）の基本的な使い方を学ぶサンプルプロジェクトです。

## 目次

- [学習内容](#学習内容)
  - [1. HashMapの作成](#1-hashmapの作成)
  - [2. 値の挿入](#2-値の挿入)
  - [3. ベクタからHashMapを作成](#3-ベクタからhashmapを作成)
  - [4. entry と or_insert](#4-entry-と-or_insert)
  - [5. 応用例：単語カウント](#5-応用例単語カウント)
- [実行方法](#実行方法)
- [参考](#参考)

## 学習内容

### 1. HashMapの作成

```rust
use std::collections::HashMap;

// HashMap::new() で空のHashMapを作成
let mut scores = HashMap::new();
```

- `HashMap`は`std::collections`モジュールにある
- キーと値の型は最初の`insert`で推論される

### 2. 値の挿入

```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- `insert()`でキーと値のペアを追加
- 同じキーに再度`insert`すると値が上書きされる

### 3. ベクタからHashMapを作成

```rust
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

| メソッド | 説明 |
|---------|------|
| `zip()` | 2つのイテレータをタプルのイテレータに結合 |
| `collect()` | イテレータからコレクションを生成 |

### 4. entry と or_insert

```rust
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
scores.entry(String::from("Red")).or_insert(92);  // 新規挿入
```

| メソッド | 説明 |
|---------|------|
| `entry()` | キーが存在するかチェックし、`Entry` enumを返す |
| `or_insert()` | キーが存在しない場合のみ値を挿入 |

- 既存のキーには何もしない
- 新しいキーには値を挿入

### 5. 応用例：単語カウント

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
// 結果: {"hello": 1, "world": 2, "wonderful": 1}
```

- `or_insert()`は値への可変参照（`&mut V`）を返す
- 参照を通じて値を更新できる

## 実行方法

```bash
cargo run
```

## 参考

- [The Rust Programming Language 日本語版 - HashMapでキーとそれに紐づいた値を保持する](https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html)
