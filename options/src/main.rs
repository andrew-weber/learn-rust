use core::num::ParseIntError;

fn sum_str_vec(strs: Vec<String>) -> Result<i32, ParseIntError> {
    let mut acc: i32 = 0;
    for s in strs {
        acc += &s.parse().unwrap()?;
    }
    Ok(acc)
}

fn main() {
    let v = vec![String::from("3"), String::from("5")];
    let total: i32 = sum_str_vec(v).unwrap();

    println!("{:?}", total);

    let v = vec![String::from("abc")];

    let total: i32 = sum_str_vec(v).unwrap_or(-1);
    println!("{:?}", total);
 
