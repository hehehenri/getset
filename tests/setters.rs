use getset::Setters;

#[derive(Setters)]
struct SimpleStruct {
    pub public_field: i16,
    private_field: i16,
}

#[test]
fn should_generate_setters_for_simple_struct() {
    let mut simple = SimpleStruct {
        public_field: 0,
        private_field: 0,
    };
    simple.set_public_field(69);
    simple.set_private_field(420);

    assert_eq!(simple.public_field, 69);
    assert_eq!(simple.private_field, 420);
}

#[derive(Setters)]
struct GenericStruct<T: Copy + Clone + Default> {
    pub public_field: T,
    private_field: T,
}

#[test]
fn should_generate_setters_for_generic_struct() {
    let mut generic = GenericStruct {
        public_field: 0,
        private_field: 0,
    };

    generic.set_public_field(1337);
    generic.set_private_field(808);

    assert_eq!(generic.public_field, 1337);
    assert_eq!(generic.private_field, 808);
}

#[derive(Setters)]
struct WithWhereStruct<T>
where
    T: Copy + Clone + Default,
{
    pub public_field: T,
    private_field: T,
}

#[test]
fn should_generate_setters_for_struct_with_where() {
    let mut with_where = WithWhereStruct {
        public_field: 1337,
        private_field: 808,
    };

    with_where.set_public_field(1337);
    with_where.set_private_field(808);

    assert_eq!(with_where.public_field, 1337);
    assert_eq!(with_where.private_field, 808);
}

#[derive(Setters)]
#[allow(dead_code)]
struct SkippedFieldsStruct {
    #[getset(skip)]
    pub global_skipped_field: i16,
    #[getset(skip_getter)]
    pub getter_skipped_field: i16,
    #[getset(skip_setter)]
    pub setter_skipped_field: i8,
}

// INFO: if there was already setters implemented for the skipped
// fields, the implementations would conflict.
#[allow(dead_code)]
impl SkippedFieldsStruct {
    fn set_global_skipped_field() {}
    fn set_setter_skipped_field() {}
}

#[test]
fn should_generate_getters_for_setter_skipped_field() {
    let mut skipped_field_struct = SkippedFieldsStruct {
        global_skipped_field: 0,
        getter_skipped_field: 0,
        setter_skipped_field: 0,
    };

    // but we still should be able to set getter skipped field
    skipped_field_struct.set_getter_skipped_field(69);
    assert_eq!(skipped_field_struct.getter_skipped_field, 69);
}
