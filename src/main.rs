use clap::Parser;

#[derive(Parser)]
struct Arguments {
    a: i32,
    b: i32,
}

struct Adder;

impl Adder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b + a
    }
}

#[cfg(test)]
mod tests {
    use crate::Adder;

    #[test]
    fn test_add_two_numbers() {
        let adder = Adder::new();

        let res = adder.add(3, 4);
        assert_eq!(7, res);
    }
}

fn main() {
    let args = Arguments::parse();
    let adder = Adder::new();
    let res = adder.add(args.a, args.b);
    println!("{} + {} = {res}", args.a, args.b);
}
