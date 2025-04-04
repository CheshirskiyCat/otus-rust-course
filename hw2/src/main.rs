fn main() {
    println!("double_int32 {}", double_int32(1));
    println!("double_int64 {}", double_int64(2_000_000_000));
    println!("double_float32 {}", double_float32(3f32));
    println!("double_float64 {}", double_float64(4f32));
    println!("int_plus_float_to_float {}", int_plus_float_to_float(5, 6f32));
    println!("int_plus_float_to_int {}", int_plus_float_to_int(7, 8f32));
    println!("tuple_sum {}", tuple_sum((9, 10)));
    println!("array_sum {}", array_sum([11, 12, 13]));
}
/**
 * функция double_int32 принимает 32-х битное целое знаковое число и возвращает 32-х битное целое
 * знаковое число, равное удвоенному входному.
 */
fn double_int32(a: i32) -> i32 {
    a * 2
}

/**
 * функция double_int64 принимает 32-х битное целое знаковое число и возвращает 64-х битное целое
 * знаковое число, равное удвоенному входному.
 */
fn double_int64(a: i32) -> i64 {
    a as i64 * 2_i64
}

/**
 * функция double_float32 принимает 32-х битное число с плавающей точкой и возвращает 32-х битное
 * число с плавающей точкой, равное удвоенному входному.
 */
fn double_float32(a: f32) -> f32 {
    a * 2_f32
}

/**
 * функция double_float64 принимает 32-х битное число с плавающей точкой и возвращает 64-х битное
 * число с плавающей точкой, равное удвоенному входному.
 */
fn double_float64(a: f32) -> f64 {
    a as f64 * 2_f64
}

/**
 * функция int_plus_float_to_float принимает 32-х битное целое знаковое число и 32-х битное число
 * с плавающей точкой. Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
 */
fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
    a as f64 + b as f64
}

/**
 * функция int_plus_float_to_int принимает 32-х битное целое знаковое число и 32-х битное число
 * с плавающей точкой. Возвращает 64-х битное целое знаковое число, равное сумме входных.
 */
fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
    a as i64 + b as i64
}

/**
 * функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме
 * чисел во входном кортеже.
 */
fn tuple_sum(a: (i32, i32)) -> i32 {
    a.0 + a.1
}

/**
 * функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме
 * чисел во входном массиве.
 */
fn array_sum(a: [i32; 3]) -> i32 {
    a.iter().sum()
}
