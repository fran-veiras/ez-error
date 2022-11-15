use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use terminal_menu::{menu, button, run, mut_menu};

fn main() {   
    // 
    let mut es_lint_problems = vec![];
    let mut index = 0;

    if let Ok(lines) = read_lines("ez-error-logs.txt") {
            // Consumes the iterator, returns an Eslint problems with index numb
            for line in lines {
                if let Ok(eslint_line) = line {
                    // if is a problem, i index it
                    if eslint_line.contains("warning") && !eslint_line.contains("problems") {
                        es_lint_problems.push(eslint_line + &format!(" \x1b[32m[{index}] ez-err\x1b[0m") + "\n");
                        index = index + 1;
                    } else {
                        // else returns eslint line with line break
                        es_lint_problems.push(eslint_line + "\n");
                    }
                }
            }
    }

    // turn into string 
    //let all_problems = String::from_iter(es_lint_problems);

    //println!("{}", all_problems);

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        Ok(io::BufReader::new(file).lines())
    }

    // hacer input de la linea seleccionada en consola.
    //
    // abrir -> ventana con problema
    

    let options = es_lint_problems.iter();

    let mut btn = vec![];

    for option in options {
        if option.contains("warning") && !option.contains("problems") {btn.push(button(&format!("{option}"))) }
    }

    let my_menu = menu(btn);

    run(&my_menu);

    println!("Selected Button: {}", mut_menu(&my_menu).selected_item_name());
}
