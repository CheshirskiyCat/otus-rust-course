//!
//! # Библиотека:
//! Реализация функций к домашнему заданию HW-4 "Заимствование"<br/>

/// Количество частей на которые нужно делить слайс.
const PARTS: usize = 4;

/// Возвращает элемент кортежа.
///
/// * `tuple` - мутабельная ссылка на кортеж
/// * `is_right` - показатель какой элемент вернуть. Если false, возвращает мутабельную ссылку
///                на первый элемент кортежа, иначе мутабельную ссылку на второй элемент.
/// * `return` - мутабельную ссылку на элемент кортежа
pub fn get_tuple_element(tuple: (i32, i32), is_right: bool) -> i32 {
    if is_right {
        tuple.0
    } else {
        tuple.1
    }
}

/// Возвращает мутабельную ссылку на элемент слайса.
///
/// * `slice` - слайс с данными
/// * `idx` - индекс в слайсе
/// * `return` - мутабельная ссылка на элемент слайса
pub fn get_mutable_link_on_element(slice: &mut [i32], idx: usize) -> &mut i32 {
    &mut slice[idx]
}

/// Возвращает ссылку на n-ый элемент с конца слайса.
///
/// * `slice` - слайс с данными
/// * `offset` - отступ от конца слайса
/// * `return` - ссылка на N-ый элемент слайса с конца
pub fn get_link_on_element(slice: &[i32], offset: usize) -> &i32 {
    if offset >= slice.len() {
        panic!(
            "offset out of bounds [offset {}, slice length: {} ]",
            offset,
            slice.len()
        );
    }
    &slice[slice.len() - offset - 1]
}

/// Делит исходный слайс на два слайса по индексу переданному в параметре.
///
/// * `slice` - слайс с данными
/// * `idx` - индекс по которому будет произведен раздел
/// * `return` - кортеж с половинками исходного слайса. Первый слайс с с нулевого по N-1,
///              второй с N-го по последний.
pub fn divide_slice_into_two_part(slice: &[i32], idx: usize) -> (&[i32], &[i32]) {
    let first = &slice[..idx];
    let second = &slice[idx..];
    (first, second)
}

/// Делит исходный слайс на равные части и возвращает полученные слайсы в массиве.
///
/// * `slice` - слайс с данными
/// * `return` - массив слайсов.
pub fn divide_slice_into_multiple_part(slice: &[i32]) -> [&[i32]; PARTS] {
    let mut result: [&[i32]; PARTS] = [&[]; PARTS];
    let base = slice.len() / PARTS;
    let addition = slice.len() % PARTS;
    let mut left_border = 0;
    let get_chunk = |idx: usize| -> usize { base + if addition > idx { 1 } else { 0 } };
    for i in 0..PARTS {
        if left_border == slice.len() {
            break;
        }
        if i == PARTS - 1 {
            result[PARTS - 1] = &slice[left_border..];
        } else {
            let right_border = left_border + get_chunk(i);
            result[i] = &slice[left_border..right_border];
            left_border = right_border;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_get_tuple_element() {
        let result = get_tuple_element((2, 4), true);
        assert_eq!(result, 2);
        let result = get_tuple_element((2, 4), false);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_function_get_mutable_link_on_element() {
        let mut vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let slice: &mut [i32] = vector[2..5].as_mut();
        let result = get_mutable_link_on_element(slice, 2);
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_function_get_link_on_element() {
        let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let slice = &vector[2..8];
        let result = get_link_on_element(slice, 1);
        assert_eq!(*result, 8);
    }

    #[test]
    fn test_function_divide_slice_into_two_part() {
        let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let slice = &vector[1..7];
        let result = divide_slice_into_two_part(slice, 3);
        assert_eq!(result.0, vec![2, 3, 4]);
        assert_eq!(result.1, vec![5, 6, 7]);
    }

    #[test]
    fn test_function_divide_slice_into_four_part() {
        let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = divide_slice_into_multiple_part(&vector[0..9]);
        assert_eq!(
            result,
            [&vec![1, 2, 3], &vec![4, 5], &vec![6, 7], &vec![8, 9]]
        );
        let result = divide_slice_into_multiple_part(&vector[0..7]);
        assert_eq!(result, [&vec![1, 2], &vec![3, 4], &vec![5, 6], &vec![7]]);
        let result = divide_slice_into_multiple_part(&vector[0..2]);
        assert_eq!(result, [&vec![1], &vec![2], &vec![], &vec![]]);
        let result = divide_slice_into_multiple_part(&[]);
        assert_eq!(result, [&[], &[], &[], &[]]);
    }
}
