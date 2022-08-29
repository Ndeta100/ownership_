fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    //get an internal reference
    let x = &data[0];
    //This code does not compile because rust does not allow mutable shared mutable reference
    data.push(7);
    print!("{}", x);
}

//(THis code does not compile)
fn as_str(data: &u32) -> &str {
    //compute the string
    let s = format!("{}", data);
    // s is already owned
    &s
}

#[cfg(test)]
mod test {
    use super::*;
}
