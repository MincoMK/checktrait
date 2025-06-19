/// `checktrait::ty!` checks whether type implements a trait in compile time.
///
/// # Parameters
///
/// * `$trait_name` - Name of the trait that check implementation
/// * `$ty` - The type to check
#[macro_export]
macro_rules! ty {
    ($trait_name:ident, $ty:ty) => {
        paste::paste! {
            #[allow(non_snake_case)]
            fn [<checktrait_ $trait_name _obj>]<T: $trait_name>() {}
            [<checktrait_ $trait_name _obj>]::<$ty>();
        }
    };
}

/// `checktrait::obj!` checks whether object implements a trait in compile time.
///
/// # Parameters
///
/// * `$trait_name` - Name of the trait that check implementation
/// * `$obj` - The object to check
#[macro_export]
macro_rules! obj {
    ($trait_name:ident, $obj:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            fn [<checktrait_ $trait_name _obj>]<T: $trait_name>(_o: T) {}
            [<checktrait_ $trait_name _obj>]($obj);
        }
    };
}

#[cfg(test)]
mod tests {
    #![allow(warnings)]

    trait MyTrait {
        fn hello();
    }

    struct MyStruct {
        asdf: String,
    }

    impl MyTrait for MyStruct {
        fn hello() {}
    }

    #[test]
    fn test_ty() {
        crate::ty!(MyTrait, MyStruct);
    }

    #[test]
    fn test_obj() {
        let obj = MyStruct {
            asdf: "123".to_string(),
        };
        crate::obj!(MyTrait, obj);
    }
}
