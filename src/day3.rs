
use core::str;
use std::{arch::x86_64::{__m128i, __m256i, __m512i, _mm256_cmpeq_epi16, _mm256_cmpeq_epi8, _mm256_movemask_epi8, _mm256_set1_epi8, _mm_adds_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_cmplt_epi8, _mm_extract_epi16, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16, _mm_movemask_epi8, _mm_set1_epi8, _mm_setr_epi32, _mm_setr_pd, _mm_setr_ps}, str::Bytes};

const MATCH_MASK : [__m128i; 65536]  = unsafe{ std::mem::transmute(*include_bytes!("day3_bins/day3_match_mask.bin")) };
const MATCH_VALS : [__m128i; 65536]  = unsafe{ std::mem::transmute(*include_bytes!("day3_bins/day3_match_vals.bin")) };
const OP1_MASK : [__m128i; 65536]  = unsafe{ std::mem::transmute(*include_bytes!("day3_bins/day3_op1_mask.bin")) };
const OP2_MASK : [__m128i; 65536]  = unsafe{ std::mem::transmute(*include_bytes!("day3_bins/day3_op2_mask.bin")) };



pub fn handle_m(b:&[u8], i:usize) -> i32 {
    // how are you supposed to do this nicely
    // also, is this aligned??

    let vec : __m128i;

    if  b.len() <= i + 20 {
    unsafe {
        // slow path for edge case indices
        let tmp = &b[i+4..b.len()];
        let mut tmp2 : [u8; 16] = [0; 16];
        tmp2[0..b.len()-i-4].copy_from_slice(tmp);
        vec = std::mem::transmute(u128::from_le_bytes(tmp2));
    }   
    } else {
    unsafe {
        vec = std::mem::transmute(u128::from_le_bytes(b[i+4..i+20].try_into().unwrap()));
    }
    }   

    if &b[i..i+4] != b"mul(" {
        return 0
    }

    let positive_mask :i32 = 0xffff;

    let nums = unsafe {
    _mm_movemask_epi8(_mm_cmpgt_epi8(vec, _mm_set1_epi8(47))) & _mm_movemask_epi8(_mm_cmplt_epi8(vec, _mm_set1_epi8(58)))
    };

    // todo: why didn't you just save the result of movemask
    let match_mask: i32; 
    unsafe {
    match_mask = _mm_movemask_epi8(MATCH_MASK[nums as usize]);
    };

    let match_vals = MATCH_VALS[nums as usize];
    let op1_mask = OP1_MASK[nums as usize];
    let op2_mask = OP2_MASK[nums as usize];

    let tmp :i32 ;
    let valid : bool;
    unsafe{ 
        tmp = (match_mask | _mm_movemask_epi8(_mm_cmpeq_epi8(match_vals, vec)));
    valid = positive_mask == (match_mask | _mm_movemask_epi8(_mm_cmpeq_epi8(match_vals, vec)));
    }


    if !valid {
        //println!("not valid!\n{:#018b}\n{:#018b}",nums,match_mask);
        return 0;
    }

    // ok, now actually mul
    let res : i32;
    unsafe {
        let a = _mm_maddubs_epi16(_mm_adds_epi8(vec, _mm_set1_epi8(-48)), op1_mask);
        let b = _mm_maddubs_epi16(_mm_adds_epi8(vec, _mm_set1_epi8(-48)), op2_mask);
        let psum = _mm_hadd_epi16(_mm_hadd_epi16(a, b),_mm_set1_epi8(47));
        res = _mm_extract_epi16::<0>(psum) * _mm_extract_epi16::<2>(psum);
        //println!("i think ops are {} and {}", _mm_extract_epi16::<0>(psum), _mm_extract_epi16::<2>(psum));
    }
    res
}

#[aoc(day3, part1)]
pub fn part1(input:&str) -> i32 {
    let x = unsafe { part1_unsafe(input) };
    x
}



