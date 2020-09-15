struct Foo {
    pub bar: Bar,
}

struct Bar {
    pub prop: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = Foo {
            bar: Bar { prop: 42 },
        };
        assert_eq!(foo.bar.prop, 42);
    }
}
