#[macro_export]
macro_rules! main {
    ($($partnum:literal = $partfun:ident$(,)?)*) => {
        fn main() {
            const INPUT: &str = include_str!(concat!(module_path!(), ".txt"));
            $(
                println!("Part {}: {}", $partnum, $partfun(INPUT));
            )*
        }
    }
}

#[macro_export]
macro_rules! tests {
    ($($testfun:ident for $partfun:ident requires:
        $(
        $input:literal = $expected_result:literal
        )*
    )*) => {
        $(
            #[test]
            fn $testfun() {
                $(
                    eprintln!("Testing {}", stringify!($input = $expected_result));
                    assert_eq!($partfun($input), $expected_result);
                )*
            }
        )*
    }
}
