# 数当てゲーム (Guessing Game)

Rust Book のチュートリアルに基づいた数当てゲームの実装です。

## 目次

- [機能](#機能)
- [プロジェクト構造](#プロジェクト構造)
- [使い方](#使い方)
- [開発](#開発)
- [API](#api)
- [テスト項目・結果](#テスト項目結果)
- [ライセンス](#ライセンス)

## 機能

- 1〜100の範囲で数当てゲームを実行
- ユーザーからの数値入力を受け付け
- 入力値のバリデーション（範囲外・数値以外はエラーメッセージを表示）
- 大小のヒント表示（「もっと大きいで！」「もっと小さいで！」）
- I/O エラーハンドリング（`Result` 型による適切なエラー処理）
- 大阪弁による親しみやすいメッセージ

## プロジェクト構造

```text
guessing_game/
├── Cargo.toml       # プロジェクト設定
├── LICENSE          # MIT ライセンス
├── README.md        # このファイル
├── src/
│   ├── main.rs      # エントリーポイント
│   └── lib.rs       # ゲームロジック
└── tests/
    └── test_lib.rs  # 統合テスト
```

## 使い方

### ビルド

```bash
cargo build
```

### 実行

```bash
cargo run
```

## 開発

### コードチェック (Clippy)

```bash
cargo clippy -- -W clippy::pedantic -D warnings
```

### テスト

```bash
cargo test
```

### カバレッジ計測

```bash
cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'
```

## API

### `run_game`

```rust
pub fn run_game() -> io::Result<()>
```

ゲームのメインロジック。標準入出力を使用するシンプルなエントリーポイント。

### `run_game_internal`

```rust
pub fn run_game_internal<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<()>
```

ジェネリック入出力を使用するゲームロジック（テスト用）。

### `run_game_with_secret`

```rust
pub fn run_game_with_secret<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    secret_number: u32,
) -> io::Result<()>
```

秘密の数字を指定してゲームを実行（テスト用）。

### `parse_guess`

```rust
#[must_use]
pub fn parse_guess(input: &str) -> Option<u32>
```

入力文字列を1〜100の範囲の数値に変換。

### `config` モジュール

```rust
pub const MIN_NUMBER: u32 = 1;   // 最小値
pub const MAX_NUMBER: u32 = 100; // 最大値
```

## テスト項目・結果

### テストケース一覧（19テスト）

| カテゴリ | テスト数 | 説明 |
| --- | --- | --- |
| parse_guess | 5 | 有効値、空白、無効入力、負数、範囲外 |
| run_game_internal | 4 | 正常入力、無効入力、EOF、エラー処理 |
| run_game_with_secret | 3 | 大きい/小さい/正解 |
| I/O エラー | 7 | 各種書き込み/読み込みエラー |

**テスト実行結果:** 19 passed, 0 failed

### Clippy 結果

```text
$ cargo clippy -- -W clippy::pedantic -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**結果:** ✅ 警告なし（pedantic レベル）

### カバレッジ結果

```text
$ cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'

Filename   Regions  Cover    Functions  Executed  Lines   Cover
----------------------------------------------------------------
lib.rs          78  78.21%           5    80.00%     42  88.10%
----------------------------------------------------------------
TOTAL           78  78.21%           5    80.00%     42  88.10%
```

> **Note:** Lines カバレッジ **88.10%**。`run_game()` はテストから呼び出せないため未カバー。

## ライセンス

[MIT License](../LICENSE)
