// Обобщаем поведение с помощью шаблонов и статического полиморфизма.
// Пусть у нас 3 типа фигур: треугольник, прямоугольник и круг
// Создайте трейт Shape, в котором есть методы:
// get_area(&self) -> f64 // возвращает зачение площади фигуры
// get_perimeter(&self) -> f64 // возвращает значение периметра фигуры
// Реализуйте данный трейт для треугольника, прямоугольника и круга

// Напишите 1 функцию perimeter_by_area, которая может принимать любую фигуру
// и возвращает отнощение ее периметра к площади (P/A)

use std::f64::consts::PI;

trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let p: f64 = self.sides_lens.into_iter().sum::<f64>() / 2.0;
        /*
           Запутался с типами. Вариант со scan-ом работает, но какой-то дикий перегон одних типов
           в другие и обратно, какая-то дичь. Может как-то можно переписать по-человечески?
        */
        // let multiplication: Box<[f64]> = self.sides_lens.iter()
        //     .scan(p, |state, &x| { *state *= p - x; return Some(*state) } )
        //     .collect();
        // let (total, _) = multiplication.split_last().unwrap();
        // f64::sqrt(*total)
        f64::sqrt(
            p * (p - self.sides_lens[0]) * (p - self.sides_lens[1]) * (p - self.sides_lens[2]),
        )
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64 {
        (2f64 * self.width) + (2f64 * self.height)
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn get_perimeter(&self) -> f64 {
        2f64 * PI * self.radius
    }
}

// исправьте сигнатуру и добавьте реализацию
fn perimeter_by_area<T: Shape>(shape: T) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}

fn main() {
    println!(
        "{:?}",
        perimeter_by_area(Triangle {
            sides_lens: [3.0, 4.0, 5.0]
        })
    );
    println!(
        "{:?}",
        perimeter_by_area(Rectangle {
            width: 2.0,
            height: 3.0
        })
    );
    println!("{:?}", perimeter_by_area(Circle { radius: 2.0 }));
}
