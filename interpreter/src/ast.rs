use crate::parser::Expression;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn generate_offset(length: usize) -> String {
    std::iter::repeat(' ').take(length).collect::<String>()
}

fn has_leafs(expr: &Expression) -> bool {
    match expr {
        Expression::Literal(l) => false,
        _ => true,
    }
}

fn calculate_height(expr: &Expression, height: usize) -> usize {
    match expr {
        Expression::Binary(left, operator, right) => {
            let left_height = calculate_height(left, height + 1);
            let right_height = calculate_height(right, height + 1);
            std::cmp::max(left_height, right_height)
        }
        Expression::Grouping(expr) => calculate_height(expr, height + 1),
        _ => height,
    }
}

fn get_widest(levels: &Vec<Vec<String>>) -> usize {
    levels
        .iter()
        .map(|l| l.iter().map(|i| i.len()).max().unwrap())
        .max()
        .unwrap()
}

struct Node {
    x: usize,
    y: usize,
    representation: String,
}

fn visit_node(expr: Box<Expression>, depth: usize, levels: &mut Vec<Vec<String>>) {
    let representation = match *expr {
        Expression::Binary(left, operator, right) => {
            visit_node(left, depth + 1, levels);
            visit_node(right, depth + 1, levels);
            operator.to_string()
        }
        Expression::Grouping(expr) => {
            visit_node(expr, depth + 1, levels);
            String::from("GR")
        }
        Expression::Unary(token_type, expr) => {
            visit_node(expr, depth + 1, levels);
            String::from("UN")
        }
        Expression::Error(err) => String::from("Err"),
        Expression::Literal(literal) => literal.to_string(),
        _ => format!("{:#?}", expr).to_string(),
    };
    match levels.get(depth) {
        Some(level) => levels[depth].push(representation),
        _ => levels[depth] = vec![representation],
    }
}

pub fn print_ast(expr: Box<Expression>) {
    let height = calculate_height(&(*expr), 0);
    let mut levels: Vec<Vec<String>> = vec![Vec::new(); height + 1];

    visit_node(expr, 0, &mut levels);

    let branch_width = get_widest(&levels);
    let middle = height * branch_width;
    for (x, level) in levels.iter().enumerate() {
        for (y, node) in level.iter().enumerate() {
            if y == 0 {
                print!("{}", generate_offset(middle - x * branch_width));
            }
            print!(
                "{}{}{}",
                generate_offset(branch_width),
                node,
                generate_offset(branch_width)
            );
        }
        println!();
    }
}
