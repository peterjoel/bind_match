/// Works similarly to `matches!` but returns an `Option` of variables bound in the pattern
///
/// The input of the macro looks like this:
///
/// `bind_match!(input_expr, pattern => binding_expr)`
///
/// Or with pattern guards:
///
/// `bind_match!(input_expr, pattern if guard => binding_expr)`
///
/// The `binding_expr` is returned, with variables bound in the pattern.
///
/// ## Example
///
/// ```
/// use bind_match::bind_match;
///
/// enum Foo {
///     A(Option<i32>, bool),
///     B { open: bool, closed: bool },
/// }
/// struct Bar {
///     foo: Foo,
///     fun: bool,
/// }
///
/// # fn main() {
///    let bar = Bar {
///        foo: Foo::A(Some(42), true),
///        fun: false,
///    };
///    let result = bind_match!(bar, Bar { foo: Foo::A(Some(n), x), fun } if !fun => (n, x));
///    assert_eq!(result, Some((42, true)));
/// # }
/// ```
/// This can be particularly useful when matching inside iterator adapters. For example, using
/// the same types as above:
/// ```
/// # use bind_match::bind_match;
/// # enum Foo {
/// #     A(Option<i32>, bool),
/// #     B { open: bool, closed: bool },
/// # }
/// # struct Bar {
/// #     foo: Foo,
/// #     fun: bool,
/// # }
/// fn fun_when_open(bars: impl IntoIterator<Item = Bar>) -> Option<bool> {
///     bars.into_iter()
///         .filter_map(|bar| bind_match!(bar, Bar { foo: Foo::B { open, .. }, fun } if open => fun ))
///         .next()
/// }
/// ```
#[macro_export]
macro_rules! bind_match {
    ($expression: expr, $($pattern: pat)|+ $(if $guard: expr)? => $binding_expr: expr) => {
        match $expression {
            $($pattern)|+ $(if $guard)? => Some($binding_expr),
            _ => None
        }
    }
}
