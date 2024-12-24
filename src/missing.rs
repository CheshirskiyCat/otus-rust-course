#![allow(dead_code)]
/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/

const UNKNOWN: i32 = -1;

fn missing_num(nums: &[i32]) -> i32 {
    let mut num: i32 = UNKNOWN;
    let len = nums.len() as i32;
    for i in 0.. len {
        num = UNKNOWN;
        'inner: for j in 0..len {
            if i == nums[j as usize] {
                num = i;
                break 'inner;
            }
        }
        if num == UNKNOWN {
            num = i;
            break;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}
