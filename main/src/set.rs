#[macro_export]
macro_rules! set {
    ($( $x:expr => $y:expr ),* $(,)?) => {
        {
            HashSet::from([
                $(($x, $y),)*
            ])
        }
    };
}
