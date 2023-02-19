fn main() {
    let vec: Vec<i32> = vec![1, 2, 2];

    println!("{}", square_sum(&vec));
    println!("{}", complex_square_sum(&vec));
}

fn square_sum(vec: &Vec<i32>) -> i32 {
    let mut output = 0;

    for num in vec {
        output += num * num;
    }

    return output;
}

fn complex_square_sum(vec: &Vec<i32>) -> i32 {
    return vec.iter().map(|num| num * num).sum();
}
