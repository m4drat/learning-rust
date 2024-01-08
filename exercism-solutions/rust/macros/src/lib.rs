#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($keyn:expr => $valn:expr),+ $(,)? ) => {
        ::std::collections::HashMap::from(
            [
                $(($keyn, $valn)),*
            ]
        )
    };
}
