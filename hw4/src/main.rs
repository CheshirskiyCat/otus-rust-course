fn main() {
    let result = library::get_tuple_element((2, 4), false);
    println!("{:?}", result);

    let mut vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice: &mut [i32] = &mut vector[2..5];
    let result2 = library::get_mutable_link_on_element(slice, 2);
    *result2 = 15;
    println!("{:?}", vector);

    let slice3 = &vector[2..8];
    let result3 = library::get_link_on_element(slice3, 0);
    println!("{:?}", result3);
    let result3 = library::get_link_on_element(slice3, 5);
    println!("{:?}", result3);

    let slice4 = &vector[1..7];
    let result4 = library::divide_slice_into_two_part(slice4, 3);
    println!("{:?} | {:?}", result4.0, result4.1);

    let result5 = library::divide_slice_into_multiple_part(&vector[0..9]);
    println!("{:?}", result5);

    let result6 = library::divide_slice_into_multiple_part(&vector[0..2]);
    println!("{:?}", result6);

    let resul7 = library::divide_slice_into_multiple_part(&[]);
    println!("{:?}", resul7);
}
