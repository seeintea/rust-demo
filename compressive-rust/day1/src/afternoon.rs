// subject 1

#[allow(unused)]
pub fn luhn(cc_number: &str) -> bool {
    let mut digits_seen = 0;
    let mut sum = 0;

    for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(d) => {
                sum += if i % 2 == 1 {
                    let dd = d * 2;
                    dd / 10 + dd % 10
                } else {
                    d
                };
                digits_seen += 1;
            }
            None => return false,
        }
    }

    if digits_seen < 2 {
        return false;
    }

    sum % 10 == 0
}

#[test]
pub fn subject_1() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));

    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));

    assert!(!luhn("0"));

    assert!(luhn(" 0 0 "));

    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));

    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

// subject 2

#[allow(unused)]
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[allow(unused)]
#[derive(Debug)]
enum Expression {
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
enum Res {
    Ok(i64),
    Err(String),
}

#[allow(unused)]
fn eval(e: Expression) -> Res {
    match e {
        Expression::Value(v) => Res::Ok(v),
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Res::Ok(v) => v,
                Res::Err(msg) => return Res::Err(msg),
            };
            let right = match eval(*right) {
                Res::Ok(v) => v,
                Res::Err(msg) => return Res::Err(msg),
            };

            Res::Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Res::Err(String::from("division by zero"));
                    } else {
                        left / right
                    }
                }
            })
        }
    }
}

#[test]
pub fn subject_2() {
    assert_eq!(eval(Expression::Value(19)), Res::Ok(19));

    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Res::Ok(30)
    );

    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };

    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Res::Ok(85)
    );

    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Res::Err(String::from("division by zero"))
    );
}
