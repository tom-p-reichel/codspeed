
use core::{num, str};
use std::arch::x86_64::{__m128i, __m256i, _mm256_add_epi8, _mm256_and_si256, _mm256_andnot_pd, _mm256_castps_si256, _mm256_cmpeq_epi8, _mm256_loadu_ps, _mm256_loadu_si256, _mm256_maddubs_epi16, _mm256_movemask_epi8, _mm256_mul_epu32, _mm256_mulhi_epu16, _mm256_mullo_epi16, _mm256_or_si256, _mm256_set1_epi16, _mm256_set1_epi32, _mm256_set1_epi8, _mm256_set_epi32, _mm256_set_epi64x, _mm256_set_m128, _mm256_srli_epi16, _mm256_sub_epi8, _mm256_xor_si256, _mm_load_sd, _mm_load_si128, _mm_loadu_si128, _mm_or_si128, _mm_storeu_si128, _mm_xor_si128};
use std::borrow::BorrowMut;
use crate::hacks::{x128, x256, X128, X256, bit_lut, Aligner};



#[aoc(day5, part1)]
pub fn part1(input:&str) -> u32 {
    let x = unsafe { part1_unsafe(input) };
    x
}


#[inline(always)]
unsafe fn read10(b:&[u8], p:&mut usize, out:&mut [u8]) -> u32 {
    //let syntax = load32(b"__,__,__,__,__,__,__,__,__,__,__" as &[u8;32]);
    let syntax = _mm256_set_epi64x(0x5f5f2c5f5f2c5f5f, 0x2c5f5f2c5f5f2c5f, 0x5f2c5f5f2c5f5f2c, 0x5f5f2c5f5f2c5f5f);
    //let syntax_mask = _mm256_cmpeq_epi8(syntax, _mm256_set1_epi8(b'_' as i8));
    let syntax_mask = _mm256_set_epi64x(0xffff00ffff00ffffu64 as i64 , 0xffff00ffff00ff,0xff00ffff00ffff00u64 as i64, 0xffff00ffff00ffffu64 as i64);
    let chunk;
    if b.len() < 32+*p {
        let mut chunk_proto : [u8;32]  = [0 ;32];

        chunk_proto[0..b.len()-*p].copy_from_slice(&b[*p..b.len()]);

        chunk = std::mem::transmute(chunk_proto);
        
    } else {
        chunk = load32(&b[*p..]);
    }
    
    let valid = _mm256_movemask_epi8(_mm256_or_si256(_mm256_cmpeq_epi8(chunk, syntax),syntax_mask)) as u32;

    let to = valid.trailing_ones();
    let len = std::cmp::min((1+to)/3,10);

    let coef = (1<<8) + (10<<16);
    
    let mut vals : [u64;4] = x256(_mm256_and_si256(_mm256_sub_epi8(chunk, _mm256_set1_epi8(b'0' as i8)),syntax_mask));

    vals[3] = vals[3].overflowing_mul(coef).0 + (((vals[2] >> 32)*coef)>>32); 
    vals[2] = vals[2].overflowing_mul(coef).0 + (((vals[1] >> 32)*coef)>>32);
    vals[1] = vals[1].overflowing_mul(coef).0 + (((vals[0] >> 32)*coef)>>32);
    vals[0] = vals[0].overflowing_mul(coef).0 ;
    
    let char_view: *const u8 = vals.as_ptr().cast();
    
    for i in (0..len) {
        out[i as usize] = *char_view.offset((i*3+2) as isize);
    }

    *p += std::cmp::min(30,to as usize); // (3*len) as usize;

    len
} 


#[inline(always)]
unsafe fn load32(a:&[u8]) -> __m256i {
    return _mm256_loadu_si256(a.as_ptr() as *const __m256i)
}


