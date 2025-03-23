use crate::evaluator::EvaluatorPassResult;
use clap::{Parser, Subcommand};
use tabled::builder::Builder;
use tabled::settings::Style;

mod ast;
mod evaluator;
mod tokenizer;

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

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        name = "-eval",
        about = "evaluates the given boolean expression, identifiers are not supported",
        aliases = &["-e"]
    )]
    Eval { expression: String },
    #[command(
        name = "-Table",
        about = "prints the truth table for the given boolean expression, identifiers are supported",
        aliases = &["-T"]
    )]
    Table { expression: String },
    #[command(
        name = "-truth",
        about = "evaluates the given boolean expression with the given inputs, identifiers are supported",
        aliases = &["-t"]
    )]
    Truth {
        #[arg(name = "identifier_values", required = true, num_args = 1..)]
        inputs: Vec<String>,
        expression: String,
    },
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
        Commands::Table { expression } => match evaluate_truth_table(&expression) {
            Ok(result) => {
                let mut header: Vec<String> = result[0]
                    .ident_states
                    .iter()
                    .map(|(c, _)| c.to_string())
                    .collect();
                header.push(String::from("Result"));

                let mut table_builder = Builder::new();
                result.iter().for_each(|row| {
                    table_builder.push_record(row.ident_states.iter().map(|(_, b)| b.to_string()))
                });

                table_builder.insert_column(
                    result[0].ident_states.iter().count(),
                    result.iter().map(|row| row.result.to_string()),
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
        },
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
