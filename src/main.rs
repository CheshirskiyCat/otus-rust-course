// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
//иции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея или ввел неправильный цвет, то вам нужно кинуть панику!

use std::io;
use const_format::concatcp;

mod matrix;
use matrix::Matrix;

struct Display {
    width: u32,
    height: u32,
    matrix: Matrix,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        width: max_width,
        height: max_height,
        matrix: Matrix::new(max_width, max_height, default_colour),
    }
}

const WRONG_FORMAT: &'static str = "Wrong data format";
const WRONG_COORDINATE: &'static str = concatcp!(WRONG_FORMAT,": missing point coordinate");
const WRONG_COLOR: &'static str = concatcp!(WRONG_FORMAT,": missing point color");
const WRONG_PARAMETER: &'static str = concatcp!(WRONG_FORMAT,": undefined parameter");
const POINT_NOT_VALID: &'static str = "Point is not valid";
const POINT_WAS_NOT_SET: &'static str = "Point was not set";

struct Point{
    x: u64,
    y: u64,
}

// Корректно ли проверять при создании структуры

// impl Point {
//
//     fn new(x: u64, y: u64, display: Display) -> Point {
//         if Point::check(x, y, display){
//             Point { x, y }
//         } else {
//             panic!("Point is not valid");
//         }
//     }
//
//     fn check(x: u64, y: u64, display: Display) -> bool {
//         display.height as u64 <= y || display.width as u64 <= x
//     }
// }

impl Point {

    fn check(&self, display: &Display) -> bool {
        display.height as u64 <= self.y || display.width as u64 <= self.x
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    // ваш код сюда
    let mut i = 0;
    let mut point = None;
    while let Some(command) = input.get(i) {
        match command {
            1 => {
                point = Some(Point {
                    x: *input.get(i + 2).expect(WRONG_COORDINATE),
                    y: *input.get(i + 1).expect(WRONG_COORDINATE),
                });
                i += 3;
            }
            2 => {
                let Some(point) = &point else {
                    panic!("{}", POINT_WAS_NOT_SET);
                };
                if point.check(display) {
                    panic!("{}", POINT_NOT_VALID);
                }
                let color = *input.get(i + 1).expect(WRONG_COLOR) as u8;
                display.matrix.set_colour(point.x, point.y, color);
                // point = None;
                i += 2;
            }
            _ => panic!("{}", WRONG_PARAMETER)
        }
    };
}

// код ниже трогать не нужно, можете просто посмотреть его

// тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_case() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 3]);
        let mut expected = Matrix::new(4, 4, 1);
        expected.set_colour(2, 2, 3);
        assert_eq!(display.matrix, expected);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 5, 5, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_error_invalid_colour() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 5]);
    }
}

fn main() {
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}