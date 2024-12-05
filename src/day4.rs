
use core::str;
use std::arch::x86_64::{__m128i, __m256i, _mm256_and_si256, _mm256_castps_si256, _mm256_castsi256_ps, _mm256_cmpeq_epi32, _mm256_cmpeq_epi8, _mm256_extractf128_pd, _mm256_extractf128_si256, _mm256_load_ps, _mm256_loadu2_m128, _mm256_loadu2_m128i, _mm256_loadu_ps, _mm256_maddubs_epi16, _mm256_maskload_ps, _mm256_min_epi8, _mm256_movemask_epi8, _mm256_movemask_ps, _mm256_set1_epi32, _mm256_set1_epi8, _mm256_set_epi16, _mm256_shuffle_epi8, _mm_and_si128, _mm_castsi128_ps, _mm_cmpeq_epi32, _mm_cmpeq_epi8, _mm_hadd_epi16, _mm_loadu_si128, _mm_loadu_si64, _mm_maddubs_epi16, _mm_movemask_epi8, _mm_movemask_ps, _mm_mullo_epi16, _mm_set1_epi32, _mm_set1_epi8, _mm_set_epi16, _mm_set_epi32, _mm_set_epi8, _mm_shuffle_epi8, _mm_srli_epi16};
use std::arch::x86_64::_mm_castps_si128;
use std::borrow::BorrowMut;
/* 
const X : u32 = 0b11;
const M : u32 = 0b01;
const A : u32 = 0b00;
const S : u32 = 0b10;
*/
const CHAR_MASK : u8 = 0b11000;
const CHAR_MUL : u32 = 0x01010100 >> 3;
// const ASSEMBLE_MASK : u32 = 0xfcfcfc00;
const LD_MASK : u32 = 0xfc000000; 
const UP_MASK : u32 = 0x00fc0000; 
const RD_MASK : u32 = 0x0000fc00;
const LT_MASK : u32 = 0x0000003f;

const XMAS : u8 = 0b11010010;
const SAMX : u8 = 0b10000111;

const ROWS : usize = 140;
const COLS : usize= 140;
const COLS_NEWL : usize = ROWS+1;

#[aoc(day4, part1)]
pub fn part1(input:&str) -> u32 {
    let x = unsafe { part1_unsafe(input) };
    x
}

