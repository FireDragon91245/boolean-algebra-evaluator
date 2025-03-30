use crate::evaluator::EvaluatorPassResult;
use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::io;
use std::io::Write;
use tabled::builder::Builder;
use tabled::settings::Style;

mod ast;
mod bin_tree;
mod evaluator;
mod tokenizer;
mod tree_print;

#[derive(Parser, Debug)]
#[clap(
    author = "FireDragon91245",
    version = "1.0",
    about = "Evaluates Boolean Algebra expressions",
    long_about = "Evaluates Boolean Algebra expressions\
    \n\
    \nSyntax:\
    \n  AND: &\
    \n  OR: |\
    \n  XOR: ^\
    \n  NOT: !\
    \n  EQUAL: =\
    \n  TRUE: 1 or true\
    \n  FALSE: 0 or false\
    \n  IDENTIFIERS: a-z"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(PartialEq)]
enum AstPrintMode {
    Default,
    Pretty,
    Extended,
    PrettyExtended,
}

impl AstPrintMode {
    fn from(pretty: bool, extended: bool) -> Self {
        match (pretty, extended) {
            (true, true) => AstPrintMode::PrettyExtended,
            (true, false) => AstPrintMode::Pretty,
            (false, true) => AstPrintMode::Extended,
            (false, false) => AstPrintMode::Default,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        name = "-eval",
        about = "evaluates the given boolean expression, identifiers are not supported",
        short_flag = 'e'
    )]
    Eval { expression: String },
    #[command(
        name = "-Table",
        about = "prints the truth table for the given boolean expression, identifiers are supported",
        short_flag = 'T'
    )]
    Table {
        expression: String,
        #[arg(
            required = false,
            default_value = "false",
            long = "true",
            short = 't',
            help = "filter rows where the result is true"
        )]
        filter_true: bool,
        #[arg(
            required = false,
            default_value = "false",
            long = "false",
            short = 'f',
            help = "filter rows where the result is false"
        )]
        filter_false: bool,
    },
    #[command(
        name = "-truth",
        about = "evaluates the given boolean expression with the given inputs, identifiers are supported",
        short_flag = 't'
    )]
    Truth {
        #[arg(name = "identifier_values", required = true, num_args = 1..)]
        inputs: Vec<String>,
        expression: String,
    },
    #[command(
        name = "-ast",
        about = "Prints the AST of the given boolean expression",
        short_flag = 'a'
    )]
    Ast {
        expression: String,
        #[arg(
            required = false,
            default_value = "false",
            long = "pretty",
            short = 'p',
            help = "enable pretty printing"
        )]
        pretty: bool,
        #[arg(
            required = false,
            default_value = "false",
            long = "extended",
            short = 'e',
            help = "enable extended printing"
        )]
        extended: bool,
    },
}

fn show_prompt(prompt: &str, options: &Vec<&str>) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if options.contains(&&*input) {
            return input;
        } else {
            print!("\n{}", prompt);
            let _ = io::stdout().flush();
        }
    }
}

fn evaluate_bool_exp(expression: &String) -> Result<bool, String> {
    let tokens = tokenizer::tokenize(expression, false)?;
    let mut parser = ast::Parser::new(tokens, expression);
    let ast = parser.parse()?;
    let evaluator = evaluator::Evaluator::new(ast);

    Ok(evaluator.evaluate(0))
}

fn evaluate_truth_table(expression: &String) -> Result<Vec<EvaluatorPassResult>, String> {
    let tokens = tokenizer::tokenize(expression, true)?;
    let mut parser = ast::Parser::new(tokens, expression);
    let ast = parser.parse()?;
    let evaluator = evaluator::Evaluator::new(ast);
    let ident_count = evaluator.get_identifiers().count();
    if ident_count >= 18 {
        match show_prompt(
            format!(
                "Performance Warning: Your about to calculate {} results! Continue? [y|n]:",
                1 << ident_count
            )
            .as_str(),
            &vec!["y", "n"],
        )
        .as_str()
        {
            "n" => return Err("Aborted".to_string()),
            _ => {}
        }
    }
    let iter = evaluator.evaluate_iter().collect::<Vec<_>>();
    Ok(iter)
}

fn evaluate_pass(expression: &String, pass: usize) -> Result<EvaluatorPassResult, String> {
    let tokens = tokenizer::tokenize(expression, true)?;
    let mut parser = ast::Parser::new(tokens, expression);
    let ast = parser.parse()?;
    let evaluator = evaluator::Evaluator::new(ast);

    Ok(EvaluatorPassResult {
        result: evaluator.evaluate(pass),
        ident_states: evaluator
            .get_identifiers()
            .map(|c| (c, evaluator.get_ident_bit(c, pass)))
            .collect(),
    })
}

