use guiedit_derive::Inspectable;

#[test]
fn test_proc_macro() {
    #[derive(Inspectable)]
    struct MyStruct {
        my_enum: MyEnum,
        my_number: i32,
    }

    #[derive(Inspectable)]
    enum MyEnum {
        Unit,
        TupleNumber(i32),
    }
}
