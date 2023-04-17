// 入出力ライブラリ
use std::io;
// 乱数
use rand::Rng;
// 比較
use std::cmp::Ordering;

fn main() {

    // 不変
    let secret_number = rand::thread_rng().gen_range(1..101); // 1から100

    // println!("秘密の数は {} です", secret_number);

    loop {

        println!("数値を入力してください");

        // let, 変数宣言
        // mut, 可変
        let mut guess = String::new(); // 関連関数, 型に対して実装される関数

        io::stdin()
            .read_line(&mut guess)// 参照渡し. mutで可変にする
            .expect("入力を読み込めませんでした"); // ResultがErrの場合メッセージ表示してプログラム終了

        // シャドーイング, 同じ変数名で覆い隠す
        let guess: u32 = match guess.trim().parse() {
            // parseはResult型を返す. ResultはOkとErrの列挙型
            Ok(num) => num,
            Err(_) => {
                println!("数値ではありません");
                continue
            }
        };

        println!("入力された数は {} です", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さいです"),
            Ordering::Greater => println!("大きいです"),
            Ordering::Equal => {
                println!("正解!");
                break;
            }
        }
    }
}
