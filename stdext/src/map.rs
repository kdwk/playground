#[macro_export]
macro_rules! map {
    ($( $x:expr => $y:expr ),* $(,)?) => {
        {
            use std::collections::HashMap;
            HashMap::from([
                $(($x, $y),)*
            ])
        }
    };
}
