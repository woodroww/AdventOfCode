use thiserror::Error;

pub type Value = i64;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("invalid opcode ({0})")]
    InvalidOpcode(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl TryFrom<Value> for Opcode {
    type Error = Error;
    fn try_from(value: Value) -> Result<Opcode> {
        use Opcode::*;
        Ok(match value {
            1 => Add,
            2 => Multiply,
            99 => Halt,
            _ => return Err(Error::InvalidOpcode(value)),
        })
    }
}
