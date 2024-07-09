// unit tests // one func
// integration tests // scope tests
// system tests // test with other part of services
// e2e tests // test scenario
// smoke tests // test base functional
// TDD // firstly test then coding
// red-green-refactor // test -> code -> refactor

fn main() {
    let output = str_data();
    println!("{output}");
}

fn str_data() -> &'static str {
    "Hello world !"
}

fn boom() {
    eprintln!("errorr");
    panic!("boom !");
}
#[cfg(test)] // unit test will not in release
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_returns_panic() {
        boom();
    }

    #[test]
    #[should_panic(expected = "Bon!")]
    fn it_returns_panic_bon() {
        boom();
    }

    #[test]
    fn it_returns_true() {
        // assert!(false);
        assert_eq!(true, true);
    }
    #[test]
    #[ignore]
    fn it_returns_ne_true() {
        assert_ne!(true, false);
    }
    #[test]
    fn it_returns_str() {
        assert_eq!(str_data(), "Hello world !", "!!! Wrong string")
    }
}
