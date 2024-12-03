

const NUM_9 : i32 = b'9' as i32;

const NUM_0 : i32 = b'0' as i32;

const SPACE : i32 = b' ' as i32;

const NEWL : i32 = b'\n' as i32;



#[aoc(day2, part1)]
pub fn part1(input:&str) -> i32 {
    let bs = input.as_bytes();
    let mut len = 0;
    let mut arr : [i32; 16 ] = [0 ; 16];
    let mut valid = 0;
    for bi in (0..bs.len()) {
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
        len = 0;
        for idx in 0..arr.len() {
            arr[idx] = 0;
        }
    }
    valid
}