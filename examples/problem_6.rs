
fn main() {
    let range = 1..=100;

    // Sum of squares
    let mut sum_of_squares: i64 = 0;
    for x in range.to_owned() {
        sum_of_squares += x*x;
    }
    
    // Square of sum
    let mut square_of_sum: i64 = 0;
    for x in range.to_owned() {
        square_of_sum += x;
    }
    square_of_sum *= square_of_sum;

    let result = square_of_sum - sum_of_squares;
    dbg!(result, square_of_sum, sum_of_squares);
}