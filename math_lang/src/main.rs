mod analyzer;
mod compiler;
mod executor;
mod parser;
mod symbol_table;

const MATH_SUFFIX: &str = ".math";

fn main() {

    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let option = args.next();

    match option.as_deref() {
        Some("repl") => run_interpreter(),
        Some("compile") => {
            let source_path = args.next();
            
            match source_path.as_deref() {
                Some(file) if file.ends_with(MATH_SUFFIX) => {
                    process_file(&current_program_path, &source_path.unwrap()); 
                }
                Some(_) => interpret(),
                None => {
                    eprintln!("Additional argumeng needed: <file.math or string>");
                    return
                }
            }        
        } 
        Some(_) => {
            eprintln!("Command not recognized. Supported commands are -compile <file.math or string> and -repl");
            return
        }
        None => {
            eprintln!("Additional Command needed. Options are -compile <file.math or string> and -repl");
            return
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

fn interpret() {

}

fn run_interpreter() {
    eprintln!("* Math Interactive Interpreter *");
    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let command = input_command();
        if command.len() == 0 {
            break;
        }
        match command.trim() {
            "quit" => break,
            "clear" => {
                variables = symbol_table::SymbolTable::new();
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
                        eprintln!("Symbol table: {:#?}", &mut variables); 
                        eprintln!("Unparsed input: `{}`.", rest)
                    } else {
                        match analyzer::analyze_program(&mut variables, &parsed_program) {
                            Ok(analyzed_program) => {
                                println!("Analyzed program: {:#?}", &analyzed_program);
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