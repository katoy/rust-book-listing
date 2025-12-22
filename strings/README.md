# Strings

Rustにおける文字列（`String`と`&str`）の基本的な使い方を学ぶサンプルプロジェクトです。

## 目次

- [Strings](#strings)
  - [目次](#目次)
  - [学習内容](#学習内容)
    - [1. 文字列の種類](#1-文字列の種類)
    - [2. Stringの作成方法](#2-stringの作成方法)
    - [3. UTF-8エンコーディング](#3-utf-8エンコーディング)
    - [4. 文字列の追加](#4-文字列の追加)
    - [5. 文字列の連結](#5-文字列の連結)
      - [`+` 演算子](#-演算子)
      - [`format!` マクロ（推奨）](#format-マクロ推奨)
  - [実行方法](#実行方法)
  - [参考](#参考)

## 学習内容

### 1. 文字列の種類

| 型 | 説明 |
|---|---|
| `&str` | 文字列スライス。不変の参照でスタック上に格納 |
| `String` | ヒープに格納される可変の文字列型 |

### 2. Stringの作成方法

```rust
// 方法1: to_string() メソッド
let s = "hello".to_string();

// 方法2: String::from() 関数
let s = String::from("hello");
```

### 3. UTF-8エンコーディング

RustのStringはUTF-8エンコーディングを使用するため、多言語の文字列を格納できます：

```rust
let hello = String::from("こんにちは");  // 日本語
let hello = String::from("안녕하세요");   // 韓国語
let hello = String::from("你好");        // 中国語
```

### 4. 文字列の追加

```rust
let mut s = String::from("foo");
s.push_str("bar");  // s は "foobar" になる
```

### 5. 文字列の連結

#### `+` 演算子

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 はムーブされ使用不可
```

#### `format!` マクロ（推奨）

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);  // 所有権を奪わない
```

## 実行方法

```bash
cargo run
```

## 参考

- [The Rust Programming Language - Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
