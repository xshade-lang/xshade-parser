fn main() {
    let a = 10;
    let b = 20;
    let c = a + b;
}

fn minus_negated() {
    let b = 10 - -5;
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

fn indexed_indexed_negated() {
    -b[-c];
}

fn ridiculous() {
    !-!~!-b[a].c[5][9].b;
}

fn multiple_binary() {
    a * b / c - d + f * g;
}

fn multiple_binary_with_unary() {
    !a * b[a] / -(c[b - c] - -d.f) + !f * ~g;
}
