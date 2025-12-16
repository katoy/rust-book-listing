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
  - [テスト項目・結果](#テスト項目結果)
    - [テストケース一覧](#テストケース一覧)
    - [Clippy 結果](#clippy-結果)
    - [カバレッジ結果](#カバレッジ結果)
  - [ライセンス](#ライセンス)

## 機能

- ユーザーからの数値入力を受け付け
- 入力値のバリデーション（数値以外はエラーメッセージを表示）
- I/O エラーハンドリング（`Result` 型による適切なエラー処理）

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
cargo clippy -- -W clippy::all
```

すべての警告をエラーとして扱う場合:

```bash
cargo clippy -- -D warnings
```

### テスト

統合テストを実行:

```bash
cargo test
```

詳細な出力付きで実行:

```bash
cargo test -- --nocapture
```

### カバレッジ計測

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) を使用してコードカバレッジを計測します。

**インストール:**

```bash
cargo install cargo-llvm-cov
```

**サマリーのみ表示:**

```bash
cargo llvm-cov --summary-only
```

**詳細表示（未実行行を確認）:**

```bash
cargo llvm-cov --text
```

**main.rs を除外して計測（推奨）:**

`main.rs` はエントリーポイントでテストから呼び出せないため、除外して計測することを推奨します:

```bash
cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'
```

**HTML レポート生成:**

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## テスト項目・結果

### テストケース一覧

| テスト名 | 説明 | 結果 |
| --- | --- | --- |
| `test_parse_guess_valid_number` | 有効な数値（42, 1, 100）のパース | ✅ OK |
| `test_parse_guess_with_whitespace` | 空白・改行を含む入力のトリム処理 | ✅ OK |
| `test_parse_guess_invalid_input` | 無効な入力（文字列、空、小数）の処理 | ✅ OK |
| `test_parse_guess_negative_number` | 負の数（u32 なので None）の処理 | ✅ OK |
| `test_run_game_valid_input` | 正常な数値入力時のゲームフロー | ✅ OK |
| `test_run_game_invalid_input` | 無効な入力時のエラーメッセージ表示 | ✅ OK |
| `test_run_game_write_error_first_writeln` | 最初の writeln でのI/Oエラー処理 | ✅ OK |
| `test_run_game_write_error_second_writeln` | 2番目の writeln でのI/Oエラー処理 | ✅ OK |
| `test_run_game_read_error` | read_line でのI/Oエラー処理 | ✅ OK |
| `test_run_game_write_error_after_read` | 結果表示（有効入力）でのI/Oエラー処理 | ✅ OK |
| `test_run_game_write_error_on_invalid_input_message` | 結果表示（無効入力）でのI/Oエラー処理 | ✅ OK |

**テスト実行結果:** 11 passed, 0 failed

### Clippy 結果

```text
$ cargo clippy -- -W clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**結果:** ✅ 警告なし

### カバレッジ結果

```text
$ cargo llvm-cov --summary-only

Filename   Regions  Cover    Functions  Executed  Lines   Cover
----------------------------------------------------------------
lib.rs          32  90.62%           2   100.00%     13  100.00%
main.rs         14   0.00%           1     0.00%      8    0.00%
----------------------------------------------------------------
TOTAL           46  63.04%           3    66.67%     21   61.90%
```

> **Note:** `main.rs` が 0% なのは、`cargo test` ではバイナリの `main()` 関数が実行されないためです
> （一般的な動作）。統合テスト (`tests/test_lib.rs`) はカバレッジ対象に含まれません。

**main.rs を除外した場合:**

```text
$ cargo llvm-cov --summary-only --ignore-filename-regex 'main\.rs'

Filename   Regions  Cover    Functions  Executed  Lines   Cover
----------------------------------------------------------------
lib.rs          32  90.62%           2   100.00%     13  100.00%
----------------------------------------------------------------
TOTAL           32  90.62%           2   100.00%     13  100.00%
```

> **Note:** Lines カバレッジは **100%** です。Regions が 100% でないのは、`?` 演算子の
> 内部実装詳細（llvm-cov のカウント方法）に起因し、実用上は問題ありません。

## ライセンス

[MIT License](../LICENSE)
