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
    let half = array.len() / 2;
    let full = array.len();
    let result = search_array(int, array, 0, half);

    if (result != -1) {
        return result;
    }

    let result = search_array(int, array, half, full);
    if (result != -1) {
        return result;
    }

    return -1;
}

fn search_array(int: i32, array: &mut [i32], start: usize, stop: usize) -> i32 {
    for x in start..stop {
        if array[x] == int {
            return x as i32;
        }
    }
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
        let mut empty: [i32; 0] = [];
        let mut one: [i32; 1] = [1];
        let mut one_three_five: [i32; 3] = [1, 3, 5];
        let mut all: [i32; 4] = [1, 3, 5, 7];

        assert_eq!(-1, binary_chop(3, &mut empty));
        assert_eq!(-1, binary_chop(3, &mut one));
        assert_eq!(0,  binary_chop(1, &mut one));

        assert_eq!(0,  chop(1, &mut one_three_five));
        assert_eq!(1,  chop(3, &mut one_three_five));
        assert_eq!(2,  chop(5, &mut one_three_five));
        assert_eq!(-1, chop(0, &mut one_three_five));
        assert_eq!(-1, chop(2, &mut one_three_five));
        assert_eq!(-1, chop(4, &mut one_three_five));
        assert_eq!(-1, chop(6, &mut one_three_five));

        assert_eq!(0,  chop(1, &mut all));
        assert_eq!(1,  chop(3, &mut all));
        assert_eq!(2,  chop(5, &mut all));
        assert_eq!(3,  chop(7, &mut all));
        assert_eq!(-1, chop(0, &mut all));
        assert_eq!(-1, chop(2, &mut all));
        assert_eq!(-1, chop(4, &mut all));
        assert_eq!(-1, chop(6, &mut all));
        assert_eq!(-1, chop(8, &mut all));
    }
}
