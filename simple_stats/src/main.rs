use std::collections::HashMap;

fn main() {
    let list_of_integers = vec![3,5,7,1204,38,28,180,1,2,45,2,3,5,25,3,7,6,543,4,7,3,3,4,5,62,4,2];

    println!("{:?}",list_of_integers);

    println!("mean is: {}",mean(&list_of_integers));

    println!("median is: {}",median(&list_of_integers));

    println!("mode is: {}",mode(&list_of_integers));
}

fn mean(x: &Vec<u32>) -> f32 {
    let mut sum: f32 = 0.0;
    for item in x.iter() {
        sum += *item as f32;
    }
    sum/x.len() as f32
}

fn median(x: &Vec<u32>) -> f32 {
    let mut sorted_vec = x.clone();
    sorted_vec.sort();
    let length = sorted_vec.len();
    if length % 2 == 1 {
        mean(&vec![sorted_vec[(length+1)/2],sorted_vec[(length-1)/2]])
    } else {
        sorted_vec[length/2] as f32
    }
}

fn mode(x: &Vec<u32>) -> u32 {
    1
}
