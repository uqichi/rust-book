pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello")
}

// コンパイラにcargo buildを走らせた時ではなく、cargo testを走らせた時にだけ、 テストコードをコンパイルし走らせるよう指示します。
#[cfg(test)]
mod tests {
    // ここではglobを使用して、外部モジュールで定義したもの全てがこのtestsモジュールでも使用可能になるようにしています。
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test f")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[ignore]
    fn it_does_not_work() -> Result<(), String> {
        if 2 + 1 == 4 {
            Ok(())
        } else {
            Err(String::from("2 plus 2 does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn heavy_test() {}
}
