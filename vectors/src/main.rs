fn main() {
    let _v1: Vec<i32> = Vec::new();

    let mut _v2 = vec![1, 2, 3];
    _v2.push(4);

    let v3 = vec![1, 2, 3, 4, 5, 6];

    let third_element = &v3[2];
    println!("The third element is {third_element}");

    let third_option = v3.get(2);
    if let Some(&num) = third_option {
        print!("The third element is {num}");
    }

    let mut v4 = vec![1, 2, 3, 4, 5];
    for i in &mut v4 {
        *i = 3;
    }
}