pub fn print_DP(x : u32) {

    let l = ['A','M','S','X'];

    println!("LD:{}{}{}{} UP:{}{}{}{} RD:{}{}{}{}",
        l[((x>>30)&3) as usize],
        l[((x>>28)&3) as usize],
        l[((x>>26)&3) as usize],
        l[((x>>24)&3) as usize],

        l[((x>>22)&3) as usize],
        l[((x>>20)&3) as usize],
        l[((x>>18)&3) as usize],
        l[((x>>16)&3) as usize],

        l[((x>>14)&3) as usize],
        l[((x>>12)&3) as usize],
        l[((x>>10)&3) as usize],
        l[((x>>8)&3) as usize],
    )
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part1_unsafe(input:&str) -> u32 {
    // horizontals
    let b = input.as_bytes();
    assert!(b[140]==b'\n');
    
    let mut dp : [u32; ROWS*COLS] = [0; ROWS*COLS];

    let mut dvcnt :u32 = 0;
    let mut hcnt : u32 = 0;

    // compiler should auto-vectorize this part adequately.. (TODO: CHECK)
    for i in 0..ROWS {
        dp[i] = ((b[i] & CHAR_MASK) as u32) * CHAR_MUL;
    }

    for row in 1..ROWS {
        dp[row*COLS] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
            | ((dp[(row-1)*COLS]*4)&UP_MASK)
            | ((dp[(row-1)*COLS+1]*4)&RD_MASK);

        for col in 1..COLS-1 {
            dp[row*COLS + col] = (((b[row*COLS_NEWL + col] & CHAR_MASK) as u32) * CHAR_MUL)
                | ((dp[(row-1)*COLS + col]*4)&UP_MASK)
                | ((dp[(row-1)*COLS + col + 1]*4)&RD_MASK)
                | ((dp[(row-1)*COLS + col - 1]*4)&LD_MASK);
        }

        dp[(row+1)*COLS-1] = (((b[(row+1)*COLS_NEWL-2] & CHAR_MASK) as u32) * CHAR_MUL)
            | ((dp[row*COLS-1]*4)&UP_MASK)
            | ((dp[row*COLS-2]*4)&LD_MASK);

        // let dance = _mm_set_epi16(0x0102, 0x0304, 0x0506, 0x0708, 0x090a, 0x0b0c, 0x0d0e, 0x0fff);
        {
            let every_other =_mm_set_epi8(16,1,16,1, 16,1,16,1, 16,1,16,1, 16,1,16,1);

            let every_other2 = _mm_set_epi16(256, 1, 256, 1, 256, 1, 256, 1);
    
            let lookup  = _mm_set_epi32(0, 0, 0, 0x08040201);
    
            let acc : __m128i;
            let mut col = 0;
            while col < COLS - 32 {
                let chunk = _mm_loadu_si128(b.as_ptr().offset((row*COLS_NEWL+col) as isize) as *const __m128i);
                let chunk4 = _mm_shuffle_epi8(lookup,_mm_srli_epi16(_mm_and_si128(chunk, _mm_set1_epi8(CHAR_MASK as i8)),3));
                let chunk8 = _mm_maddubs_epi16(chunk4, every_other);
                let mut chunk16 = _mm_hadd_epi16(_mm_mullo_epi16(chunk8,every_other2),_mm_set1_epi8(11) );
                let future : &mut [u128] = std::mem::transmute::<_,&mut [u128;1]>(chunk16.borrow_mut());

                future[0] = future[0] * (1 + (1 << 4) + (1<< 8) + (1<<12));

                //let mask = _mm_movemask_epi8(_mm_cmpeq_epi8(_mm_and_si128(chunk16, _mm_set1_epi8(0b11110000u8 as i8)),_mm_set1_epi8(0b11110000u8 as i8)));

                println!("yay! {:#x} {:#x} {:?}", future[0], future[0] * (1 + (1 << 4) + (1<< 8) + (1<<12)) , &input[row*COLS_NEWL+col..row*COLS_NEWL+col+16]);

                col += 13;


            }
        };

        
    }

    // this one doesn't get vectorized for some reason, did it by hand
    // TODO: maybe align this? currently banging out loudu's.
    //let misalignment = dp.as_ptr().offset((row*COLS) as isize).align_offset(align_of::<__m256i>());
    //for i in (1..COLS-misalignment

    for i in (0..(ROWS*COLS)/8) {
            
        let x = _mm256_castps_si256(_mm256_loadu_ps(std::mem::transmute(dp.as_ptr().offset((i*8) as isize ))));
        let m1 :u32 = std::mem::transmute(_mm256_movemask_epi8(_mm256_cmpeq_epi8(x, _mm256_set1_epi8(std::mem::transmute(XMAS)))));
        let m2 : u32 = std::mem::transmute(_mm256_movemask_epi8(_mm256_cmpeq_epi8(x, _mm256_set1_epi8(std::mem::transmute(SAMX)))));
        dvcnt += (m1|m2).count_ones();
    }

    // do the last little bit that doesn't divide
    for i in (((ROWS*COLS)/8) * 8..ROWS*COLS) {
        let x : [u8;4] = std::mem::transmute(dp[i]);
        for j in (0..4) {
            dvcnt += (x[j] == XMAS) as u32;
            dvcnt += (x[j] == SAMX) as u32;
        }
    }

    


    //print_DP(dp[139*140]);

    //println!("dvcnt: {} hcnt: {}",dvcnt,hcnt);

    return dvcnt+hcnt;
}