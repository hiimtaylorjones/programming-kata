use std::thread;

pub fn chop(int: i32, array: &mut [i32]) -> i32{
    let mut count = 0;
    for x in array.iter() {
        if x == &int {
            return count;
        }
        count += 1;
    }
    return -1;
}

pub fn binary_chop(int: i32, array: &mut [i32]) -> i32 {
    let half = (array.len() / 2) as i32;
    let full = array.len() as i32;

    println!("{:?}", half);
    println!("{:?}", full);
    thread::spawn(|| {
        let mut count = 0;
        for x in 0..half {
            let index = x as usize;
            if array[index] == int {
                return index;
            }
        }
    });
    thread::spawn(|| {
        let mut count = 0;
        for x in half..full {
            let index = x as usize;
            if array[index] == int {
                return index;
            }
        }
    });
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_chop() {
        assert_eq!(-1, chop(3, &mut []));
        assert_eq!(-1, chop(3, &mut [1]));
        assert_eq!(0,  chop(1, &mut [1]));
        //
        assert_eq!(0,  chop(1, &mut [1, 3, 5]));
        assert_eq!(1,  chop(3, &mut [1, 3, 5]));
        assert_eq!(2,  chop(5, &mut [1, 3, 5]));
        assert_eq!(-1, chop(0, &mut [1, 3, 5]));
        assert_eq!(-1, chop(2, &mut [1, 3, 5]));
        assert_eq!(-1, chop(4, &mut [1, 3, 5]));
        assert_eq!(-1, chop(6, &mut [1, 3, 5]));
        // //
        assert_eq!(0,  chop(1, &mut [1, 3, 5, 7]));
        assert_eq!(1,  chop(3, &mut [1, 3, 5, 7]));
        assert_eq!(2,  chop(5, &mut [1, 3, 5, 7]));
        assert_eq!(3,  chop(7, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(0, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(2, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(4, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(6, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(8, &mut [1, 3, 5, 7]));
    }

    #[test]
    fn test_binary_chop() {
        assert_eq!(-1, binary_chop(3, &mut []));
        assert_eq!(-1, binary_chop(3, &mut [1]));
        assert_eq!(0,  binary_chop(1, &mut [1]));
        //
        assert_eq!(0,  chop(1, &mut [1, 3, 5]));
        assert_eq!(1,  chop(3, &mut [1, 3, 5]));
        assert_eq!(2,  chop(5, &mut [1, 3, 5]));
        assert_eq!(-1, chop(0, &mut [1, 3, 5]));
        assert_eq!(-1, chop(2, &mut [1, 3, 5]));
        assert_eq!(-1, chop(4, &mut [1, 3, 5]));
        assert_eq!(-1, chop(6, &mut [1, 3, 5]));
        // // //
        assert_eq!(0,  chop(1, &mut [1, 3, 5, 7]));
        assert_eq!(1,  chop(3, &mut [1, 3, 5, 7]));
        assert_eq!(2,  chop(5, &mut [1, 3, 5, 7]));
        assert_eq!(3,  chop(7, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(0, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(2, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(4, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(6, &mut [1, 3, 5, 7]));
        assert_eq!(-1, chop(8, &mut [1, 3, 5, 7]));
    }
}
