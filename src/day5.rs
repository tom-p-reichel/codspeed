
use core::{num, str};
use std::arch::x86_64::{__m128i, __m256i, _mm256_and_si256, _mm256_andnot_pd, _mm256_castps_si256, _mm256_cmpeq_epi8, _mm256_loadu_ps, _mm256_loadu_si256, _mm256_maddubs_epi16, _mm256_movemask_epi8, _mm256_or_si256, _mm256_set1_epi32, _mm256_set1_epi8, _mm256_set_epi32, _mm256_srli_epi16, _mm256_sub_epi8, _mm256_xor_si256};
use std::borrow::BorrowMut;
use crate::hacks::{x128, x256, X128, X256};



#[aoc(day5, part1)]
pub fn part1(input:&str) -> u32 {
    let x = unsafe { part1_unsafe(input) };
    x
}

/* 
// i'm tired of casting the same underlying data to slightly different meanings
// this competition is fixed on x86 it's fine
#[derive(Clone, Copy)]
#[repr(align(128))]
#[repr(packed)]
union x128 {
    u : u128,
    m : __m128i,
    half : [u64;2]
}

#[derive(Clone, Copy)]
#[repr(align(128))]
#[repr(packed)]
union x256 {
    half : [u128 ;2],
    quarter : [u64 ; 4],

    m : __m128i,
    half : [u64;2]
}
*/

unsafe fn read10(b:&[u8], p:&mut usize, out:&mut [u8]) -> u32 {
    let syntax = load32(b"__,__,__,__,__,__,__,__,__,__,__" as &[u8;32]);
    let syntax_mask = _mm256_cmpeq_epi8(syntax, _mm256_set1_epi8(b'_' as i8));

    let chunk = load32(&b[*p..]);
    
    let valid = _mm256_movemask_epi8(_mm256_or_si256(_mm256_cmpeq_epi8(chunk, syntax),syntax_mask)) as u32;

    let len = valid.trailing_ones()/3;

    let coef = (1<<8) + (10<<16);
    
    let mut vals : [u64;4] = x256(_mm256_and_si256(_mm256_sub_epi8(chunk, _mm256_set1_epi8(b'0' as i8)),syntax_mask));

    vals[3] = vals[3] * coef + (((vals[2] >> 32)*coef)>>32); 
    vals[2] = vals[2] * coef + (((vals[1] >> 32)*coef)>>32);
    vals[1] = vals[1] * coef + (((vals[0] >> 32)*coef)>>32);
    vals[0] = vals[0] * coef ;
    
    let char_view: *const u8 = vals.as_ptr().cast();
    
    for i in (0..len) {
        out[i as usize] = *char_view.offset((i*3+2) as isize);
    }

    *p += (3*len) as usize;

    len
} 



unsafe fn load32(a:&[u8]) -> __m256i {
    return _mm256_loadu_si256(a.as_ptr() as *const __m256i)
}


#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part1_unsafe(input:&str) -> u32 {
    
    let mut gt : [X128; 128] = [0.into(); 128];
    let mut lt : [X128; 128] = [0.into(); 128];

    let b = input.as_bytes();

    let mut p = 0;

    {

        let syntax = load32(b"__|__\n__|__\n__|__\n__|__\n__|__\n__" as &[u8;32]);
        let syntax_mask = _mm256_cmpeq_epi8(syntax, _mm256_set1_epi8(b'_' as i8));
        // j = 10
        // a = 1
        // p = 0
        //let num_pos = _mm256_and_si256(load32(b"japjapjapjapjapjapjapjapjapjappp" as &[u8;32]) , _mm256_set1_epi8(0b1111));

        loop {

            let chunk = load32(&b[p..]);
            
            let valid = _mm256_movemask_epi8(_mm256_or_si256(_mm256_cmpeq_epi8(chunk, syntax),syntax_mask)) as u32;

            let len = valid.trailing_ones()/6;

            let coef = (1<<8) + (10<<16);
            
            let mut vals : [u64;4] = x256(_mm256_and_si256(_mm256_sub_epi8(chunk, _mm256_set1_epi8(b'0' as i8)),syntax_mask));
            vals[3] = vals[3] * coef + (((vals[2] >> 32)*coef)>>32); 
            vals[2] = vals[2] * coef + (((vals[1] >> 32)*coef)>>32);
            vals[1] = vals[1] * coef + (((vals[0] >> 32)*coef)>>32);
            vals[0] = vals[0] * coef ;
            
            let char_view: *const u8 = vals.as_ptr().cast();
            
            for i in (0..len) {
                let a = *char_view.offset(((6*i+2) as isize));
                let b = *char_view.offset(((6*i+5) as isize));

                gt[a as usize] = (x128::<_,u128>(gt[a as usize]) | (1<<b)).into();
                lt[b as usize] = (x128::<_,u128>(lt[b as usize]) | (1<<b)).into();



            }

            p += (6*len) as usize;

            if valid != 0xffffffff {
                break;
            }
        };
    }

    p += 1;
    

    {
        
    }

    x128::<_,[u64;2]>(gt[0])[0] as u32
}