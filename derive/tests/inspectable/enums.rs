use guiedit_derive::Inspectable;

#[test]
fn test_enums() {
    #[derive(Inspectable)]
    enum Enum {
        Unit,
        Tuple(i32, String),
        Named { value: Vec<i32> },
    }

    #[derive(Inspectable)]
    enum EnumWithGenerics<MaybeInspectable> {
        Unit,
        Tuple(i32, String),
        Named { value: Vec<MaybeInspectable> },
    }
}
