use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use terminal_menu::{menu, button, run, TerminalMenuItem, mut_menu};
use regex::Regex;
mod utils;

fn main() {   
    // 
    let mut es_lint_problems = vec![];

    if let Ok(lines) = read_lines("ez-error-logs.txt") {
            // Consumes the iterator, returns an Eslint problems with index numb
            for line in lines {
                if let Ok(eslint_line) = line {
                    // if is a problem, i index it
                    if eslint_line.contains("warning") && !eslint_line.contains("problems") {
                        es_lint_problems.push(eslint_line + &format!(" \x1b[32mez-err\x1b[0m") + "\n");
                    } else {
                        // else returns eslint line with line break
                        es_lint_problems.push(eslint_line + "\n");
                    }
                }
            }
    }

    // turn into string 

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        Ok(io::BufReader::new(file).lines())
    }

    // hacer input de la linea seleccionada en consola.
    //
    // abrir -> ventana con problema
    

    fn test(options: std::slice::Iter<'_, String>) -> Result<Vec<TerminalMenuItem>, i32> {
    
        let mut btn = vec![];
        
        for option in options {
            let format_to_select = option.replace("\n", "");
            if option.contains("warning") && !option.contains("problem") {btn.push(button (format!("{format_to_select}"))) } else {}
        }

        Ok(btn) 
    }

    let options = es_lint_problems.iter();
    
    let version = test(options);

   match version {
        Ok(btn) => {
            let options = menu(btn);
             
            run(&options);

            println!("Selected button {}", mut_menu(&options).selected_item_name());

            let select_type_of_search = menu(vec![button("stackoverflow"), button("browser")]);

            run(&select_type_of_search);

            // let type_of_search = mut_menu(&select_type_of_search).selected_item_name();

            println!("{}", mut_menu(&select_type_of_search).selected_item_name());
            
            let re = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();

            let mut result = re.replace_all(mut_menu(&options).selected_item_name(), "").replace("ez-err", "").to_string();

            // result.find("warning")
            // por ahora solo con warning funciona 
            //

            utils::open_browser::open(result.drain(result.find("warning").unwrap()..result.len()).as_str());
        },
        Err(e) => println!("error parsing header: {e:?}"),
    };

    // println!("Selected Button: {}", mut_menu(&my_menu).selected_item_name());
}






