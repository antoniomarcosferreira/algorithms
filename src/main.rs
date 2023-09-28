use crate::faang::two_sum::two_sum;

mod faang;

fn main() {
    let two_sum_res = two_sum(vec!(2, 7, 11, 15), 9);
    println!("Two sum return: {:?}", two_sum_res);
}