#[target_feature(enable = "avx2")]
pub unsafe fn part1_unsafe(input:&str) -> i32 {
    let b = input.as_bytes(); 
    let mut cnter = 0;
    let scan;

    let misalignment =  b.as_ptr().align_offset(align_of::<__m256i>());

    let b_aln = b.as_ptr().add(misalignment);

    scan = (b_aln as *const u8 as *const __m256i);

    for i in (0..  (input.len() -4 - misalignment ) / 32) {
        
        // let mut v =  (((!( unsafe {*scan.offset(i as isize) } ^ MASK_M ) & MAGIC)) + MAGIC2) & (!MAGIC);
        let mut v : i32  = _mm256_movemask_epi8(_mm256_cmpeq_epi8(*scan.offset(i as isize), _mm256_set1_epi8(0x6d)));
        // v.trailing_zeros();
        while v != 0 {
            let tz = v.trailing_zeros();
            v -= 1 << tz;
            let loc = i*32+(tz as usize);
            //println!("st {} tz {} loc {}", tz, &input[loc..loc+20],loc);
            cnter += handle_m(b, loc);
        } 

    }

    // edge cases -- this is actually terribly slow even for the last 32 vals...
    for i in (0 .. misalignment) {
        cnter += handle_m(b, i);
    }

    for i in ((input.len()-4) -(input.len()-4)%32..input.len()-4) {
        cnter += handle_m(b, i);
    }
    cnter
}


#[aoc(day3, part2)]
pub fn part2(input:&str) -> i32 {
    let x = unsafe { part2_unsafe(input) };
    x
}

// laaazy functions
pub fn handle_d(b:&[u8], i:usize) -> Option<bool> {
    if b.len() <= i+4 {
        return None
    }
    if &b[i..i+4] == b"do()" {
        return Some(true)
    }
    if b.len() <= i+7 {
        return None
    }

    if &b[i..i+7] == b"don't()" {
        return Some(false)
    }

    None
}

#[target_feature(enable = "avx2")]
pub unsafe fn part2_unsafe(input:&str) -> i32 {
    let b = input.as_bytes(); 
    let mut cnter = 0;
    let scan;

    let misalignment =  b.as_ptr().align_offset(align_of::<__m256i>());

    let b_aln = b.as_ptr().add(misalignment);

    scan = (b_aln as *const u8 as *const __m256i);

    let mut toggle = true;
    
    for i in (0 .. misalignment) {
        if b[i] == b'm' {
            if toggle {
                cnter += handle_m(b, i)
            }
        } else {
            handle_d(b, i).map(|x| {toggle = x;});
        }
    }


    for i in (0..  (input.len() -4 - misalignment ) / 32) {
        
        // let mut v =  (((!( unsafe {*scan.offset(i as isize) } ^ MASK_M ) & MAGIC)) + MAGIC2) & (!MAGIC);
        let vm : i32  = _mm256_movemask_epi8(_mm256_cmpeq_epi8(*scan.offset(i as isize), _mm256_set1_epi8(0x6d)));
        let vd : i32 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(*scan.offset(i as isize), _mm256_set1_epi8(0x64)));
        let mut v = vm | vd;
        // v.trailing_zeros();
        while v != 0 {
            let tz = v.trailing_zeros();
            v -= 1 << tz;
            let loc = i*32+(tz as usize);
            //println!("st {} tz {} loc {}", tz, &input[loc..loc+20],loc);
            if b[loc] == b'm' {
                if toggle {
                    cnter += handle_m(b, loc)
                }
            } else {
                handle_d(b, loc).map(|x| {toggle = x;});
            }
        } 

    }

    // edge cases -- this is actually terribly slow even for the last 32 vals...

    for i in ((input.len()-4) - ((input.len()-4)%32)..input.len()-4) {
        if b[i] == b'm' {
            if toggle {
                cnter += handle_m(b, i)
            }
        } else {
            handle_d(b, i).map(|x| {toggle = x;});
        }
    }
    cnter
}