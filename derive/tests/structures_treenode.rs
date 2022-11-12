use guiedit::{Inspectable, TreeNode};

#[test]
fn test_structures() {
    #[derive(Inspectable, TreeNode)]
    struct UnitStruct;

    #[derive(Inspectable, TreeNode)]
    struct TupleStruct(String);

    #[derive(Inspectable, TreeNode)]
    struct NamedStruct<'s> {
        inner_ref: &'s mut TupleStruct,
        inner: TupleStruct,
        my_number: i32,
    }
}
