use std::collections::HashMap;

fn main() {
    let list_of_integers = vec![3,5,7,1204,38,28,180,1,2,45,2,3,5,25,3,7,6,543,4,7,3,3,4,5,62,2];

    println!("{:?}",list_of_integers)
    println!("mean is: {}",mean(&list_of_integers));
}

fn mean(x: &Vec<u32>) -> f32 {
    let mut sum: f32 = 0.0;
    for item in x.iter() {
        sum += *item as f32;
    }
    sum/x.len() as f32
}