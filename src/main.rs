#[derive(Clone)]
enum UnaryOperator {
    Minus,
}

#[derive(Clone)]
enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
struct UnaryExpression {
    operator: UnaryOperator,
    operand: Box<Expression>,
}

#[derive(Clone)]
struct BinaryExpression {
    operator: BinaryOperator,
    left_operand: Box<Expression>,
    right_operand: Box<Expression>,
}

#[derive(Clone)]
enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Constant(f32),
    Variable,
}

fn derivative_binary(input: BinaryExpression) -> Expression {
    match input.operator {
        BinaryOperator::Addition => Expression::Binary(BinaryExpression {
            operator: BinaryOperator::Addition,
            left_operand: Box::new(derivative(*input.left_operand)),
            right_operand: Box::new(derivative(*input.right_operand)),
        }),
        BinaryOperator::Subtraction => Expression::Binary(BinaryExpression {
            operator: BinaryOperator::Subtraction,
            left_operand: Box::new(derivative(*input.left_operand)),
            right_operand: Box::new(derivative(*input.right_operand)),
        }),
        BinaryOperator::Multiplication => Expression::Binary(BinaryExpression {
            operator: BinaryOperator::Addition,
            left_operand: Box::new(Expression::Binary(BinaryExpression {
                operator: BinaryOperator::Multiplication,
                left_operand: Box::new(derivative(*input.left_operand.clone())),
                right_operand: Box::new(*input.right_operand.clone()),
            })),
            right_operand: Box::new(Expression::Binary(BinaryExpression {
                operator: BinaryOperator::Multiplication,
                left_operand: Box::new(*input.left_operand),
                right_operand: Box::new(derivative(*input.right_operand)),
            })),
        }),
        BinaryOperator::Division => Expression::Binary(BinaryExpression {
            operator: BinaryOperator::Division,
            left_operand: Box::new(Expression::Binary(BinaryExpression {
                operator: BinaryOperator::Subtraction,
                left_operand: Box::new(Expression::Binary(BinaryExpression {
                    operator: BinaryOperator::Multiplication,
                    left_operand: Box::new(derivative(*input.left_operand.clone())),
                    right_operand: Box::new(*input.right_operand.clone()),
                })),
                right_operand: Box::new(Expression::Binary(BinaryExpression {
                    operator: BinaryOperator::Multiplication,
                    left_operand: Box::new(*input.left_operand),
                    right_operand: Box::new(derivative(*input.right_operand.clone())),
                })),
            })),
            right_operand: Box::new(Expression::Binary(BinaryExpression {
                operator: BinaryOperator::Multiplication,
                left_operand: Box::new(*input.right_operand.clone()),
                right_operand: Box::new(*input.right_operand),
            })),
        }),
    }
}

fn derivative_unary(input: UnaryExpression) -> Expression {
    match input.operator {
        UnaryOperator::Minus => Expression::Unary(UnaryExpression {
            operator: UnaryOperator::Minus,
            operand: Box::new(derivative(*input.operand)),
        }),
    }
}

fn derivative(input: Expression) -> Expression {
    match input {
        Expression::Unary(unary_expr) => derivative_unary(unary_expr),
        Expression::Binary(bin_expr) => derivative_binary(bin_expr),
        Expression::Constant(_) => Expression::Constant(0.0),
        Expression::Variable => Expression::Constant(1.0),
    }
}

fn simplify(input: Expression) -> Expression {
    match input {
        Expression::Binary(bin_expr) => match bin_expr.operator {
            BinaryOperator::Multiplication => {
                match (*bin_expr.left_operand, *bin_expr.right_operand) {
                    (Expression::Constant(c), right_operand) if c == 1.0 => right_operand,
                    (left_operand, Expression::Constant(c)) if c == 1.0 => left_operand,
                    (left_operand, right_operand) => Expression::Binary(BinaryExpression {
                        operator: BinaryOperator::Multiplication,
                        left_operand: Box::new(simplify(left_operand)),
                        right_operand: Box::new(simplify(right_operand)),
                    }),
                }
            }
            BinaryOperator::Addition => match (*bin_expr.left_operand, *bin_expr.right_operand) {
                (Expression::Variable, Expression::Variable) => {
                    Expression::Binary(BinaryExpression {
                        operator: BinaryOperator::Multiplication,
                        left_operand: Box::new(Expression::Constant(2.0)),
                        right_operand: Box::new(Expression::Variable),
                    })
                }
                (left_operand, right_operand) => Expression::Binary(BinaryExpression {
                    operator: BinaryOperator::Addition,
                    left_operand: Box::new(simplify(left_operand)),
                    right_operand: Box::new(simplify(right_operand)),
                }),
            },
            _ => Expression::Binary(BinaryExpression {
                operator: bin_expr.operator,
                left_operand: Box::new(simplify(*bin_expr.left_operand)),
                right_operand: Box::new(simplify(*bin_expr.right_operand)),
            }),
        },
        Expression::Unary(unary_expr) => Expression::Unary(UnaryExpression {
            operator: unary_expr.operator,
            operand: Box::new(simplify(*unary_expr.operand)),
        }),
        Expression::Constant(c) => Expression::Constant(c),
        Expression::Variable => Expression::Variable,
    }
}

fn print_expression(expression: Expression) -> String {
    match expression {
        Expression::Unary(unary_expr) => match unary_expr.operator {
            UnaryOperator::Minus => "(-".to_string() + &print_expression(*unary_expr.operand) + ")",
        },
        Expression::Binary(bin_expr) => match bin_expr.operator {
            BinaryOperator::Addition => {
                "(".to_string()
                    + &print_expression(*bin_expr.left_operand)
                    + "+"
                    + &print_expression(*bin_expr.right_operand)
                    + ")"
            }
            BinaryOperator::Subtraction => {
                "(".to_string()
                    + &print_expression(*bin_expr.left_operand)
                    + "-"
                    + &print_expression(*bin_expr.right_operand)
                    + ")"
            }
            BinaryOperator::Multiplication => {
                "(".to_string()
                    + &print_expression(*bin_expr.left_operand)
                    + "*"
                    + &print_expression(*bin_expr.right_operand)
                    + ")"
            }
            BinaryOperator::Division => {
                "(".to_string()
                    + &print_expression(*bin_expr.left_operand)
                    + "/"
                    + &print_expression(*bin_expr.right_operand)
                    + ")"
            }
        },
        Expression::Constant(c) => c.to_string(),
        Expression::Variable => "x".to_string(),
    }
}

fn main() {
    let expression: Expression = Expression::Binary(BinaryExpression {
        operator: BinaryOperator::Multiplication,
        left_operand: Box::new(Expression::Variable),
        right_operand: Box::new(Expression::Variable),
    });

    println!(
        "{}",
        print_expression(simplify(simplify(derivative(expression))))
    );
}
