struct Canonical;
impl<S: Into<std::borrow::Cow<'static, str>> From<S> for Canonical { //~ ERROR expected
    fn foo() {}
}
fn bar() {}
