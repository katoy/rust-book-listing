# Rust Book 学習プロジェクト

Rust プログラミング言語の公式チュートリアル「The Rust Programming Language」の学習用リポジトリです。

## プロジェクト一覧

| プロジェクト | 説明 | テスト | カバレッジ |
| --- | --- | --- | --- |
| [guessing_game](./guessing_game/) | 数当てゲーム（テスト可能版） | 19 passed | 88.10% |
| [guessing_game_simple](./guessing_game_simple/) | 数当てゲーム（簡易版） | 10 passed | - |

### guessing_game vs guessing_game_simple

| 項目 | guessing_game | guessing_game_simple |
| --- | --- | --- |
| API | `run_game()` + 内部ジェネリック関数 | `run_game()` のみ |
| テスト容易性 | 高い（モック入出力可能） | 低い（純粋関数のみ） |
| シンプルさ | やや複雑 | シンプル |

## 使い方

各プロジェクトのディレクトリに移動して実行:

```bash
cd guessing_game
cargo run
```

または

```bash
cd guessing_game_simple
cargo run
```

## 開発

### テスト実行

```bash
cd guessing_game && cargo test
cd guessing_game_simple && cargo test
```

### コードチェック

```bash
cargo clippy -- -W clippy::pedantic -D warnings
```

## ライセンス

[MIT License](./LICENSE)
