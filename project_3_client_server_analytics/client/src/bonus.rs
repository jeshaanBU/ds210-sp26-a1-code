extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::dataset::Value;
use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Your solution goes here.
fn split_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
        } else if chars[i] == '(' || chars[i] == ')' || chars[i] == '!' {
            tokens.push(chars[i].to_string());
            i += 1;
        } else if i + 1 < chars.len() && chars[i] == '=' && chars[i + 1] == '=' {
            tokens.push("==".to_string());
            i += 2;
        } else if chars[i] == '"' {
            let mut j = i + 1;
            let mut s = String::new();
            while j < chars.len() && chars[j] != '"' {
                s.push(chars[j]);
                j += 1;
            }
            tokens.push(format!("\"{}\"", s));
            i = j + 1;
        } else {
            let mut j = i;
            let mut s = String::new();
            while j < chars.len()
                && !chars[j].is_whitespace()
                && chars[j] != '('
                && chars[j] != ')'
                && chars[j] != '!'
            {
                if j + 1 < chars.len() && chars[j] == '=' && chars[j + 1] == '=' {
                    break;
                }
                s.push(chars[j]);
                j += 1;
            }
            tokens.push(s);
            i = j;
        }
    }

    tokens
}


fn make_value(token: &String) -> Value {
    if token.starts_with('"') && token.ends_with('"') {
        Value::String(token[1..token.len() - 1].to_string())
    } else {
        Value::Integer(token.parse::<i32>().unwrap())
    }
}

fn parse_filter(tokens: &Vec<String>, pos: &mut usize) -> Condition {
    let mut left;

    if tokens[*pos] == "!" {
        *pos += 1;
        left = Condition::Not(Box::new(parse_filter(tokens, pos)));
    } else if tokens[*pos] == "(" {
        *pos += 1;
        left = parse_filter(tokens, pos);
        *pos += 1; // skip ")"
    } else {
        let column = tokens[*pos].clone();
        *pos += 1;
        *pos += 1; // skip "=="
        let value = make_value(&tokens[*pos]);
        *pos += 1;
        left = Condition::Equal(column, value);
    }

    while *pos < tokens.len() && (tokens[*pos] == "AND" || tokens[*pos] == "OR") {
        let op = tokens[*pos].clone();
        *pos += 1;
        let right = parse_filter(tokens, pos);

        if op == "AND" {
            left = Condition::And(Box::new(left), Box::new(right));
        } else {
            left = Condition::Or(Box::new(left), Box::new(right));
        }
    }

    left
}


fn parse_query_from_string(input: String) -> Query {
    let tokens = split_tokens(&input);
    let mut pos = 0;

    pos += 1; // filter
    let filter = parse_filter(&tokens, &mut pos);

    pos += 2; // group by
    let group_by = tokens[pos].clone();
    pos += 1;

    let agg_name = tokens[pos].clone();
    pos += 1;
    let agg_column = tokens[pos].clone();

    let aggregation = match agg_name.as_str() {
        "COUNT" => Aggregation::Count(agg_column),
        "SUM" => Aggregation::Sum(agg_column),
        "AVERAGE" => Aggregation::Average(agg_column),
        _ => panic!("invalid aggregation"),
    };

    Query::new(filter, group_by, aggregation)
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}