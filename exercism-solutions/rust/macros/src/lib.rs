#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($key0:expr => $val0:expr $(, $keyn:expr => $valn:expr)* $(,)? ) => {
        ::std::collections::HashMap::from(
            [
                ($key0, $val0),
                $(($keyn, $valn)),*
            ]
        )
    };
}
