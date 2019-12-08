#[macro_export]
macro_rules! main {
    ($($partnum:literal = $partfun:ident$(,)?)*) => {
        fn main() {
            $(
                println!("Part {}: {}", $partnum, $partfun(include_str!(concat!(module_path!(), ".txt"))));
            )*
        }
    }
}

#[macro_export]
macro_rules! tests {
    ($($testfun:ident for $partfun:ident:
        $(
        $input:literal = $expected_result:literal
        )*
        $([$input_result:literal])?
    )*) => {
        $(
            #[test]
            fn $testfun() {
                $(
                    eprintln!("Testing {}", stringify!($input = $expected_result));
                    assert_eq!($partfun($input), $expected_result);
                )*
                $(
                    eprintln!("Testing answer for input");
                    assert_eq!($partfun(include_str!(concat!(module_path!(), ".txt"))), $input_result);
                )?
            }
        )*
    }
}