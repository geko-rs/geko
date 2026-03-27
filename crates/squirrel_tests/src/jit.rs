use std::{collections::HashMap, mem, sync::Arc};

use miette::NamedSource;
use squirrel_ast::{
    atom::BinOp,
    expr::Expression,
    stmt::{Block, Statement},
};
use squirrel_jit::{CodeGenerator, Signature, Typ};
use squirrel_lex::token::Span;

#[test]
pub fn test_jit_1() {
    let source = Arc::new(NamedSource::new("-".to_string(), "-".to_string()));
    let mut generator = CodeGenerator::new().unwrap();
    let result = generator.codegen(
        Signature::new(
            "sum",
            HashMap::from([("a".to_string(), Typ::Int), ("b".to_string(), Typ::Int)]),
            Some(Typ::Int),
        ),
        &Block {
            span: Span(source.clone(), 0..0),
            statements: vec![Statement::Return {
                span: Span(source.clone(), 0..0),
                expr: Some(Expression::Bin {
                    span: Span(source.clone(), 0..0),
                    op: BinOp::Add,
                    lhs: Box::new(Expression::Variable {
                        span: Span(source.clone(), 0..0),
                        name: "a".to_string(),
                    }),
                    rhs: Box::new(Expression::Variable {
                        span: Span(source.clone(), 0..0),
                        name: "b".to_string(),
                    }),
                }),
            }],
        },
    );
    match result {
        Ok(result) => {
            let code_fn = unsafe { mem::transmute::<_, fn(i64, i64) -> i64>(result) };
            println!("res: {}", code_fn(5, 15));
        }
        Err(_) => {}
    }
    println!("{result:?}")
}

#[test]
pub fn test_jit_fibonacci() {
    use std::{collections::HashMap, mem, sync::Arc};

    use miette::NamedSource;
    use squirrel_ast::{
        atom::{BinOp, Lit},
        expr::Expression,
        stmt::{Block, Statement},
    };
    use squirrel_jit::{CodeGenerator, Signature, Typ};
    use squirrel_lex::token::Span;

    let source = Arc::new(NamedSource::new("-".to_string(), "-".to_string()));
    let mut generator = CodeGenerator::new().unwrap();

    let span = Span(source.clone(), 0..0);

    let result = generator.codegen(
        Signature::new(
            "fib",
            HashMap::from([("n".to_string(), Typ::Int)]),
            Some(Typ::Int),
        ),
        &Block {
            span: span.clone(),
            statements: vec![Statement::If {
                span: span.clone(),
                condition: Expression::Bin {
                    span: span.clone(),
                    op: BinOp::Le,
                    lhs: Box::new(Expression::Variable {
                        span: span.clone(),
                        name: "n".into(),
                    }),
                    rhs: Box::new(Expression::Lit {
                        span: span.clone(),
                        lit: Lit::Number("1".to_string()),
                    }),
                },
                then: Block {
                    span: span.clone(),
                    statements: vec![Statement::Return {
                        span: span.clone(),
                        expr: Some(Expression::Variable {
                            span: span.clone(),
                            name: "n".into(),
                        }),
                    }],
                },
                else_: Some(Box::new(Statement::Block(Box::new(Block {
                    span: span.clone(),
                    statements: vec![Statement::Return {
                        span: span.clone(),
                        expr: Some(Expression::Bin {
                            span: span.clone(),
                            op: BinOp::Add,
                            lhs: Box::new(Expression::Call {
                                span: span.clone(),
                                what: Box::new(Expression::Variable {
                                    span: span.clone(),
                                    name: "fib".into(),
                                }),
                                args: vec![Expression::Bin {
                                    span: span.clone(),
                                    op: BinOp::Sub,
                                    lhs: Box::new(Expression::Variable {
                                        span: span.clone(),
                                        name: "n".into(),
                                    }),
                                    rhs: Box::new(Expression::Lit {
                                        span: span.clone(),
                                        lit: Lit::Number("1".to_string()),
                                    }),
                                }],
                            }),
                            rhs: Box::new(Expression::Call {
                                span: span.clone(),
                                what: Box::new(Expression::Variable {
                                    span: span.clone(),
                                    name: "fib".into(),
                                }),
                                args: vec![Expression::Bin {
                                    span: span.clone(),
                                    op: BinOp::Sub,
                                    lhs: Box::new(Expression::Variable {
                                        span: span.clone(),
                                        name: "n".into(),
                                    }),
                                    rhs: Box::new(Expression::Lit {
                                        span: span.clone(),
                                        lit: Lit::Number("2".to_string()),
                                    }),
                                }],
                            }),
                        }),
                    }],
                })))),
            }],
        },
    );

    match result {
        Ok(result) => {
            let fib = unsafe { mem::transmute::<_, fn(i64) -> i64>(result) };

            println!("fib(10) = {}", fib(10));
        }
        Err(e) => panic!("{e:?}"),
    }
}
