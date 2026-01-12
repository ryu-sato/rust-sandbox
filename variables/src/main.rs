use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    // コンパイルエラー
    // println!("The value of the element at index 10 is: {}", a[10]);

    println!("Please enter an array index.");
           // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
              // 入力された値は数字ではありません

    let element = a[index];

    // 5以上になるとパニック
    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
}
