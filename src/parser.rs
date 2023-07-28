use anyhow::{bail, Result};
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

use crate::error::ParserError;
use crate::token::{Expr, Op};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/numeric_evaluator.pest"]
pub(crate) struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(power, Right))
            .op(Op::prefix(unary_minus))
        };
}

fn parse_function(pairs: Pairs<Rule>) -> Result<Expr> {
    let mut name = String::new();
    let mut args: Vec<Box<Expr>> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::function_name => name = String::from(pair.as_str()),
            Rule::function_args => {
                args = pair
                    .into_inner()
                    .map(|arg| parse_expr(arg.into_inner()))
                    .map(|arg| Box::new(arg.unwrap()))
                    .collect()
            }
            rule => {
                bail!(ParserError::InvalidToken(format!("{:?}", rule)))
            }
        }
    }

    if name != "" {
        Ok(Expr::Function { name, args })
    } else {
        bail!(ParserError::NoFunctionName)
    }
}

fn remove_non_numeric_and_parse(content: &str) -> i32 {
    let content: String = content.chars().filter(|c| c.is_numeric()).collect();
    content.parse::<i32>().unwrap()
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(Expr::Number(primary.as_str().parse::<f64>().unwrap())),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::function => parse_function(primary.into_inner()),
            Rule::literal => Ok(Expr::Literal(remove_non_numeric_and_parse(
                primary.as_str(),
            ))),
            Rule::symbol => Ok(Expr::Symbol(remove_non_numeric_and_parse(
                primary.as_str(),
            ))),
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .map_infix(|lhs, op, rhs| {
            let op: Result<Op> = match op.as_rule() {
                Rule::add => Ok(Op::Add),
                Rule::subtract => Ok(Op::Subtract),
                Rule::multiply => Ok(Op::Multiply),
                Rule::divide => Ok(Op::Divide),
                Rule::modulo => Ok(Op::Modulo),
                Rule::power => Ok(Op::Power),
                rule => bail!(ParserError::InvalidOperator(format!("{:?}", rule))),
            };
            let op = op?;
            Ok(Expr::BinOp {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Ok(Expr::UnaryMinus(Box::new(rhs?))),
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .parse(pairs)
}

pub fn parse(expression: &str) -> Result<Expr> {
    let mut pairs = CalculatorParser::parse(Rule::equation, expression)?;
    parse_expr(pairs.next().unwrap().into_inner())
}
