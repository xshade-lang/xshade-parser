fn main() {

}

fn main() -> () {

}

fn main(a: A) -> B {

}

fn main(a: A, b: B) -> C {

}

fn main(a: A, b: B) -> C<A, B> {

}

fn main(a: A, b: B<C>) -> (a: A, b: B<C>) {

}

fn identity<T>(item: T) -> T {

}
