
#[macro_export]
macro_rules! map {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut temp = std::collections::HashMap::new();
            $(
                temp.insert($x, $y);
            )*
            temp
        }
    }
}

pub use map;