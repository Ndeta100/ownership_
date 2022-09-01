use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    //get an internal reference
    let x = &data[0];
    //This code does not compile because rust does not allow inmutable shared mutable reference
    data.push(7);
    print!("{}", x);

    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share();
    println!("{:?}", loan);
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}

//(THis code does not compile)
fn as_str(data: &str) -> &str {
    //compute the string
    let s = format!("{}", data);
    // s is already owned
    &s
}

fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
    //Remember that `output` will be `2` if `input >10;`
}

fn compute_optimize(input: &u32, output: &mut u32) {
    let cached_input = *input;
    if cached_input > 10 {
        //If the input is greater than 10, the previous code would set the output to 1 and then double it, resulting in an output of 2 (because `>10 implies `>5`) Here, we avoid the double assignment and just set it directly to 2
        *output *= 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}
fn compute_optimize_output(input: &u32, output: &mut u32) {
    let mut temp = *output;
    if *input > 10 {
        temp = 1;
    }
    if *input > 5 {
        temp *= 2;
    }
    *output = temp;
}

fn l() {
    let x = 0;
    let z;
    let y = &x;
    z = y;
}
//he right way to do it
fn to_string(data: &u32) -> String {
    format!("{}", data)
}
//Okay lifetimes
fn kk() {
    let mut data = vec![1, 2, 3, 4];
    let x = &data[1];
    println!("{}", x);
    data.push(7874);
}
#[derive(Debug)]
struct X<'a>(&'a i32);
impl Drop for X<'_> {
    fn drop(&mut self) {}
}

//Limit lifetime
#[derive(Debug)]
struct Foo;
impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*&Self
    }
    fn share(&self) {
        //
    }
}
//Improperly reduced borrows
fn get_default<'a, K, V>(map: &'a mut HashMap<K, V>, key: K) -> &'a mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }
    }
}
//Higher-Rank Trait Bounds (HRTBs)
struct Closure<T> {
    data: (u8, u16),
    func: T,
}
impl<T> Closure<T>
where
    T: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}
fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}
#[cfg(test)]
mod test {
    use super::*;
}
