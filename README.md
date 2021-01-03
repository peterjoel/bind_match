# bind_match

Convenience macro similar to `matches!` but lets you bind to and pattern and return an `Option` of the result.

The input of the macro looks like this:

`bind_match!(input_expr, pattern => binding_expr)`

Or with pattern guards:

`bind_match!(input_expr, pattern if guard => binding_expr)`

The `binding_expr` is returned, with variables bound in the pattern.

## Example

This can be particularly useful when matching inside iterator adapters.

```Rust
use bind_match::bind_match;

enum Foo {
    A(Option<i32>, bool),
    B { open: bool, closed: bool },
}
struct Bar {
    foo: Foo,
    fun: bool,
}

fn fun_when_open(bars: impl IntoIterator<Item = Bar>) -> Option<bool> {
    bars.into_iter()
        .filter_map(|bar| bind_match!(bar, Bar { foo: Foo::B { open, .. }, fun } if open => fun ))
        .next()
}
```
