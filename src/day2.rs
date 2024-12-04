

const NUM_9 : i32 = b'9' as i32;

const NUM_0 : i32 = b'0' as i32;

const NEWL : i32 = b'\n' as i32;



#[aoc(day2, part1)]
pub fn part1(input:&str) -> i32 {
    let bs = input.as_bytes();
    let mut len = 0;
    let mut arr : [i32; 16 ] = [0 ; 16];
    let mut valid = 0;
    for bi in 0..bs.len() {
        //let c : i32 = unsafe { (*bs.get_unchecked(bi)).into() } ;
        let c : i32 = unsafe { *bs.get_unchecked(bi) }.into();
        let isnum = c >= NUM_0 && c <= NUM_9;
        unsafe { *arr.get_unchecked_mut(len) =  (*arr.get_unchecked(len)) + (isnum as i32)*((*arr.get_unchecked(len))*9 + (c - NUM_0)) } ;
        //arr[len] =  arr[len] + (isnum as i32)*(arr[len]*9 + (c - NUM_0)) ;

        len += (!isnum) as usize;
        if c == NEWL {
            let dir = unsafe { ((*arr.get_unchecked(0)) - (*arr.get_unchecked(1))).signum() } ;
            //let dir = (arr[0] - arr[1]).signum();
            let mut cvalid = true;
            for i in 0..len-1 {
                //let cdiff =  arr[i] - arr[i+1];
                let cdiff = unsafe {* arr.get_unchecked(i) - *arr.get_unchecked(i+1) };
                cvalid &= cdiff.signum() == dir && cdiff.abs() < 4 && cdiff.abs() >0;
            }
            valid += cvalid as i32;
            len = 0;
            for idx in 0..arr.len() {
                arr[idx] = 0;
            }
        }

    }
    if len != 0 {
        //let dir = unsafe { ((*arr.get_unchecked(0)) - (*arr.get_unchecked(1))).signum() } ;
        let dir = (arr[0] - arr[1]).signum();
        let mut cvalid = true;
        for i in 0..len-1 {
            let cdiff =  arr[i] - arr[i+1];
            cvalid &= cdiff.signum() == dir && cdiff.abs() < 4 && cdiff.abs() >0;
        }
        valid += cvalid as i32;
        for idx in 0..16 {
            arr[idx] = 0;
        }
    }
    valid
}

pub fn check_pair(a:i32, b:i32, dir :i32) -> bool {
    let diff = a - b;
    return diff.signum() == dir && diff.abs() > 0 && diff.abs() < 4;
}

pub fn p2in(len_b:&usize, arr_b:&[i32 ; 16]) -> bool {
    let arr = *arr_b;
    let len = *len_b;
    let dir;
    {
        let a = (arr[0] - arr[1]).signum();
        let b = (arr[1] - arr[2]).signum();
        let c = (arr[2] - arr[3]).signum();
        dir = if a==b {a} else {if b==c {b} else {if c==a {c} else {0}}};
    }
    //let dir = unsafe { ((*arr.get_unchecked(0)) - (*arr.get_unchecked(len-1))).signum() } ;
    //let dir = (arr[0] - arr[1]).signum();
    let mut violations : i32 = 0;
    let mut violation_idx : [i32 ; 3] = [0 ; 3];
    for i in 0..len-1 {
        //let cdiff =  arr[i] - arr[i+1];
        let cdiff = unsafe {* arr.get_unchecked(i) - *arr.get_unchecked(i+1) };
        unsafe { *violation_idx.get_unchecked_mut(violations as usize) = i as i32 } ;
        violations += (cdiff.signum() != dir || cdiff.abs() >= 4 || cdiff.abs() <= 0) as i32;
        if violations >= 3 {
            return false
        }

    }
    if violations == 1 {
        let idx = violation_idx[0];
        if idx == 0 || idx as usize == len-2 {
            return true
        }
        return check_pair(arr[idx as usize], arr[(idx+2) as usize], dir) || check_pair(arr[(idx-1) as usize], arr[(idx+1) as usize], dir);
    }
    if violations == 2{
        if violation_idx[1] - violation_idx[0] != 1 {
            return false
        }
        return check_pair(arr[violation_idx[0] as usize], arr[(violation_idx[0]+2) as usize], dir);
    }

    true
}

#[aoc(day2, part2)]
pub fn part2(input:&str) -> i32 {
    let bs = input.as_bytes();
    let mut len = 0;
    let mut arr : [i32; 16 ] = [0 ; 16];
    let mut valid = 0;
    for bi in 0..bs.len() {
        //let c : i32 = unsafe { (*bs.get_unchecked(bi)).into() } ;
        let c : i32 = unsafe { *bs.get_unchecked(bi) }.into();
        let isnum = c >= NUM_0 && c <= NUM_9;
        unsafe { *arr.get_unchecked_mut(len) =  (*arr.get_unchecked(len)) + (isnum as i32)*((*arr.get_unchecked(len))*9 + (c - NUM_0)) } ;
        //arr[len] =  arr[len] + (isnum as i32)*(arr[len]*9 + (c - NUM_0)) ;

        len += (!isnum) as usize;
        if c == NEWL {
            //println!("{:?}, {}",arr,p2in(&len, &arr));
            // /assert_eq!(p2in(&len, &arr),p2in_dumb(&len, &arr));
            valid += p2in(&len, &arr) as i32;
            len = 0;
            for idx in 0..16 {
                arr[idx] = 0;
            }
            
        }

    }
    if len != 0 {
        //let dir = unsafe { ((*arr.get_unchecked(0)) - (*arr.get_unchecked(1))).signum() } ;
        valid += p2in(&len, &arr) as i32;
    }
    valid
}