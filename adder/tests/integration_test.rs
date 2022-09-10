// Cargoはtestsディレクトリを特別に扱い、
// cargo testを走らせた時にのみこのディレクトリのファイルをコンパイルするのです。

// testsディレクトリのテストはそれぞれ個別のクレートであるため、 各々ライブラリをインポートする必要があるためです。
extern crate adder;

#[test]
fn it_adds() {
    assert_eq!(4, adder::add(2, 2));
}