#[inline(always)]
pub unsafe fn setbit(x:&mut [u64;2], y:u32) {
    //*x.get_unchecked_mut((y/64) as usize) |= 1  << (y%64);
    /* 
    x[0] |= bit_lut[y as usize][0];
    x[1] |= bit_lut[y as usize][1];
    */
    let mask = _mm_load_si128( &x128(*bit_lut.as_ptr().offset(y as isize)));
    let other = _mm_loadu_si128(std::mem::transmute(x as & [u64;2]));

    //_mm_storeu_si128(x.as_mut_ptr() as *mut __m128i, _mm_or_si128(mask, other));
    *x = std::mem::transmute(_mm_or_si128(mask, other));
}

#[inline(always)]
pub unsafe fn flipbit(x:&mut [u64;2], y:u32) {
    //*x.get_unchecked_mut((y/64) as usize) ^= 1  << (y%64);
    /*
    x[0] ^= bit_lut[y as usize][0];
    x[1] ^= bit_lut[y as usize][1];
    */
    let mask = _mm_load_si128( &x128(*bit_lut.as_ptr().offset(y as isize)));
    let other = _mm_loadu_si128(std::mem::transmute(x as & [u64;2]));

    _mm_storeu_si128(x.as_mut_ptr() as *mut __m128i, _mm_xor_si128(mask, other));

}



