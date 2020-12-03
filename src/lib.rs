pub mod array_2d;

#[macro_export]
macro_rules! main {
    ($($partfun:ident$(,)?)*) => {
        fn main() {
            let mut part = 1;
            $(
                println!("Part {}: {}", part, $partfun(include_str!(concat!(module_path!(), ".txt"))));
                part += 1;
            )*
        }
    }
}

#[macro_export]
macro_rules! tests {
    ($(fn $partfun:ident:
        $(
        $input:expr => $expected_result:expr
        )*
        $(=> $input_result:literal)?
    )*) => {
        $(
            concat_idents::concat_idents!(test_name = test, _, $partfun {
                #[test]
                fn test_name() {
                    $(
                        eprintln!("Testing {}", stringify!($input = $expected_result));
                        assert_eq!($partfun($input), $expected_result);
                    )*
                    $(
                        eprintln!("Testing answer for input");
                        assert_eq!($partfun(include_str!(concat!(module_path!(), ".txt"))), $input_result);
                    )?
                }
            });
        )*
    }
}
