#[macro_export]
macro_rules! note {
    ($value:ident, $($tone:expr),+) => (
        $crate::NoteDef::new($crate::NoteValue::$value, vec![$($tone),+])
    )
}

#[macro_export]
macro_rules! rest {
    ($value:ident) => {
        $crate::NoteDef::new($crate::NoteValue::$value, vec![])
    };
}

#[macro_export]
macro_rules! pattern {
    ($($note:expr),+) => (
        $crate::Pattern::new(vec![$($note),+])
    )
}

#[macro_export]
macro_rules! frame {
    ($($pattern:expr),+) => (
        $crate::TrackKeyframe::new(vec![$($pattern.clone()),+])
    )
}

#[macro_export]
macro_rules! track {
    ($($frame:expr),+) => (
        $crate::Track::new(vec![$($frame.clone()),+])
    )
}
