#![feature(step_trait)]

use std::{env, iter::Step, ops::Sub, rc::Rc};

type Val = u128;

fn f_plusone<'a, T, U>(f_n: Rc<T>) -> Rc<dyn Fn(U, U) -> U + 'a>
where
    T: Fn(U, U) -> U + ?Sized + 'a,
    U: Sub<Output = U> + Step + Copy,
{
    Rc::new(move |n: U, m: U| -> U {
        let mut acc: U = n;

        for _ in <U as Step>::forward(m - m, 1)..m {
            acc = f_n(n, acc);
        }

        acc
    })
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        return Err(format!("Usage: {} n a b", args[0]));
    }

    let n: u32 = args[1].parse().unwrap();
    let a: Val = args[2].parse().unwrap();
    let b: Val = args[3].parse().unwrap();

    let f_1 = |n: Val, m: Val| n + m;

    let mut f: Rc<dyn Fn(Val, Val) -> Val> = Rc::new(f_1);
    for _ in 1..n {
        f = f_plusone(f);
    }

    println!("{}", f(a, b));

    Ok(())
}
