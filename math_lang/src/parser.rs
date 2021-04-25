extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedFunctionExpr<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator {
    Multiply,
    Divide,
    Exponent,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExprOperator {
    Add,
    Subtract,
    Modulo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FunctionOperator {
    Identity,
    Sin,
    Cos,
    Tan,
}

pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);

pub type ParsedFunctionExpr<'a> = (FunctionOperator, ParsedExpr<'a>);


#[derive(Debug)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str),
    InputOperation(&'a str),
    OutputOperation(ParsedFunctionExpr<'a>),
    Assignment(&'a str, ParsedFunctionExpr<'a>),
    DeclarationToAssignment(&'a str, ParsedFunctionExpr<'a>),
}

pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

pub fn parse_program(input: &str) -> IResult<&str, ParsedProgram> {
    many0(preceded(
        skip_spaces,
        alt((parse_declaration_to_assigment,
            parse_declaration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    ))(input)
}

fn parse_declaration(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((tag("var"), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::Declaration(output.2)))
}

fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((tag("in"), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.2)))
}

fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((tag("out"), skip_spaces, parse_function_expr))(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag("="),
        skip_spaces,
        parse_function_expr,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_declaration_to_assigment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        tag("var"), 
        skip_spaces,
        parse_identifier,
        skip_spaces,
        tag("="),
        skip_spaces,
        parse_function_expr,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::DeclarationToAssignment(output.2, output.6)))
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_subexpr(input: &str) -> IResult<&str, ParsedFunctionExpr> {

    delimited(
        preceded(skip_spaces, char('(')),
        parse_function_expr,
        preceded(skip_spaces, char(')')),   
    )(input)
}

fn parse_factor(input: &str) -> IResult<&str, ParsedFactor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(double, ParsedFactor::Literal),
            map(parse_subexpr, |expr| {
                ParsedFactor::SubExpression(Box::new(expr))
            }),
        )),
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, ParsedTerm> {
    tuple((
        parse_factor,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('*'), |_| TermOperator::Multiply),
                    map(char('/'), |_| TermOperator::Divide),
                    map(char('^'), |_| TermOperator::Exponent),
                )),
            ),
            parse_factor,
        ))),
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, ParsedExpr> {
   
        tuple((
            parse_term,
            many0(tuple((
                preceded(
                    skip_spaces,
                    alt((
                        map(char('+'), |_| ExprOperator::Add),
                        map(char('-'), |_| ExprOperator::Subtract),
                        map(tag("mod"), |_| ExprOperator::Modulo),
                    )),
                ),
                parse_term,
            ))),
        ))
    (input)
}

fn parse_function_expr(input: &str) -> IResult<&str, ParsedFunctionExpr> {
    tuple((
        alt((
            map(tag("sin"), |_| FunctionOperator::Sin),
            map(tag("cos"), |_| FunctionOperator::Cos),
            map(tag("tan"), |_| FunctionOperator::Tan),
            |_| {Ok((input, FunctionOperator::Identity))}, 
        )),
        parse_expr,
    ))(input)
}

fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |ch| chars.contains(ch))(input)
}
