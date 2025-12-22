fn main() {
    // &str（文字列スライス）: 不変の参照、スタック上に格納
    let data: &str = "initial contents";
    println!("{}", data);

    // 方法1: to_string() メソッドで &str から String に変換
    let s: String = data.to_string();
    println!("{}", s);

    // 方法2: リテラルから直接 to_string() を呼び出す
    let s: String = "initial contents".to_string();
    println!("{}", s);

    // 方法3: String::from() 関数を使用
    let s: String = String::from("initial contents");
    println!("{}", s);

    // RustのStringはUTF-8エンコーディング
    // 様々な言語の文字列を格納できる
    let _hello = String::from("السلام عليكم"); // アラビア語
    let _hello = String::from("Dobrý den"); // チェコ語
    let _hello = String::from("Hello"); // 英語
    let _hello = String::from("שָׁלוֹם"); // ヘブライ語
    let _hello = String::from("नमस्ते"); // ヒンディー語
    let _hello = String::from("こんにちは"); // 日本語
    let _hello = String::from("안녕하세요"); // 韓国語
    let _hello = String::from("你好"); // 中国語
    let _hello = String::from("Olá"); // ポルトガル語
    let _hello = String::from("Здравствуйте"); // ロシア語
    let _hello = String::from("Hola"); // スペイン語

    // push_str(): 文字列スライス(&str)を追加するメソッド
    // s2の所有権は移動しないため、後で使用可能
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s1は"foobar"になる
    println!("s2 is {}", s2); // s2はまだ使用可能

    // + 演算子による文字列連結
    // 内部的には fn add(self, s: &str) -> String を呼び出す
    // - 左辺(s1)の所有権がムーブされる
    // - 右辺は参照(&String → &str に自動変換: deref coercion)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // s1はムーブされ使用不可、s2は参照なので使用可能

    // + 演算子を複数回使用した連結
    // 注意: s1の所有権がムーブされる（s2, s3は参照なので使用可能）
    // 複雑になると読みにくくなる
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // 結果: "tic-tac-toe"
    println!("{}", s);

    // format!マクロによる連結（推奨）
    // - println!と同じ書式指定が使える
    // - 所有権を奪わない（すべての変数が後でも使用可能）
    // - 可読性が高い
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3); // 結果: "tic-tac-toe"
    println!("{}", s);
}