#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part1_unsafe(input:&str) -> u32 {

    let mut aligned_gt = Aligner::<[[u64;2]; 128]> {thing :[[0;2]; 128]};
    let mut aligned_lt = Aligner::<[[u64;2]; 128]> {thing :[[0;2]; 128]};


    let gt : &mut [[u64;2]; 128] = &mut aligned_gt.thing;
    let lt : &mut [[u64;2]; 128] = &mut aligned_lt.thing;

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

            let chunkp1 = load32(&b[p+1..]);
            
            let valid = _mm256_movemask_epi8(_mm256_or_si256(_mm256_cmpeq_epi8(chunk, syntax),syntax_mask)) as u32;
            
            let len = valid.trailing_ones()/6;

            let mut vals : [u64;4] = x256(_mm256_add_epi8(
                _mm256_mullo_epi16(_mm256_and_si256(_mm256_sub_epi8(chunk, _mm256_set1_epi8(b'0' as i8)),syntax_mask),_mm256_set1_epi16(10)),
                _mm256_sub_epi8(chunkp1, _mm256_set1_epi8(b'0' as i8))
            ));

            let char_view: *const u8 = vals.as_ptr().cast();
            
            for i in (0..len) {
                let a = *char_view.offset(((6*i) as isize));
                let b = *char_view.offset(((6*i+3) as isize));

                //gt[a as usize] = (x128::<_,u128>(gt[a as usize]) | (1<<b)).into();
                                
                setbit(&mut lt[b as usize],a.into());
                
                setbit(&mut gt[a as usize],b.into());


                //lt[b as usize] = (x128::<_,u128>(lt[b as usize]) | (1<<b)).into();
                //println!("{}|{}",a,b);


            }

            p += (6*len) as usize;

            if valid != 0xffffffff {
                break;
            }
        };
    }

    p += 1;
    
    let mut line : [u8; 25] = [0;25];

    let mut medians = 0;
    let mut idx = 0;

    loop {
        // parse a line...
        let mut line_cnt :usize = 0;

        let mut read = read10(b, &mut p, &mut line);

        line_cnt += read as usize;

        while read == 10 && b[p] != b'\n'{
            // we read 10 full ints. try more.
            read = read10(b,&mut p, &mut line[line_cnt as usize..] );
            line_cnt += read as usize;
        }

        //println!("read {}",line_cnt);

        //println!("{:?}",&line[0..line_cnt as usize]);

        // finally, we can actually do the problem
        let mut line_vec : [u64;2] = [0;2];

        let mut sorted : bool = true;


        for i in (0..line_cnt) {


            for j in (0 as usize..2) {
                sorted &= lt[line[i] as usize][j] & line_vec[j] == line_vec[j] ;
            }
            if !sorted {
                break
            }
            setbit(&mut line_vec, line[i as usize] as u32);


        }

        // println!("{} {}",idx,sorted);
        idx += 1;

        if sorted {
            medians += line[line_cnt/2 ] as u32;
        }

        p += 1;

        if p >= b.len() {
            break
        }
    }

    medians
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
pub unsafe fn part2_unsafe(input:&str) -> u32 {

    let mut aligned_gt = Aligner::<[[u64;2]; 128]> {thing :[[0;2]; 128]};
    let mut aligned_lt = Aligner::<[[u64;2]; 128]> {thing :[[0;2]; 128]};


    let gt : &mut [[u64;2]; 128] = &mut aligned_gt.thing;
    let lt : &mut [[u64;2]; 128] = &mut aligned_lt.thing;

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

            let chunkp1 = load32(&b[p+1..]);
            
            let valid = _mm256_movemask_epi8(_mm256_or_si256(_mm256_cmpeq_epi8(chunk, syntax),syntax_mask)) as u32;
            
            let len = valid.trailing_ones()/6;

            let mut vals : [u64;4] = x256(_mm256_add_epi8(
                _mm256_mullo_epi16(_mm256_and_si256(_mm256_sub_epi8(chunk, _mm256_set1_epi8(b'0' as i8)),syntax_mask),_mm256_set1_epi16(10)),
                _mm256_sub_epi8(chunkp1, _mm256_set1_epi8(b'0' as i8))
            ));

            let char_view: *const u8 = vals.as_ptr().cast();
            
            for i in (0..len) {
                let a = *char_view.offset(((6*i) as isize));
                let b = *char_view.offset(((6*i+3) as isize));

                //gt[a as usize] = (x128::<_,u128>(gt[a as usize]) | (1<<b)).into();
                                
                setbit(&mut lt[b as usize],a.into());
                
                setbit(&mut gt[a as usize],b.into());


                //lt[b as usize] = (x128::<_,u128>(lt[b as usize]) | (1<<b)).into();
                //println!("{}|{}",a,b);


            }

            p += (6*len) as usize;

            if valid != 0xffffffff {
                break;
            }
        };
    }

    p += 1;
    
    let mut line : [u8; 25] = [0;25];

    let mut medians = 0;
    let mut idx = 0;

    loop {
        // parse a line...
        let mut line_cnt :usize = 0;

        let mut read = read10(b, &mut p, &mut line);

        line_cnt += read as usize;

        while read == 10 && b[p] != b'\n'{
            // we read 10 full ints. try more.
            read = read10(b,&mut p, &mut line[line_cnt as usize..] );
            line_cnt += read as usize;
        }

        //println!("read {}",line_cnt);

        //println!("{:?}",&line[0..line_cnt as usize]);

        // finally, we can actually do the problem
        let mut line_vec : [u64;2] = [0;2];

        let mut sorted : bool = true;


        for i in (0..line_cnt) {


            for j in (0 as usize..2) {
                sorted &= lt[line[i] as usize][j] & line_vec[j] == line_vec[j] ;
            }


            setbit(&mut line_vec, line[i as usize] as u32);


        }

        // println!("{} {}",idx,sorted);
        idx += 1;

        if !sorted {
            for i in (0..line_cnt) {
                let below = (line_vec[0] & lt[line[i] as usize][0]).count_ones() + (line_vec[1] & lt[line[i] as usize][1]).count_ones(); 
                if below == (line_cnt / 2) as u32 {
                    medians += line[i] as u32;
                    break
                }
            }
        }

        p += 1;

        if p >= b.len() {
            break
        }
    }

    medians
}




#[aoc(day5, part2)]
pub fn part2(input:&str) -> u32 {
    let x = unsafe { part2_unsafe(input) };
    x
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1_example() {
        assert_eq!(part1("11|22\n22|33\n11|33\n\n11,22,33\n"), 22);
    }

    /* 
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}