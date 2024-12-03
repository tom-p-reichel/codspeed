
use core::str;
use std::arch::x86_64::{__m128i, _mm_cmpeq_epi8, _mm_set1_epi8};
/* 
const MASK_M : u128 = 0x6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d;

const MAGIC : u128 = 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f;

const MAGIC2 : u128 = 0x01010101010101010101010101010101;
*/


const MASK_M : u128 = 0x6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d;

const MAGIC : u128 = 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f;

const MAGIC2 : u128 = 0x01010101010101010101010101010101;


pub fn handle_m(b:&[u8], i:usize) -> u32 {
    if  b.len() < i + 8 {
        return 0
    }
    if &b[i..i+4] != b"mul(" {
        return 0
    }
    let p = i+4;
    let acc = 0;
    1
    
}

#[aoc(day3, part1)]
pub fn part1(input:&str) -> u32 {
    let x = unsafe { part1_unsafe(input) };
    x
}

#[target_feature(enable = "avx2")]
pub unsafe fn part1_unsafe(input:&str) -> u32 {
    let b = input.as_bytes(); 
    let mut cnter = 0;
    let scan;
    scan = (b.get_unchecked(0) as *const u8 as *const u128);

    for i in (0..  input.len() / 16) {
        let mut v =  (((!( unsafe {*scan.offset(i as isize) } ^ MASK_M ) & MAGIC)) + MAGIC2) & (!MAGIC);
        // v.trailing_zeros();
        while v != 0 {
            let tz = v.trailing_zeros();
            v -= 1 << tz;
            let loc = i*16+(tz as usize)/8;
            cnter += handle_m(b, loc)
        } 

    }
    cnter
}
