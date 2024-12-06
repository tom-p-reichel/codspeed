use std::arch::x86_64::{__m128, __m128d, __m128i, __m256, __m256d, __m256i};

// Look on my works, ye Mighty, and despair!

macro_rules! pointless_distinction {
    ($a:ty,$b:ty) => {

        impl From<$b> for $a {
            fn from(value : $b) -> Self {
                return unsafe { std::mem::transmute(value)} ;
            }
        }

        impl Into<$b> for $a {
            fn into(self) -> $b {
                return unsafe { std::mem::transmute(self)} ;
            }
        }


    };
}

#[derive(Clone,Copy)]
#[repr(align(16))]
pub struct X128(__m128i);
 
pointless_distinction!(X128,u128);
pointless_distinction!(X128,__m128i);
pointless_distinction!(X128,__m128);
pointless_distinction!(X128,__m128d);
pointless_distinction!(X128,[u64;2]);

pub fn x128<F,I>(x : F) -> I where X128 : From<F>+Into<I>  {
    X128::from(x).into()
}

/* 
impl X128 {
    pub fn cast<T>(&mut self) -> &mut T 
    where X128 : Into<T> {
        self.into()

    }
}
*/

#[derive(Clone,Copy)]
#[repr(align(32))]
pub struct X256(__m256i);

pointless_distinction!(X256,[u128;2]);
pointless_distinction!(X256,__m256i);
pointless_distinction!(X256,__m256);
pointless_distinction!(X256,__m256d);
pointless_distinction!(X256,[u64;4]);

pub fn x256<F,I>(x : F) -> I where X256 : From<F>+Into<I>  {
    X256::from(x).into()
}




const fn make_bit_lut() ->  [[u64;2];128] {
    let mut lut : [[u64;2];128] = [[0;2]; 128];
    let mut i = 0;
    while i < 128 {
        lut[i][i/64] = 1 << (i%64);
        i = i+1;
    }
    return lut
}

#[repr(align(16))]
pub struct Aligner<T> {
    pub thing : T 
}


pub const bit_lut_aligned : Aligner<[[u64;2];128]> = Aligner {thing:make_bit_lut()};

pub const bit_lut : &[[u64;2];128] = &bit_lut_aligned.thing;