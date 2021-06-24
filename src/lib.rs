pub mod bench_abomonation;
pub mod bench_bcs;
pub mod bench_bincode;
pub mod bench_borsh;
pub mod bench_capnp;
pub mod bench_cbor;
pub mod bench_flatbuffers;
pub mod bench_nachricht;
pub mod bench_postcard;
pub mod bench_prost;
pub mod bench_rkyv;
pub mod bench_rmp;
pub mod bench_ron;
pub mod bench_serde_json;
pub mod bench_speedy;
pub mod datasets;

use core::{mem, ops};
use rand::Rng;
use std::{cmp::Ord, collections::BTreeMap};

pub trait Generate {
    fn generate<R: Rng>(rng: &mut R) -> Self;
}

impl Generate for () {
    fn generate<R: Rng>(_: &mut R) -> Self {
        ()
    }
}

impl Generate for bool {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen_bool(0.5)
    }
}

macro_rules! impl_generate {
    ($ty:ty) => {
        impl Generate for $ty {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                rng.gen()
            }
        }
    };
}

impl_generate!(u8);
impl_generate!(u16);
impl_generate!(u32);
impl_generate!(u64);
impl_generate!(u128);
impl_generate!(usize);
impl_generate!(i8);
impl_generate!(i16);
impl_generate!(i32);
impl_generate!(i64);
impl_generate!(i128);
impl_generate!(isize);
impl_generate!(f32);
impl_generate!(f64);

macro_rules! impl_tuple {
    () => {};
    ($first:ident, $($rest:ident,)*) => {
        impl<$first: Generate, $($rest: Generate,)*> Generate for ($first, $($rest,)*) {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                ($first::generate(rng), $($rest::generate(rng),)*)
            }
        }

        impl_tuple!($($rest,)*);
    };
}

impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);

macro_rules! impl_array {
    () => {};
    ($len:literal, $($rest:literal,)*) => {
        impl<T: Generate> Generate for [T; $len] {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                let mut result = mem::MaybeUninit::<Self>::uninit();
                let result_ptr = result.as_mut_ptr().cast::<T>();
                for i in 0..$len {
                    unsafe {
                        result_ptr.add(i).write(T::generate(rng));
                    }
                }
                unsafe {
                    result.assume_init()
                }
            }
        }

        impl_array!($($rest,)*);
    }
}

impl_array!(
    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8,
    7, 6, 5, 4, 3, 2, 1, 0,
);

impl<T: Generate> Generate for Option<T> {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        if rng.gen_bool(0.5) {
            Some(T::generate(rng))
        } else {
            None
        }
    }
}

pub fn generate_vec<R: Rng, T: Generate>(rng: &mut R, range: ops::Range<usize>) -> Vec<T> {
    let len = rng.gen_range(range);
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::generate(rng));
    }
    result
}

pub fn generate_btreemap<R, K, V>(rng: &mut R, range: ops::Range<usize>) -> BTreeMap<K, V>
where
    R: Rng,
    K: Generate + Ord,
    V: Generate,
{
    generate_vec::<R, (K, V)>(rng, range).into_iter().collect()
}

pub fn generate_oneof<'a, R: Rng, T>(rng: &mut R, ts: &'a [T]) -> &'a T {
    let idx = rng.gen_range(0..ts.len());
    &ts[idx]
}

pub fn zlib_size(mut bytes: &[u8]) -> usize {
    let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut bytes, &mut encoder).unwrap();
    encoder.finish().into_result().unwrap().len()
}
