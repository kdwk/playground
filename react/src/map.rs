#[macro_export]
macro_rules! map {
    ($( $x:expr => $y:expr ),* $(,)?) => {
        {
            HashMap::from([
                $(($x, $y),)*
            ])
        }
    };
}
