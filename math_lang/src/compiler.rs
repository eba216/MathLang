use crate::analyzer::{
    AnalyzedFunctionExpr, AnalyzedExpr, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm,
};
use crate::parser::{ExprOperator, TermOperator, FunctionOperator};
use crate::symbol_table::SymbolTable;

fn translate_to_rust_factor(variables: &SymbolTable, analyzed_factor: &AnalyzedFactor) -> String {
    match analyzed_factor {
        AnalyzedFactor::FunctionExpression(expr) => {
            "(".to_string() + &translate_to_rust_function_expr(variables, expr)
        }
        AnalyzedFactor::Literal(value) => value.to_string() + "f64",
        AnalyzedFactor::Identifier(handle) => "".to_string() + &variables.get_name(*handle),
        AnalyzedFactor::SubExpression(expr) => {
            "(".to_string() + &translate_to_rust_expr(variables, expr) + ")"
        }
        
    }
}

fn translate_to_rust_term(variables: &SymbolTable, analyzed_term: &AnalyzedTerm) -> String {
    let mut result = translate_to_rust_factor(variables, &analyzed_term.0);
    for factor in &analyzed_term.1 {
        match factor.0 {
            TermOperator::Multiply => {
                result += " * ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
            TermOperator::Divide => {
                result += " / ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
            TermOperator::Exponent => {
                result += ".pow";
                result += &translate_to_rust_factor(variables, &factor.1);
                result += ""

            }
        }
    }
    result
}

fn translate_to_rust_expr(variables: &SymbolTable, analyzed_expr: &AnalyzedExpr) -> String {
    let mut result = translate_to_rust_term(variables, &analyzed_expr.0);
    for term in &analyzed_expr.1 {
        match term.0 {
            ExprOperator::Add => {
                result += " + ";
                result += &translate_to_rust_term(variables, &term.1);
            }
            ExprOperator::Subtract => {
                result += " - ";
                result += &translate_to_rust_term(variables, &term.1);
            }
            ExprOperator::Modulo => {
                result += " % ";
                result += &translate_to_rust_term(variables, &term.1);
            }
        }
    }
    result
}

fn translate_to_rust_function_expr(variables: &SymbolTable, analyzed_expr: &AnalyzedFunctionExpr) -> String {
    let mut result = translate_to_rust_expr(variables, &analyzed_expr.1);
    match &analyzed_expr.0 {
        FunctionOperator::Sin => {result += ").sin()";}
        FunctionOperator::Cos => {result += ").cos()";}
        FunctionOperator::Tan => {result += ").tan()";}
        FunctionOperator::ArcSin => {result += ").asin()";}
        FunctionOperator::ArcCos => {result += ").acos()";}
        FunctionOperator::ArcTan => {result += ").atan()";}
        FunctionOperator::Sinh => {result += ").sinh()";}
        FunctionOperator::Cosh => {result += ").cosh()";}
        FunctionOperator::Tanh => {result += ").tanh()";}
        FunctionOperator::ArcSinh => {result += ").asinh()";}
        FunctionOperator::ArcCosh => {result += ").acosh()";}
        FunctionOperator::ArcTanh => {result += ").atanh()";}
        FunctionOperator::Exp => {result += ").exp()";}
        FunctionOperator::Exp2 => {result += ").exp2()";}
        FunctionOperator::Log => {result += ").ln()";}
        FunctionOperator::Log10 => {result += ").log10()";}
        FunctionOperator::Log2 => {result += ").log2()";}
        FunctionOperator::Abs => {result += ").abs()";}
        FunctionOperator::Ceil => {result += ").ceil()";}
        FunctionOperator::Floor => {result += ").floor()";}
        FunctionOperator::Signum => {result += ").signum()";}
        FunctionOperator::Sqrt => {result += ").sqrt()";}
    }
    result
}

fn translate_to_rust_statement(
    variables: &SymbolTable,
    analyzed_statement: &AnalyzedStatement,
) -> String {
    match analyzed_statement {
        
        AnalyzedStatement::DeclarationToAssignment(handle, expr) => {
            format!("let mut {} = {}", 
            variables.get_name(*handle), 
            translate_to_rust_expr(&variables, expr))
        }
        AnalyzedStatement::Assignment(handle, expr) => format!(
            "{} = {}",
            variables.get_name(*handle),
            translate_to_rust_expr(&variables, expr)
        ),
        AnalyzedStatement::Declaration(handle) => {
            format!("let mut {} = 0.0", variables.get_name(*handle))
        }
        AnalyzedStatement::InputOperation(handle) => {
            format!("{} = input()", variables.get_name(*handle))
        }
        AnalyzedStatement::OutputOperation(expr) => format!(
            "println!(\"<output>: {}\", {})",
            "{}",
            translate_to_rust_expr(&variables, expr)
        ),
    }
}

pub fn translate_to_rust_program(
    variables: &SymbolTable,
    analyzed_program: &AnalyzedProgram,
) -> String {
    let mut rust_program = String::new();
    rust_program += "use std::io::Write;\n";
    rust_program += "\n";
    rust_program += "#[allow(dead_code)]\n";
    rust_program += "fn input() -> f64 {\n";
    rust_program += "    let mut text = String::new();\n";
    rust_program += "    eprint!(\"<input>: \");\n";
    rust_program += "    std::io::stderr().flush().unwrap();\n";
    rust_program += "    std::io::stdin()\n";
    rust_program += "        .read_line(&mut text)\n";
    rust_program += "        .expect(\"Cannot read line.\");\n";
    rust_program += "    text.trim().parse::<f64>().unwrap_or(0.)\n";
    rust_program += "}\n";
    rust_program += "\n";
    rust_program += "fn main() {\n"; 
    for statement in analyzed_program {
        rust_program += "    ";
        rust_program += &translate_to_rust_statement(&variables, statement);
        rust_program += ";\n";
    }
    rust_program += "}\n";
    rust_program
}