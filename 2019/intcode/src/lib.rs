use thiserror::Error;

pub type Value = i64;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("invalid opcode ({0})")]
    InvalidOpcode(Value),
    #[error("invalid mode ({0})")]
    InvalidMode(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Store = 3,
    Output = 4,
    Halt = 99,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Opcode::Add => "Add",
            Opcode::Multiply => "Multiply",
            Opcode::Store => "Store",
            Opcode::Output => "Output",
            Opcode::Halt => "Halt",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<Value> for Opcode {
    type Error = Error;
    fn try_from(value: Value) -> Result<Opcode> {
        use Opcode::*;
        Ok(match value {
            1 => Add,
            2 => Multiply,
            3 => Store,
            4 => Output,
            99 => Halt,
            _ => return Err(Error::InvalidOpcode(value)),
        })
    }
}

// parameters are the additional values included in the array after the opcode value

// Parameter modes are stored in the same value as the instruction's opcode.
// Param mode:
//     0  Position     Parameter is an address of a value.
//     1  Immediate    Parameter is the value (only for reads).
//     2  Relative     Parameter is an offset to relative base address, which produces the value.

pub enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl TryFrom<Value> for Mode {
    type Error = Error;
    fn try_from(value: Value) -> Result<Mode> {
        use Mode::*;
        Ok(match value {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => return Err(Error::InvalidMode(value)),
        })
    }
}
