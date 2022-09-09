/*
Rustはデフォルトで、標準ライブラリで定義されているアイテムの中のいくつかを、すべてのプログラムのスコープに取り込みます。 このセットはprelude（プレリュード）と呼ばれ、標準ライブラリのドキュメントでその中のすべてを見ることができます。
使いたい型がpreludeにない場合は、その型をuse文で明示的にスコープに入れる必要があります。 std::ioライブラリをuseすると、ユーザ入力を受け付ける機能など（入出力に関する）多くの便利な機能が利用できるようになります。
*/
use std::io;
// Rngトレイトは乱数生成器が実装すべきメソッドを定義しており、それらのメソッドを使用するには、このトレイトがスコープ内になければなりません。
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // rand::thread_rng関数を呼び出して、これから使う、ある特定の乱数生成器を取得しています。 なお、この乱数生成器は現在のスレッドに固有で、オペレーティングシステムからシード値を得ています。
    // そして、この乱数生成器のgen_rangeメソッドを呼び出しています。 このメソッドはuse rand::Rng文でスコープに導入したRngトレイトで定義されています。
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        // newがString型の関連関数であることを示しています
        let mut guess = String::new();

        io::stdin()
            // 変数のように参照もデフォルトで不変であることです。 したがって、&guessではなく&mut guessと書いて可変にする必要があります
            .read_line(&mut guess)
            // 同時に値（この場合はio::Result）も返します。 Result型の目的は、エラー処理に関わる情報を符号化（エンコード）することです
            .expect("Failed to read line");

        // シャドーイング（shadowing）は、guess_strとguessのような重複しない変数を二つ作る代わりに、guessという変数名を再利用させてくれるのです。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match式は複数のアーム（腕）で構成されます。
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
