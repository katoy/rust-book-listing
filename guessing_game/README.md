# 数当てゲーム (Guessing Game)

Rust Book のチュートリアルに基づいた数当てゲームの実装です。

## 目次

- [数当てゲーム (Guessing Game)](#数当てゲーム-guessing-game)
  - [目次](#目次)
  - [機能](#機能)
  - [プロジェクト構造](#プロジェクト構造)
  - [使い方](#使い方)
    - [ビルド](#ビルド)
    - [実行](#実行)
  - [開発](#開発)
    - [コードチェック (Clippy)](#コードチェック-clippy)
    - [テスト](#テスト)
    - [カバレッジ計測](#カバレッジ計測)
      - [インストール](#インストール)
      - [サマリーのみ表示](#サマリーのみ表示)
      - [main.rs を除外して計測（推奨）](#mainrs-を除外して計測推奨)
      - [HTML レポート生成](#html-レポート生成)
  - [ライセンス](#ライセンス)

## 機能

- ユーザーからの数値入力を受け付け
- 入力値のバリデーション（数値以外はエラーメッセージを表示）

## プロジェクト構造

```text
guessing_game/
├── Cargo.toml       # プロジェクト設定
├── README.md        # このファイル
└── src/
    ├── main.rs      # エントリーポイント
    ├── lib.rs       # ゲームロジック
    └── tests.rs     # ユニットテスト
```

## 使い方

### ビルド

```bash
cargo build
```

リリースビルド:

```bash
cargo build --release
```

### 実行

```bash
cargo run
```

## 開発

### コードチェック (Clippy)

Clippy を使用してコードの品質をチェックします:

```bash
cargo clippy
```

すべての警告をエラーとして扱う場合:

```bash
cargo clippy -- -D warnings
```

### テスト

ユニットテストを実行:

```bash
cargo test
```

詳細な出力付きで実行:

```bash
cargo test -- --nocapture
```

### カバレッジ計測

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) を使用してコードカバレッジを計測します。

#### インストール

```bash
cargo install cargo-llvm-cov
```

#### サマリーのみ表示

```bash
cargo llvm-cov --summary-only
```

#### main.rs を除外して計測（推奨）

`main.rs` はエントリーポイントでテストから呼び出せないため、除外して計測することを推奨します:

```bash
cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'
```

期待される出力:

```text
Filename       Regions    Cover   Functions  Executed   Lines    Cover
-----------------------------------------------------------------------
lib.rs              32   100.00%          2   100.00%      12   100.00%
tests.rs            79   100.00%          6   100.00%      37   100.00%
-----------------------------------------------------------------------
TOTAL              111   100.00%          8   100.00%      49   100.00%
```

#### HTML レポート生成

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## ライセンス

MIT License
