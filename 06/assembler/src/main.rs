mod parser;
mod translator;

use parser::parse_file;
use translator::translate_instruction;

fn main() {
    const PATH: &str = "/home/sam/src/nand2tetris/projects/06/add/Add.asm";

    for instruction in parse_file(PATH) {
        translate_instruction(instruction);
    }
}

/******************************************************************************
*
* SymbolTable:
*
* Keeps track of any symbol references. On init, we run the parser and iterate over it to populate
* the symbol table. Then, we iterate over it again, this time doing the actual code translation.
*
*******************************************************************************/
