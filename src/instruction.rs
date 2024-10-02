#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    MoveRight, // >
    MoveLeft,  // <
    Add,       // +
    Sub,       // -
    Output,    // .
    Input,     // ,
    LoopStart, // [
    LoopEnd,   // ]
}

impl Instruction {
    pub fn parse(input: &str) -> Vec<Self> {
        input.chars().flat_map(Self::try_from).collect()
    }
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '>' => Self::MoveRight,
            '<' => Self::MoveLeft,
            '+' => Self::Add,
            '-' => Self::Sub,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::LoopStart,
            ']' => Self::LoopEnd,
            _ => Err(())?,
        })
    }
}

impl From<Instruction> for char {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::MoveRight => '>',
            Instruction::MoveLeft => '<',
            Instruction::Add => '+',
            Instruction::Sub => '-',
            Instruction::Output => '.',
            Instruction::Input => ',',
            Instruction::LoopStart => '[',
            Instruction::LoopEnd => ']',
        }
    }
}
