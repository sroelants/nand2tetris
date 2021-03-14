/******************************************************************************
*
* Code generator:
*
* This module takes a ParsedInstruction and turns it into its
* binary representation.
*
******************************************************************************/

use crate::parser::Instruction;

pub fn translate_instruction(instruction: Instruction) {
    println!("{:?}", instruction);
}