fn print_ast(expression: &String, mut mode: AstPrintMode) -> Result<(), String> {
    let tokens = tokenizer::tokenize(expression, true)?;
    let mut parser = ast::Parser::new(tokens, expression);
    let ast = parser.parse()?;
    let tree = ast::ast_to_tree(&ast);
    let nodes = ast::count_nodes(&ast);
    if (mode == AstPrintMode::Default || mode == AstPrintMode::Extended) && nodes > 10 {
        match show_prompt(
            "Performance warning: switch to more efficient pretty printer: [y|n]:",
            &vec!["n", "y"],
        )
        .as_str()
        {
            "y" => {
                if mode == AstPrintMode::Default {
                    mode = AstPrintMode::Pretty
                } else {
                    mode = AstPrintMode::PrettyExtended
                }
            }
            _ => {}
        }
    }
    match mode {
        AstPrintMode::Default => println!("{:#}", tree),
        AstPrintMode::Pretty => println!("{}", tree),
        AstPrintMode::Extended => println!("{:#.2}", tree),
        AstPrintMode::PrettyExtended => println!("{:.2}", tree),
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Eval { expression } => match evaluate_bool_exp(&expression) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        Commands::Table {
            expression,
            filter_false,
            filter_true,
        } => {
            if filter_true && filter_false {
                eprintln!("Cannot filter for both true and false");
                return;
            }
            let filter = if filter_true {
                |result: &EvaluatorPassResult| result.result
            } else if filter_false {
                |result: &EvaluatorPassResult| !result.result
            } else {
                |_result: &EvaluatorPassResult| true
            };
            match evaluate_truth_table(&expression) {
                Ok(result) => {
                    let mut header: Vec<String> = result[0]
                        .ident_states
                        .iter()
                        .sorted_by(|(a, _), (b, _)| a.cmp(b))
                        .map(|(c, _)| c.to_string())
                        .collect();
                    header.push(String::from("Result"));

                    let mut table_builder = Builder::new();
                    result
                        .iter()
                        .filter(|res: &&EvaluatorPassResult| filter(*res))
                        .for_each(|row| {
                            table_builder.push_record(
                                row.ident_states
                                    .iter()
                                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                                    .map(|(_, b)| b.to_string()),
                            )
                        });

                    table_builder.insert_column(
                        result[0].ident_states.iter().count(),
                        result
                            .iter()
                            .filter(|res: &&EvaluatorPassResult| filter(*res))
                            .map(|row| row.result.to_string()),
                    );
                    table_builder.insert_record(0, header);

                    let mut table = table_builder.build();
                    table.with(Style::rounded());
                    println!("{}", table);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
        }
        Commands::Truth { inputs, expression } => match parse_ident_states(&inputs) {
            Ok(pass) => match evaluate_pass(&expression, pass) {
                Ok(result) => {
                    println!("{}", result.result);
                    return;
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        Commands::Ast {
            expression,
            pretty,
            extended,
        } => {
            let mode = AstPrintMode::from(pretty, extended);
            if let Err(e) = print_ast(&expression, mode) {
                eprintln!("{}", e);
                return;
            }
        }
    }
}

fn parse_ident_states(input: &Vec<String>) -> Result<usize, String> {
    if input.len() == 1 {
        let input = input[0].clone();
        if input.chars().all(|c| c == '0' || c == '1') {
            Ok(usize::from_str_radix(&*input, 2).unwrap())
        } else if input.eq_ignore_ascii_case("true") {
            return Ok(1);
        } else if input.eq_ignore_ascii_case("false") {
            return Ok(0);
        } else if input.chars().all(|c| c.is_numeric()) {
            return Ok(usize::from_str_radix(&*input, 10).unwrap());
        } else {
            Err(format!("Invalid input: {}\nEither must be a boolean (true|false|0|1) or a binary string (010101) or number (uint)", input))
        }
    } else {
        let sum = input
            .iter()
            .enumerate()
            .map(|(i, c)| {
                return if c.eq_ignore_ascii_case("true") || c.eq_ignore_ascii_case("1") {
                    Ok(1 << i)
                } else if c.eq_ignore_ascii_case("false") | c.eq_ignore_ascii_case("0") {
                    Ok(0)
                } else {
                    Err(format!(
                        "Invalid input: {} at index {}\nEither must be a boolean (true|false|0|1)",
                        c, i
                    ))
                };
            })
            .sum();

        sum
    }
}
