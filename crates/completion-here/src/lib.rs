use proto::generated::foo::{Whatever, Wrapper};

// Completion works here for concrete type
fn completion_works_concrete(whatever: Whatever) {
    whatever.not_in_scope()
}

// No completion here for generic type
fn no_completion<T: Wrapper>(whatever: T) {
    whatever.inner().not_in_scope()
}
