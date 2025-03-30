// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

pub struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

pub fn create(size: usize) -> RingBuffer {
    RingBuffer{
        read_idx: 0,
        write_idx: 0,
        data: vec![0; size]
    }
}

pub fn write(rb: &mut RingBuffer, items:Vec<char>) -> usize {
    let mut counter: usize = 0;
    for i in 0..items.len() {
        if rb.data[rb.write_idx] != 0 { break; }
        rb.data[rb.write_idx] = items[i].try_into().expect("wrong char");
        counter += 1;
        rb.write_idx += 1;
        if rb.write_idx == rb.data.len() { rb.write_idx = 0 }
        if rb.read_idx == rb.write_idx { break; }
    }
    counter
}


pub fn read(rb: &mut RingBuffer, items:usize) -> Vec<char> {
    let mut result:Vec<char> = Vec::new();
    for _ in 0..items {
        result.push(rb.data[rb.read_idx].into());
        rb.data[rb.read_idx] = 0;
        rb.read_idx += 1;
        if rb.read_idx == rb.data.len() { rb.read_idx = 0 }
        if rb.data[rb.read_idx] == 0 { break; }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = create(3);
        assert_eq!(buffer.data.len(), 3);
    }

    #[test]
    fn test_write_buffer() {
        let mut buffer = create(3);
        let num = write(&mut buffer, vec!('a', 'b', 'c', 'd'));
        assert_eq!(num, 3);
        assert_eq!(buffer.data, vec!(97, 98, 99));
    }

    #[test]
    fn test_read_buffer() {
        let mut buffer = create(3);
        write(&mut buffer, vec!('a', 'b'));
        let result = read(&mut buffer, 1);
        assert_eq!(result, vec!['a']);
        assert_eq!(buffer.data, vec!(0, 98, 0));
    }
}
