pub trait Draw {
    fn draw(&self);
}

// Drawトレイトを実装するトレイトオブジェクトのベクタを保持するcomponentsフィールドがある Screen構造体の定義
pub struct Screen {
    // Box<Draw>はトレイトオブジェクトです
    // トレイトオブジェクトは、 指定したトレイトを実装するある型のインスタンスを指します。
    // トレイトオブジェクトは、ダイナミックディスパッチを行う
    // ダイナミックディスパッチの場合、コンパイラは、どのメソッドを呼び出すか実行時に弾き出すコードを生成します。
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// トレイト境界を伴うジェネリックな型引数を使用する構造体を定義
// ジェネリックな型引数は、一度に1つの具体型にしか置き換えられないのに対して、トレイトオブジェクトは、 実行時にトレイトオブジェクトに対して複数の具体型で埋めることができます
// こうすると、全てのコンポーネントの型がButtonだったり、TextFieldだったりするScreenのインスタンスに制限されてしまいます。 絶対に同種のコレクションしか持つ予定がないのなら、ジェネリクスとトレイト境界は、 定義がコンパイル時に具体的な型を使用するように単相化されるので、望ましいです。
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// impl<T> Screen<T> {
//     where T: Draw {
//         pub fn run(&self) {
//             for component in self.components.iter() {
//                 component.draw();
//             }
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}
