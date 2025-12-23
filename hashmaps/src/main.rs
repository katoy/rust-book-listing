fn main() {
    // HashMapを使うには std::collections::HashMap をインポート
    use std::collections::HashMap;

    // 方法1: HashMap::new() で空のHashMapを作成
    // キーと値の型は最初のinsertで推論される
    let mut scores = HashMap::new();

    // insert(): キーと値のペアを追加
    // キーが既に存在する場合は値が上書きされる
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // 方法2: 2つのベクタから zip() と collect() で作成
    // zip(): 2つのイテレータをタプルのイテレータに結合
    // collect(): イテレータからコレクションを生成
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    // entry(): キーが存在するかチェックし、Entry enumを返す
    // or_insert(): キーが存在しない場合のみ値を挿入
    // "Yellow"と"Blue"は既に存在するので上書きされない
    // "red"は存在しないので新規挿入される
    scores.entry(String::from("Yellow")).or_insert(90);
    scores.entry(String::from("Blue")).or_insert(91);
    scores.entry(String::from("red")).or_insert(92);
    println!("{:?}", scores);

    // 応用例: 単語の出現回数をカウント
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace(): 空白で文字列を分割
    for word in text.split_whitespace() {
        // entry().or_insert(0): キーがなければ0を挿入
        // 戻り値は値への可変参照(&mut V)
        let count = map.entry(word).or_insert(0);
        // 参照を通じて値をインクリメント
        *count += 1;
    }

    // 結果: {"hello": 1, "world": 2, "wonderful": 1}
    println!("{:?}", map);
}
