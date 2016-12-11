/// Try to parse something, and check the result. For example:
///
///     assert_parse!(OperatorName: "quokka" => Ok(OperatorName::Question, "okka"));
///
/// This asserts that calling `OperatorName::parse` on the text `"qu"`
/// succeeds, producing the value `OperatorName::Question`, and leaving the
/// unparsed text `"okka"`.
macro_rules! assert_parse {
    ($nonterm:ty : $input:expr => Ok($ex_value:expr, $ex_tail:expr)) => {
        let input = $input as &[u8];
        let input_printable = String::from_utf8_lossy(input).into_owned();
        let ex_value = $ex_value;
        let ex_tail = $ex_tail as &[u8];
        match <$nonterm>::parse(IndexStr::from(input)) {
            Err(e) => panic!("Parsing {:?} as {} failed: {}",
                             input_printable, stringify!($nonterm), e),
            Ok((value, tail)) => {
                if value != ex_value {
                    panic!("Parsing {:?} as {} produced {:?}, expected {:?}",
                           input_printable, stringify!($nonterm), value, ex_value);
                }
                if tail != ex_tail {
                    panic!("Parsing {:?} as {} left a tail of {:?}, expected {:?}",
                           input_printable, stringify!($nonterm), tail.as_ref(), ex_tail);
                }
            }
        }
    }
}