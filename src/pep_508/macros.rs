macro_rules! set {
    ($($tt:tt)+) => {
        ::chumsky::primitive::any().filter(|c| matches!(c, $($tt)+))
    };
}

pub(crate) use set;
