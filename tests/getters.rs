use getset::Getters;

#[derive(Getters)]
struct SimpleStruct {
    pub public_field: i16,
    private_field: i16,
}

#[test]
fn should_generate_getters_for_simple_struct() {
    let simple = SimpleStruct {
        public_field: 69,
        private_field: 420,
    };
    assert_eq!(simple.get_public_field().clone(), 69);
    assert_eq!(simple.get_private_field().clone(), 420);
}

#[derive(Getters)]
struct GenericStruct<T: Copy + Clone + Default> {
    pub public_field: T,
    private_field: T,
}

#[test]
fn should_generate_getters_for_generic_struct() {
    let generic = GenericStruct {
        public_field: 1337,
        private_field: 808,
    };

    assert_eq!(generic.get_public_field(), &1337);
    assert_eq!(generic.get_private_field(), &808);
}

#[derive(Getters)]
struct WithWhereStruct<T>
where
    T: Copy + Clone + Default,
{
    pub public_field: T,
    private_field: T,
}

#[test]
fn should_generate_getters_for_struct_with_where() {
    let with_where = WithWhereStruct {
        public_field: 1337,
        private_field: 808,
    };

    assert_eq!(with_where.get_public_field(), &1337);
    assert_eq!(with_where.get_private_field(), &808);
}
