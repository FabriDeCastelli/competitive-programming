macro_rules! define_min_max {
    ($type:ty) => {
        impl MinMax for $type {
            const MIN: Self = <$type>::MIN;
            const MAX: Self = <$type>::MAX;

            fn one() -> Self {
                1 as $type
            }
        }
    };
}

macro_rules! define_min_max_for_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> MinMax for ($($t),*) where $($t: MinMax),* {
            const MIN: Self = ($($t::MIN),*);
            const MAX: Self = ($($t::MAX),*);

            fn one() -> Self {
                ($(<$t as MinMax>::one()),*)
            }
        }
    };
}

pub(crate) trait MinMax {
    const MIN: Self;
    const MAX: Self;

    fn one() -> Self;
}

define_min_max!(i8);
define_min_max!(i16);
define_min_max!(i32);
define_min_max!(i64);
define_min_max!(i128);
define_min_max!(u8);
define_min_max!(u16);
define_min_max!(u32);
define_min_max!(u64);
define_min_max!(u128);
define_min_max!(f32);
define_min_max!(f64);
define_min_max_for_tuple!(T1, T2);

#[test]
fn test_tuples_ordering() {
    let a = (1, 2);
    let b = (2, 1);
    let min = <(i32, i32)>::MIN;
    let max = <(i32, i32)>::MAX;
    assert_eq!(a.min(b), a);
    assert_eq!(a.max(b), b);
    assert!(a.ge(&min));
    assert!(a.le(&max));
    assert_eq!(min, min);
    assert_eq!(max, max);

    let zero = <(i32, i32)>::default();
    assert_eq!(zero, (0, 0));
}
