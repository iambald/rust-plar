#![feature(box_syntax, box_patterns)]

enum Expression {
	Var(&'static str),
	Const(i32),
	Add(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>)
}

fn simplify1(expr : Expression) -> Expression {
	match expr {
		Expression::Add(box Expression::Const(0), box rhs) => rhs,
		Expression::Add(box lhs, box Expression::Const(0)) => lhs,
		Expression::Add(box Expression::Const(a), box Expression::Const(b)) => Expression::Const(a + b),
		Expression::Mul(box Expression::Const(0), _) => Expression::Const(0),
		Expression::Mul(_, box Expression::Const(0)) => Expression::Const(0),
		Expression::Mul(box Expression::Const(1), box rhs) => rhs,
		Expression::Mul(box lhs, box Expression::Const(1)) => lhs,
		Expression::Mul(box Expression::Const(a), box Expression::Const(b)) => Expression::Const(a * b),
		_ => expr
	}
}

fn simplify(expr : Expression) -> Expression {
	match expr {
		Expression::Add(lhs, rhs) => {
			simplify1(Expression::Add(Box::new(simplify(*lhs)), Box::new(simplify(*rhs))))
		},
		Expression::Mul(lhs, rhs) => {
			simplify1(Expression::Mul(Box::new(simplify(*lhs)), Box::new(simplify(*rhs))))
		},
		_ => expr
	}
}

fn expression_to_string(expr : &Expression) -> String {
	match *expr {
		Expression::Add(ref lhs, ref rhs) => format!("({} + {})", expression_to_string(&(*lhs)), expression_to_string(&(*rhs))),
		Expression::Mul(ref lhs, ref rhs) => format!("({} * {})", expression_to_string(&(*lhs)), expression_to_string(&(*rhs))),
		Expression::Const(i) => format!("{}", i),
		Expression::Var(s) => format!("{}", s)
	}
}

fn main() {
	let e = Expression::Add(
				Box::new(Expression::Mul(
					Box::new(Expression::Add(
						Box::new(Expression::Mul(
							Box::new(Expression::Const(1)), Box::new(Expression::Var("x")))),
						Box::new(Expression::Const(1)))),
					Box::new(Expression::Const(3)))),
				Box::new(Expression::Const(12)));

	println!("Simplifying {}...", expression_to_string(&e));

	match simplify(e) {
		Expression::Const(i) => println!("{}", i),
		_ => println!("Could not simplify the expression to a constant.")
	}
}
