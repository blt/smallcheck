/// `Serial` describes an ordered enumeration over a type. The implementor gets
/// to decide what 'small' means.
pub trait Serial: Clone + Send + 'static {
    fn new() -> Self;

    fn next<S: Serial>(s: &mut S) -> Option<Self>;
}

impl Serial for () {
    fn new() -> () {
        ()
    }

    fn next<S: Serial>(_: &mut S) -> Option<()> { None }
}

macro_rules! impl_serial_for_tuple {
    (($var_a:ident, $type_a:ident) $(, ($var_n:ident, $type_n:ident))*) => (
        impl<$type_a: Serial, $($type_n: Serial),*> Serial
                for ($type_a, $($type_n),*) {
                    fn new() -> ($type_a, $(type_n),*) {
                        (
                            $type_a::new(),
                            $({
                                $type_n::new()
                            },
                            )*
                        )
                    }

                    // Well, crap. I think to implement `next` for an arbitrary
                    // structure I'm going to have to look into the lazy
                    // approach. The problem here being that, while I can fiddle
                    // with simple types directly this is a cheat. `Serial`
                    // really has some state or, absent state, must _always_ be
                    // able to be interpreted into the 'next' value. 
                }
    );
}

impl_serial_for_tuple!((a, A));
impl_serial_for_tuple!((a, A), (b, B));
impl_serial_for_tuple!((a, A), (b, B), (c, C));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G), (h, H));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G), (h, H), (i, I));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G), (h, H), (i, I), (j, J));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G), (h, H), (i, I), (j, J), (k, K));
impl_serial_for_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F),
                    (g, G), (h, H), (i, I), (j, J), (k, K), (l, L));

macro_rules! unsigned_serial {
    ($($ty:ty),*) => {
        $(
            impl Serial for $ty {
                fn new() -> $ty {
                    0
                }

                fn next<S: Serial>(s: &mut S) -> Option<$ty> {
                    *s.checked_add(1)
                }
            }
        )*
    }
}

unsigned_arbitrary! {
    usize, u8, u16, u32, u64
}
