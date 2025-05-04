#[macro_export]
macro_rules! count {
    () => { 0usize };
    ($head:ident $(, $tail:ident)*) => {
        1usize + crate::count!($($tail),*)
    };
}

#[macro_export]
macro_rules! make_enum {
    ($name:ident, $($variants:ident),+) => {
        paste::paste! {
            #[repr(u32)]
            #[derive(Clone, Copy, Debug)]
            pub enum $name {
                $($variants),+
            }

            pub const [<$name _Len>]: usize = crate::count!($($variants),+);

            pub const [<$name _Variants>]: [$name; [<$name _Len>]] = [$($name::$variants),+];
        }
    };
}
