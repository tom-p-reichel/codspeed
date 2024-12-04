
use core::str;
use std::{arch::x86_64::{__m128i, __m256i, _mm256_cmpeq_epi8, _mm256_movemask_epi8, _mm256_set1_epi8, _mm_adds_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_cmplt_epi8, _mm_extract_epi16, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16, _mm_movemask_epi8, _mm_set1_epi8, _mm_setr_epi32, _mm_setr_pd, _mm_setr_ps}, str::Bytes};


const X : u32 = 0b11;
const M : u32 = 0b01;
const A : u32 = 0b00;
const S : u32 = 0b10;
const CHAR_MASK : u8 = 0b11000;
const CHAR_MUL : u32 = (1<<5) + (1<<13) + (1<<21);
const ASSEMBLE_MASK : u32 = 0xfcfcfc00;
const LD_MASK : u32 = 0xfc000000; 
const UP_MASK : u32 = 0x00fc0000; 
const RD_MASK : u32 = 0x0000fc00;

const ROWS : usize = 140;
const COLS : usize= 140;
const COLS_NEWL : usize = ROWS+1;

#[aoc(day4, part1)]
pub fn part1(input:&str) -> u32 {
    let x = unsafe { part1_unsafe(input) };
    x
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part1_unsafe(input:&str) -> u32 {
    // horizontals
    let b = input.as_bytes();
    assert!(b[140]==b'\n');
    
    let mut DP : [u32; ROWS*COLS] = [0; ROWS*COLS];

    // compiler should auto-vectorize this part adequately.. (TODO: CHECK)
    for i in (0..ROWS) {
        DP[i] = (((b[i] & CHAR_MASK) as u32) * CHAR_MUL);
    }

    for row in (1..ROWS) {
        DP[row*COLS] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
            + ((DP[(row-1)*COLS]*4)&UP_MASK)
            + ((DP[(row-1)*COLS+1]*4)&RD_MASK);

        for col in (1..COLS-1) {
            DP[row*COLS + col] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
                + ((DP[(row-1)*COLS + col]*4)&UP_MASK)
                + ((DP[(row-1)*COLS + col + 1]*4)&RD_MASK)
                + ((DP[(row-1)*COLS + col - 1]*4)&LD_MASK);
        }

        DP[(row+1)*COLS-1] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
            + ((DP[row*COLS-1]*4)&UP_MASK)
            + ((DP[row*COLS-2]*4)&LD_MASK);
    }




    return DP[140*140 - 2];
}