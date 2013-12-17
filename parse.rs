/// Parser

use std::char;


#[deriving(ToStr)]
pub enum Expr {
    Empty,
    Range(char, char),
    Concatenate(~Expr, ~Expr),
    Alternate(~Expr, ~Expr),
    Repeat(~Expr, uint, Option<uint>, Greedy),
}


#[deriving(ToStr)]
pub enum Greedy {
    NonGreedy,
    Greedy,
}


/// Parse a regular expression into an AST.
pub fn parse(s: &str) -> Expr {
    let (e, s_) = parse_main(s);
    if s_.len() > 0 {
        fail!("invalid syntax")
    } else {
        e
    }
}


fn parse_main<'a>(s: &'a str) -> (Expr, &'a str) {
    let mut stack: ~[Expr] = ~[];
    parse_loop(s, &mut stack)
}


fn parse_loop<'a>(s: &'a str, stack: &mut ~[Expr]) -> (Expr, &'a str) {
    match uncons(s) {
        Some((c, s1)) => {
            match c {
                '.' => {
                    stack.push(Range('\0', char::MAX));
                    parse_loop(s1, stack)
                },
                '|' => {
                    let left = coalesce(stack);
                    let (right, s_) = parse_loop(s1, stack);
                    (Alternate(~left, ~right), s_)
                },
                '(' => {
                    // Collect everything before the parens
                    let before = coalesce(stack);
                    // Parse inside the parens
                    let (inner, s_) = parse_loop(s1, stack);
                    // Match the closing paren
                    match uncons(s_) {
                        Some((')', s_1)) => (concatenate(before, inner), s_1),
                        _ => fail!("unbalanced parenthesis")
                    }
                },
                ')' => (coalesce(stack), s),
                _ => {
                    stack.push(Range(c, c));
                    parse_loop(s1, stack)
                }
            }
        },
        None => (coalesce(stack), s)
    }
}


/// Fold the elements of the vector using `Concatenate`, clearing the
/// vector in the process.
fn coalesce(stack: &mut ~[Expr]) -> Expr {
    while stack.len() > 1 {
        let right = stack.pop();
        let left = stack.pop();
        stack.push(concatenate(left, right));
    }
    stack.pop_opt().unwrap_or(Empty)
}


/// Smart constructor for `Concatenate`.  If either of the children is
/// `Empty`, it is ignored.
fn concatenate(left: Expr, right: Expr) -> Expr {
    match left {
        Empty => right,
        _ => match right {
            Empty => left,
            _ => Concatenate(~left, ~right)
        }
    }
}


/// Return the first element and the rest of a `str`, or `None` if
/// empty.
fn uncons<'a>(s: &'a str) -> Option<(char, &'a str)> {
    if s.len() > 0 {
        Some(s.slice_shift_char())
    } else {
        None
    }
}
