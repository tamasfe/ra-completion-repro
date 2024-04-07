#[macro_export]
macro_rules! include_example {
    () => {
        include!(concat!(env!("OUT_DIR"), "/example.rs"));
    };
}

#[macro_export]
macro_rules! create_wrapper {
    () => {
        pub trait Wrapper {
            type Inner: NotInScope;
            fn inner(&self) -> Self::Inner;
        }
    };
}
