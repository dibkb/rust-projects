fn main() {

    let vec1 = vec![1,2,3,4,5];
    let vec2: Vec<_> = vec1.iter().map(|x| x*2).collect();
    let x = 34;
    let equal_to_x_closure = |y:u32| y == x;
    println!("{:?}",vec2);
}

