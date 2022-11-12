use guiedit::Inspectable;

#[test]
fn test_structures() {
    #[derive(Inspectable)]
    struct UnitStruct;

    #[derive(Inspectable)]
    struct TupleStruct(String);

    #[derive(Inspectable)]
    struct NamedStructNoMembers {}

    #[derive(Inspectable)]
    struct NamedStruct<'s> {
        inner_ref: &'s mut TupleStruct,
        #[inspectable(ignore)]
        #[allow(dead_code)]
        inner: TupleStruct,
        my_number: i32,
    }
}
