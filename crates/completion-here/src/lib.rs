use proto::generated::foo::Wrapper;

fn foo<T: Wrapper>(whatever: T) {
    let t = whatever.inner();
    t.not_in_scope();
}
