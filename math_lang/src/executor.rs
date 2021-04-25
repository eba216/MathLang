use crate::analyzer::{
    AnalyzedFunctionExpr, AnalyzedExpr, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm,
};
use crate::parser::{ExprOperator, TermOperator, FunctionOperator};
use crate::symbol_table::SymbolTable;


fn evaluate_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> f64 {
    match factor {
        AnalyzedFactor::Literal(value) => *value,
        AnalyzedFactor::Identifier(handle) => variables.get_value(*handle),
        AnalyzedFactor::SubExpression(expr) => evaluate_function_expr(variables, expr),
    }
}

fn evaluate_term(variables: &SymbolTable, term: &AnalyzedTerm) -> f64 {
    let mut result = evaluate_factor(variables, &term.0);
    for factor in &term.1 {
        match factor.0 {
            TermOperator::Multiply => result *= evaluate_factor(variables, &factor.1),
            TermOperator::Divide => result /= evaluate_factor(variables, &factor.1),
            TermOperator::Exponent => result = result.powf(evaluate_factor(variables, &factor.1)),
        }
    }
    result
}

fn evaluate_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> f64 {
    let mut result = evaluate_term(variables, &expr.0);
    for term in &expr.1 {
        match term.0 {
            ExprOperator::Add => result += evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result -= evaluate_term(variables, &term.1),
            ExprOperator::Modulo => result %= evaluate_term(variables, &term.1),
        }
    }
    result
}

fn evaluate_function_expr(variables: &SymbolTable, function_expr: &AnalyzedFunctionExpr) -> f64 {
    let mut result = evaluate_expr(variables, &function_expr.1);
    match function_expr.0 {
            FunctionOperator::Identity => {}, 
            FunctionOperator::Sin => {result = result.sin()},
            FunctionOperator::Cos => {result = result.cos()},
            FunctionOperator::Tan => {result = result.tan()},
    }
    result
}

fn execute_statement(variables: &mut SymbolTable, statement: &AnalyzedStatement) {
    match statement {
        AnalyzedStatement::Assignment(handle, expr) => {
            variables.set_value(*handle, evaluate_function_expr(variables, expr));
        }
        AnalyzedStatement::DeclarationToAssignment(handle, expr) => {
            variables.set_value(*handle, evaluate_function_expr(variables, expr));
        }
        AnalyzedStatement::Declaration(_) => {}
        AnalyzedStatement::InputOperation(handle) => {
            let mut text = String::new();
            eprint!("<input>: ");
            std::io::stdin()
                .read_line(&mut text)
                .expect("Cannot read line.");
            let value = text.trim().parse::<f64>().unwrap_or(0.);
            variables.set_value(*handle, value);
        }
        AnalyzedStatement::OutputOperation(expr) => {
            println!("<output>: {}", evaluate_function_expr(variables, expr));
        }
    }
}

pub fn execute_program(variables: &mut SymbolTable, program: &AnalyzedProgram) {
    for statement in program {
        execute_statement(variables, statement);
    }
}