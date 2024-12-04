
use core::str;

/* 
const X : u32 = 0b11;
const M : u32 = 0b01;
const A : u32 = 0b00;
const S : u32 = 0b10;
*/
const CHAR_MASK : u8 = 0b11000;
const CHAR_MUL : u32 = (1<<5) + (1<<13) + (1<<21);
// const ASSEMBLE_MASK : u32 = 0xfcfcfc00;
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
    
    let mut dp : [u32; ROWS*COLS] = [0; ROWS*COLS];

    // compiler should auto-vectorize this part adequately.. (TODO: CHECK)
    for i in 0..ROWS {
        dp[i] = ((b[i] & CHAR_MASK) as u32) * CHAR_MUL;
    }

    for row in 1..ROWS {
        dp[row*COLS] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
            + ((dp[(row-1)*COLS]*4)&UP_MASK)
            + ((dp[(row-1)*COLS+1]*4)&RD_MASK);

        for col in 1..COLS-1 {
            dp[row*COLS + col] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
                + ((dp[(row-1)*COLS + col]*4)&UP_MASK)
                + ((dp[(row-1)*COLS + col + 1]*4)&RD_MASK)
                + ((dp[(row-1)*COLS + col - 1]*4)&LD_MASK);
        }

        dp[(row+1)*COLS-1] = (((b[row*COLS_NEWL] & CHAR_MASK) as u32) * CHAR_MUL)
            + ((dp[row*COLS-1]*4)&UP_MASK)
            + ((dp[row*COLS-2]*4)&LD_MASK);
    }




    return dp[140*140 - 2];
}