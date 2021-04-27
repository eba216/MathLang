mod analyzer;
mod compiler;
mod executor;
mod parser;
mod symbol_table;

use crate::symbol_table::SymbolTable;

const MATH_SUFFIX: &str = ".math";
const PI: f64 = 3.141592653589793115997963468544185161590576171875;
const EXP: f64 = 2.718281828459045090795598298427648842334747314453125;

fn main() {

    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let option = args.next();

    match option.as_deref() {
        Some("--repl") => run_interpreter(),
        Some("--compile") => {
            let source_path = args.next();
            
            match source_path.as_deref() {
                Some(file) => { 
                    if file.ends_with(MATH_SUFFIX) {
                        process_file(&current_program_path, &source_path.unwrap());    
                    } else {
                        eprintln!(
                            "{}: Invalid argument '{}': It must end with {}",
                            current_program_path, file, MATH_SUFFIX
                        );
                        return;
                    }
                }  
                None => {
                    eprintln!("Additional argumeng needed: <file.math or string>");
                    return;
                }
            }        
        }
        Some("--interpret") => {
            let program = args.next();
            match program.as_deref() {
                Some(program) => { interpret(program)},
                None => {
                    eprintln!("Additional argumeng needed: <file.math or string>");
                    return;
                }

            }
        }
        Some(_) => {
            eprintln!("Command not recognized. Supported commands are --compile <file.math>, --intepret <string> , and --repl");
            return;
        }
        None => {
            eprintln!("Additional Command needed. Options are --compile <file.math>, --intepret <string> , and --repl");
            return;
        }
    } 

}

fn process_file(current_program_path: &str, source_path: &str) {
    
    if !source_path.ends_with(MATH_SUFFIX) {
        eprintln!(
            "{}: Invalid argument '{}': It must end with {}",
            current_program_path, source_path, MATH_SUFFIX
        );
        return;
    }
    let target_path = source_path[0..source_path.len() - MATH_SUFFIX.len()].to_string() + ".rs";
    let source_code = std::fs::read_to_string(&source_path);
    if source_code.is_err() {
        eprintln!(
            "Failed to read from file {}: ({})",
            source_path,
            source_code.unwrap_err()
        );
        return;
    }
    let source_code = source_code.unwrap();

    let parsed_program;
    match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => { 
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path, trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    }

    let analyzed_program;
    let mut variables = symbol_table::SymbolTable::new();
    match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => {
            analyzed_program = analyzed_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path, err);
            return;
        }
    } 

    match std::fs::write(
        &target_path,
        compiler::translate_to_rust_program(&variables, &analyzed_program),
    ) {
        Ok(_) => eprintln!("Compiled {} to {}.", source_path, target_path),
        Err(err) => eprintln!("Failed to write to file {}: ({})", target_path, err),
    }
}

fn interpret(program:&str) {
    eprintln!("\n* Interpreting *\n");
    let mut variables = symbol_table::SymbolTable::new();
    initialize_math_constants(&mut variables);

    loop  {
        let pattern = parser::parse_program(&program);
        match pattern {
            Ok((rest, parsed_program)) => { 
                if rest.len() > 0 {
                    //eprintln!("Symbol table: {:#?}", &mut variables); 
                    eprintln!("Unparsed input: `{}`.", rest)
                } else {
                    match analyzer::analyze_program(&mut variables, &parsed_program) {
                        Ok(analyzed_program) => {
                            eprintln!("Symbol table: {:#?}", &mut variables); 
                            eprint!("\n");
                            eprintln!("Analyzed program: {:#?}", &analyzed_program);
                            executor::execute_program(&mut variables, &analyzed_program);   
                            break;
                        }
                        Err(err) => {eprintln!("Error: {}", err); break;},
                    }
                }
            }
            Err(err) =>  {eprintln!("Error: {:?}", err); break;},
        } 
        break; 
    }
}

fn run_interpreter() {
    eprintln!("\n* Math Interactive Interpreter *\n");
    let mut variables = symbol_table::SymbolTable::new();
    initialize_math_constants(&mut variables);
    
    loop {
        let command = input_command();
        if command.len() == 0 {
            break;
        }
        match command.trim() {
            "quit" => {eprintln!("Goodbye"); break},
            "clear" => {
                variables = symbol_table::SymbolTable::new();
                initialize_math_constants(&mut variables);
                eprintln!("Cleared variables.");
            }
            "variables" => {
                eprintln!("Variables:");
                for v in variables.iter() {
                    eprintln!("  {}: {}", v.0, v.1);
                }
                
            }
            trimmed_command => match parser::parse_program(&trimmed_command) {
                Ok((rest, parsed_program)) => {
                    if rest.len() > 0 {
                        //eprintln!("Symbol table: {:#?}", &mut variables); 
                        eprintln!("Unparsed input: `{}`.", rest)
                    } else {
                        match analyzer::analyze_program(&mut variables, &parsed_program) {
                            Ok(analyzed_program) => {
                                //println!("Analyzed program: {:#?}", &analyzed_program);
                                executor::execute_program(&mut variables, &analyzed_program)        
                                
                            }
                            Err(err) => {eprintln!("Error: {}", err)},
                        }
                    }
                }
                Err(err) =>  eprintln!("Error: {:?}", err),
            },
        }

    }
    
}

fn input_command() -> String {
    let mut text = String::new();
    eprint!("<shell>: ");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    text
}

fn initialize_math_constants(variables: &mut SymbolTable) {
    let pi = String::from("pi");
    let e = String::from("e"); 
    
    if let Ok(_) = variables.insert_symbol(&pi){}
    if let Ok(_) = variables.insert_symbol(&e){}
    
    if let Ok(v) = variables.find_symbol(&pi) {
        variables.set_value(v, PI);
    }
    if let Ok(v) = variables.find_symbol(&e) {
        variables.set_value(v,EXP);
    }

}