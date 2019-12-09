fn main() {
    let entries = 367479..=893698;

    let possible_passwords = entries
        .map(digits)
        .filter(|x| 
            x[0] <= x[1] &&
            x[1] <= x[2] &&
            x[2] <= x[3] &&
            x[3] <= x[4] &&
            x[4] <= x[5] )
        .filter(|x| 
            (x[0] == x[1] && x[1] != x[2]) ||
            (x[1] == x[2] && (x[0] != x[1] && x[2] != x[3])) ||
            (x[2] == x[3] && (x[1] != x[2] && x[3] != x[4])) ||
            (x[3] == x[4] && (x[2] != x[3] && x[4] != x[5])) ||
            (x[4] == x[5] && x[3] != x[4]) )
        .map(to_int)
        .count();

    println!("{:?} valid passwords", possible_passwords);
}

fn to_int(digits: Vec<usize>) -> usize {
    let count = digits.len() - 1;
    return digits.iter().rev().enumerate().fold(0, |total, (index, digit)| total + 10_usize.pow((count - index) as u32) * digit );
}

fn digits(number: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    let mut number = number;
    while number > 10 {
        digits.push(number % 10);
        number = number / 10;
    }
    digits.push(number);
    return digits;
}
