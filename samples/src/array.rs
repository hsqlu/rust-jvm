fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }
}

//创建空Vec
let v: Vec<i32> = Vec::new();
//使用宏创建空Vec
let v: Vec<i32> = vec![];
//创建包含5个元素的Vec
let v = vec![1, 2, 3, 4, 5];
//创建十个零
let v = vec![0; 10];
//创建可变的Vec，并压入元素3
let mut v = vec![1, 2];
v.push(3);
//创建拥有两个元素的Vec，并弹出一个元素
let mut v = vec![1, 2];
let two = v.pop();
//创建包含三个元素的可变Vec，并索引一个值和修改一个值
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;