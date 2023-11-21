use std::marker::PhantomData;

struct Fact<T> {
    t: PhantomData<T>,
}

impl<T> Fact<T> {
    fn new() -> Fact<T> {
        Fact { t: PhantomData }
    }
}

trait FunFact {
    fn fact(&self) -> String;
}

impl<T> FunFact for Fact<Vec<T>> {
    fn fact(&self) -> String {
        let facts = ["Vec is heap-allocated.", "Vec may re-allocate on growing."];
        facts[rand::random::<usize>() % facts.len()].to_string()
    }
}

impl FunFact for Fact<u8> {
    fn fact(&self) -> String {
        let facts = ["u8 has 8 bit length.", "u8 can't be less then zero."];
        facts[rand::random::<usize>() % facts.len()].to_string()
    }
}

fn main() {
    let f: Fact<Vec<u8>> = Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());

    let f: Fact<u8> = Fact::new();
    println!("Fact about u8: {}", f.fact());
    println!("Fact about u8: {}", f.fact());
    println!("Fact about u8: {}", f.fact());
}
