#![allow(unused_imports)]

use crate::{Context, DeepSizeOf};

#[cfg(features = "slotmap")]
mod slotmap_impl {
    use super::*;

    known_deep_size!(0, slotmap::KeyData, slotmap::DefaultKey);

    impl<K, V> DeepSizeOf for slotmap::SlotMap<K, V>
    where
        K: DeepSizeOf + slotmap::Key,
        V: DeepSizeOf + slotmap::Slottable,
    {
        #[inline]
        fn deep_size_of_children(&self, context: &mut Context) -> usize {
            self.iter()
                .fold(0, |sum, (key, val)| {
                    sum + key.deep_size_of_children(context)
                        + val.deep_size_of_children(context)
                })
            + self.capacity() * size_of::<(u32, V)>>()
        }
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_impl {
    use super::*;

    use core::mem::size_of;

    use smallvec::{Array, SmallVec};

    impl<A: Array> DeepSizeOf for SmallVec<A>
    where
        A::Item: DeepSizeOf,
    {
        #[inline]
        fn deep_size_of_children(&self, context: &mut Context) -> usize {
            self.iter().fold(0, |sum, item| sum + item.deep_size_of_children(context))
                + if self.capacity() <= self.inline_size() {
                    self.inline_size()
                } else {
                    self.capacity() + self.inline_size()
                }
                + (size_of::<usize>() * 2)
        }
    }
}

#[cfg(feature = "half")]
mod half_impl {
    use super::*;

    impl DeepSizeOf for half::f16 {
        #[inline(always)]
        fn deep_size_of_children(&self, _context: &mut Context) -> usize {
            0
        }
    }
}

#[cfg(feature = "packed_simd")]
mod packed_simd_impl {
    use super::*;

    macro_rules! impl_packed_simd {
        ($($vec:ident,)*) => {
            $(
                impl DeepSizeOf for packed_simd::$vec {
                    #[inline(always)]
                    fn deep_size_of_children(&self, _context: &mut Context) -> usize { 0 }
                }
            )*
        }
    }

    macro_rules! impl_packed_ptr {
        ($($vec:ident,)*) => {
            $(
                impl<T> DeepSizeOf for packed_simd::$vec<T> {
                    #[inline(always)]
                    fn deep_size_of_children(&self, _context: &mut Context) -> usize { 0 }
                }
            )*
        }
    }

    impl_packed_ptr! {
        cptrx2,
        cptrx4,
        cptrx8,
        mptrx2,
        mptrx4,
        mptrx8,
    }

    impl_packed_simd! {
        f32x2,
        f32x4,
        f32x8,
        f32x16,
        f64x2,
        f64x4,
        f64x8,
        i128x1,
        i128x2,
        i128x4,
        i16x2,
        i16x4,
        i16x8,
        i16x16,
        i16x32,
        i32x2,
        i32x4,
        i32x8,
        i32x16,
        i64x2,
        i64x4,
        i64x8,
        i8x2,
        i8x4,
        i8x8,
        i8x16,
        i8x32,
        i8x64,
        isizex2,
        isizex4,
        isizex8,
        m128x1,
        m128x2,
        m128x4,
        m16x2,
        m16x4,
        m16x8,
        m16x16,
        m16x32,
        m32x2,
        m32x4,
        m32x8,
        m32x16,
        m64x2,
        m64x4,
        m64x8,
        m8x2,
        m8x4,
        m8x8,
        m8x16,
        m8x32,
        m8x64,
        msizex2,
        msizex4,
        msizex8,
        u128x1,
        u128x2,
        u128x4,
        u16x2,
        u16x4,
        u16x8,
        u16x16,
        u16x32,
        u32x2,
        u32x4,
        u32x8,
        u32x16,
        u64x2,
        u64x4,
        u64x8,
        u8x2,
        u8x4,
        u8x8,
        u8x16,
        u8x32,
        u8x64,
        usizex2,
        usizex4,
        usizex8,
    }
}
