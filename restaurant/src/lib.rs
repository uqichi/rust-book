// front_of_houseモジュールを宣言する。その中身はsrc/front_of_house.rs内にある
// mod front_of_houseの後にブロックではなくセミコロンを使うと、Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令します。
mod front_of_house;

/*
相対パスを使うか絶対パスを使うかは、プロジェクトによって決めましょう。
要素を定義するコードを、その要素を使うコードと別々に動かすか一緒に動かすか、
どちらが起こりそうかによって決めるのが良いです。
例えば、front_of_houseモジュールとeat_at_restaurant関数をcustomer_experience
というモジュールに移動させると、add_to_waitlistへの絶対パスを更新しないといけませんが、
相対パスは有効なままです。 しかし、eat_at_restaurant関数だけをdining
というモジュールに移動させると、add_to_waitlistへの絶対パスは同じままですが、
相対パスは更新しないといけないでしょう。
コードの定義と、その要素の呼び出しは独立に動かしそうなので、絶対パスのほうが好ましいです。
 */

/*
親モジュールの要素は子モジュールの非公開要素を使えませんが、
子モジュールの要素はその祖先モジュールの要素を使えます。
これは、子モジュールは実装の詳細を覆い隠しますが、
子モジュールは自分の定義された文脈を見ることができるためです。
 */

// 絶対パスで要素をスコープに持ち込む
use crate::front_of_house::hosting;
// 相対パスで要素をスコープに持ち込む
// use self::front_of_house::hosting;

// 関数をスコープにuseで持ち込む
// use crate::front_of_house::hosting::add_to_waitlist;
// このやりかたは慣例的ではない。 一方で、構造体やenumその他の要素をuseで持ち込むときは、フルパスを書くのが慣例的です。
// こちらの慣例の背後には、はっきりとした理由はありません。自然に発生した慣習であり、
// みんなRustのコードをこのやり方で読み書きするのに慣れてしまったというだけです。

// 新しい名前をasキーワードで与える
// use std::fmt::Result;
// use std::io::Result as IoResult;

// pub useを使って名前を再公開(re-exporting)する
// pub use crate::front_of_house::hosting;
// pub useを使うことで、外部のコードがhosting::add_to_waitlistを使ってadd_to_waitlist関数を呼び出せるようになりました。 pub useを使っていなければ、eat_at_restaurant関数はhosting::add_to_waitlistを自らのスコープ内で使えるものの、外部のコードはこの新しいパスを利用することはできないでしょう。
// 再公開は、あなたのコードの内部構造と、あなたのコードを呼び出すプログラマーたちのその領域に関しての見方が異なるときに有用です。 例えば、レストランの比喩では、レストランを経営している人は「接客部門 (front of house)」と「後方部門 (back of house)」のことについて考えるでしょう。 しかし、レストランを訪れるお客さんは、そのような観点からレストランの部門について考えることはありません。 pub useを使うことで、ある構造でコードを書きつつも、別の構造で公開するということが可能になります。 こうすることで、私達のライブラリを、ライブラリを開発するプログラマにとっても、ライブラリを呼び出すプログラマにとっても、よく整理されたものとすることができます。

// 巨大なuseのリストをネストしたパスを使って整理する
// use std::{cmp::Ordering, io};
// この行は std::io とstd::io::Write をスコープに持ち込みます。
// use std::io::{self, Write};

// glob演算子
// use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    // crateという名前を使ってクレートルートからスタートするというのは、/を使ってファイルシステムのルートからスタートするようなものです。
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // useキーワードでパスをスコープに持ち込む
    // ファイルシステムにおいてシンボリックリンクを張ることに似ています。
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summber("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile; we're not allowed to see or modify the seasonal fruit
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Parent path
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summber(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumを公開すると、そのヴァリアントはすべて公開されます。 pubはenumキーワードの前にだけおけばよいのです。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
