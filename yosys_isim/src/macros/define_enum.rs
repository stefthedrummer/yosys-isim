#[macro_export]
macro_rules! count {
    () => { 0usize };
    ($head:ident $(, $tail:ident)*) => {
        1usize + crate::count!($($tail),*)
    };
}

#[macro_export]
macro_rules! define_enum {
    (enum $name:ident
        repr($repr:ident)
        derive( $($derive:ident),* ) {
            $($variants:ident ,)+
        }) => {
        paste::paste! {
            #[repr($repr)]
            #[derive($($derive),*)]
            #[allow(non_camel_case_types)]
            pub enum $name {
                $($variants),+
            }

            pub const [<$name _Len>]: usize = crate::count!($($variants),+);

            pub const [<$name _Variants>]: [$name; [<$name _Len>]] = [$($name::$variants),+];
        }
    };
}
