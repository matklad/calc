use super::*;

#[test]
fn t1() {
    let input = "92";
    let expr: Expr = input.parse().unwrap();
    let value = expr.eval().unwrap();
    assert_eq!(value, 92);
}

#[test]
fn t2() {
    let input = "6 sqr";
    let expr: Expr = input.parse().unwrap();
    let value = expr.eval().unwrap();
    assert_eq!(value, 36);
}

#[test]
fn t3() {
    let input = "1 1 +";
    let expr: Expr = input.parse().unwrap();
    let value = expr.eval().unwrap();
    assert_eq!(value, 2);
}

#[test]
fn t4() {
    let expr: Expr = "3 sqr 4 sqr + 5 sqr -".parse::<Expr>().unwrap();
    let value = expr.eval().unwrap();
    assert_eq!(value, 0);
}
