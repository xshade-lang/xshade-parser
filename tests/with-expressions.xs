fn main() {
    let a = 10;
    let b = 20;
    let c = a + b;
}

fn index() {
    a[b];
}

fn multi_indexed() {
    a[b][c][d];
}

fn indexed_indexed() {
    a[b[c]];
}

fn index_negated() {
    -a[b];
}

fn field() {
    a.b;
}

fn field_negated() {
    -a.b;
}

fn multi_field() {
    a.b.c;
}

fn indexed_field_negated() {
    -b[c].a;
}
