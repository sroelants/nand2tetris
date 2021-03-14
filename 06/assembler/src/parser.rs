/******************************************************************************
*
* Parser:
*
* Responsible for reading a file and returning an iterable of parsed
* instructions. Outward facing api conists only of the `parse_file` function.
*
******************************************************************************/

use regex::Regex;
use std::fs;

#[derive(Debug)]
pub enum Symbol {
    Name(String),
    Addr(u32),
}

#[derive(Debug)]
pub enum Instruction {
    A {
        symbol: Symbol,
    },

    L {
        symbol: Symbol,
    },

    C {
        dest: Option<String>,
        comp: Option<String>,
        jump: Option<String>,
    },
}

pub fn parse_file(path: &str) -> Vec<Instruction> {
    let instructions = fs::read_to_string(path).expect(&format!("File not found: {}", path));

    instructions
        // Normalize the code: Remove whitespace and comments
        .split('\n')
        .map(|s| s.to_owned())
        .map(remove_comments)
        .map(remove_whitespace)
        .filter(|line| !line.is_empty())
        // Parse the instructions
        .map(|instr| parse_instruction(&instr))
        // Filter out invalid lines and unwrap the remaining instructions
        .filter(|opt| opt.is_some())
        .map(|some| some.unwrap())
        .collect::<Vec<_>>()
}

// Question: How strict do I want to be with validation? Do I just assume the
// inputs to be well-formed?
fn parse_instruction(instruction: &str) -> Option<Instruction> {
    let a_regex = Regex::new("@(.*)").unwrap();
    let l_regex = Regex::new(r"\((.*)\)").unwrap();
    let c_regex = Regex::new(r"(:?(.*)=)?(.*)(:?;(.*))?").unwrap();

    if a_regex.is_match(instruction) {
        parse_a(instruction)
    } else if l_regex.is_match(instruction) {
        parse_l(instruction)
    } else if c_regex.is_match(instruction) {
        parse_c(instruction)
    } else {
        None
    }
}

/// Parse A-instruction
///
/// # Example
///
/// ```
/// let answer = parse_a("@foo")
///
/// assert_eq!(answer, Some(Instruction::A { symbol: Name("foo")}));
/// ```
fn parse_a(instruction: &str) -> Option<Instruction> {
    let a_regex = Regex::new("@(.*)").unwrap();

    let symbol_str = a_regex
        .captures_iter(instruction)
        .next()
        .unwrap()
        .get(1) // Get the first capture group
        .unwrap()
        .as_str()
        .to_owned();

    parse_symbol(symbol_str).map(|symbol| Instruction::A { symbol })
}

//TODO: Make this parsing logic more specific, check for illegal characters, etc...
// If the symbol has illegal characters, return None.
fn parse_symbol(string: String) -> Option<Symbol> {
    if let Ok(parsed) = string.parse::<u32>() {
        Some(Symbol::Addr(parsed))
    } else {
        Some(Symbol::Name(string))
    }
}

/// Parse L-instruction
///
/// # Example
///
/// ```
/// let answer = parse_l("(FOO)")
///
/// assert_eq!(answer, Some(Instruction::A { symbol: Name("FOO")}));
/// ```
fn parse_l(instruction: &str) -> Option<Instruction> {
    let l_regex = Regex::new(r"\((.*)\)").unwrap();

    let symbol_str = l_regex
        .captures_iter(instruction)
        .next()
        .unwrap()
        .get(1) // Get the first capture group
        .unwrap()
        .as_str()
        .to_owned();

    parse_symbol(symbol_str).map(|symbol| Instruction::L { symbol })
}

/// Parse C-instruction
///
/// # Example
///
/// ```
/// let answer = parse_l("D=D+A;JMP")
///
/// assert_eq!(answer, Some(Instruction::C { dest: Some("D"), comp: Some("D+A"), jump: Some("JMP")}));
/// ```
fn parse_c(instruction: &str) -> Option<Instruction> {
    let c_regex = Regex::new(r"(?:(?P<dest>.*)=)?(?P<comp>.*)(?:;(?P<jump>.*))?").unwrap();
    let capture = c_regex.captures_iter(instruction).next().unwrap();

    Some(Instruction::C {
        dest: capture.name("dest").map(|m| m.as_str().to_owned()),
        comp: capture.name("comp").map(|m| m.as_str().to_owned()),
        jump: capture.name("jump").map(|m| m.as_str().to_owned()),
    })
}

/******************************************************************************
 *
 * Helpers
 *
 ******************************************************************************/

fn remove_comments(string: String) -> String {
    let comments_regex = Regex::new(r"//.*$").unwrap();
    comments_regex.replace_all(&string, "").to_string()
}

fn remove_whitespace(string: String) -> String {
    let ws_regex = Regex::new(r"\s*").unwrap();
    ws_regex.replace_all(&string, "").to_string()
}
