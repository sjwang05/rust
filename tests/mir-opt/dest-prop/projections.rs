// skip-filecheck
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ test-mir-pass: DestinationPropagation

struct Foo {
    n1: u8,
    n2: u8,
}

// EMIT_MIR projections.fields.DestinationPropagation.diff
fn fields() -> u8 {
    let mut foo = Foo { n1: 0, n2: 0 };

    let n = 0;
    let mut n1 = n;
    let mut n2 = 2;
    foo.n1 = n1;
    foo.n2 = n2;

    foo.n1
}

// EMIT_MIR projections.fields2.DestinationPropagation.diff
fn fields2() -> u8 {
    let mut foo = Foo { n1: 0, n2: 0 };

    let n = 0;
    let mut n1 = n;
    let mut n2 = 2;
    foo.n1 = n1;
    foo.n2 = n2;
    foo.n1 = foo.n2;
    foo.n2 = foo.n1;

    foo.n1
}

// EMIT_MIR projections.swap_fields.DestinationPropagation.diff
fn swap_fields() -> u8 {
    let mut foo = Foo { n1: 0, n2: 1 };
    let temp = foo.n1;
    foo.n1 = foo.n2;
    foo.n2 = temp;
    foo.n1
}

fn main() {
    fields();
    fields2();
    assert_eq!(swap_fields(), 1);
}
