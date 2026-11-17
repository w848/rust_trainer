enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

// Small helpers so building trees is less noisy.
fn num(n: i64) -> Expr { Expr::Num(n) }
fn add(a: Expr, b: Expr) -> Expr { Expr::Add(Box::new(a), Box::new(b)) }
fn mul(a: Expr, b: Expr) -> Expr { Expr::Mul(Box::new(a), Box::new(b)) }
fn neg(a: Expr) -> Expr { Expr::Neg(Box::new(a)) }

fn eval(e: &Expr) -> i64 {
    todo!()
}

fn to_text(e: &Expr) -> String {
    todo!()
}

fn main() {
    let e = mul(add(num(1), num(2)), neg(num(3)));
    println!("{} = {}", to_text(&e), eval(&e));
}
