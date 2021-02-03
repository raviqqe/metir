use super::instruction::Instruction;
use super::terminal_instruction::TerminalInstruction;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    instructions: Vec<Instruction>,
    terminal_instruction: TerminalInstruction,
}

impl Block {
    pub fn new(
        instructions: Vec<Instruction>,
        terminal_instruction: impl Into<TerminalInstruction>,
    ) -> Self {
        Self {
            instructions: instructions.into_iter().collect(),
            terminal_instruction: terminal_instruction.into(),
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn terminal_instruction(&self) -> &TerminalInstruction {
        &self.terminal_instruction
    }
}
