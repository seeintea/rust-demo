use std::fmt;
use std::error;

/// Brainfuck state
/// 
/// document: https://esolangs.org/wiki/Brainfuck
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BfIR {
  PtrForward(u32),
  PtrBack(u32),
  ValueAdd(u8),
  ValueSub(u8),
  OutputValue,
  InputValue,
  Jz,
  Jnz,
}

/// CompileErrorKind
/// 
/// [ [ ] : UnclosedLeftOperator
/// 
/// [ ] ] : UnexpectedRightOperator
#[derive(Debug, thiserror::Error)]
pub enum CompileErrorKind {
  #[error("Unclosed left operator")]
  UnclosedLeftOperator,
  #[error("Unexpected right operator")]
  UnexpectedRightOperator,
}

/// CompileError
/// 
/// throw error info includes row、col and error kind
#[derive(Debug)]
pub struct CompileError {
  pub row: u32,
  pub col: u32,
  pub kind: CompileErrorKind,
}
// realization fmt::Display error::Error for CompileError
impl fmt::Display for CompileError {
  fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(format, "{} at line {}:{}", self.kind, self.row, self.col)
  }
}

impl error::Error for CompileError {}

/// compile
/// # Example
/// ```
/// compile("+-") // Vec[BfIR::ValueAdd(1), BfIR::ValueSub(1),]
/// ```
pub fn compile(code: &str) -> Result<Vec<BfIR>, CompileError> {
  let mut res: Vec<BfIR> = vec![];
  // record Jz's row and col
  let mut jz_record: Vec<(u32, u32)> = vec![];
  let mut row = 1;
  let mut col = 0;
  for c in code.chars() {
    col += 1;
    match c {
        '\n' => {
          row += 1;
          col = 0;
        },
        '>' => res.push(BfIR::PtrForward(1)),
        '<' => res.push(BfIR::PtrBack(1)),
        '+' => res.push(BfIR::ValueAdd(1)),
        '-' => res.push(BfIR::ValueSub(1)),
        '.' => res.push(BfIR::OutputValue),
        ',' => res.push(BfIR::InputValue),
        '[' => {
          jz_record.push((row, col));
          res.push(BfIR::Jz);
        },
        ']' => {
          jz_record.pop().ok_or(CompileError {
            row, col, kind: CompileErrorKind::UnexpectedRightOperator
          })?;
          res.push(BfIR::Jnz);
        },
        _ => {}
    }
  }
  if let Some((row, col)) = jz_record.pop() {
    return Err(CompileError {
      row, col, kind: CompileErrorKind::UnclosedLeftOperator
    })
  }
  Ok(res)
}

/// optimize
/// in Brainfuck has many consecutive identical characters use it can compress 
/// # Example
/// ```
/// let mut code = compile("[+++++]").unwrap();
/// optimize(&code) // [BfIR::Jz, BfIR::ValueAdd(5), BfIR::Jnz]
/// ```
pub fn optimize(code: &mut Vec<BfIR>) {
  let len = code.len();
  let mut idx = 0;
  let mut current = 0;

  // use macro_rules
  macro_rules! _merge_ir {
    ($opt:ident, $value:ident) => {{
      let mut next = idx + 1;
      while next < len {
        if let $opt(val) = code[next] {
          $value = $value.wrapping_add(val);
        } else {
          break;
        }
        next += 1;
      }
      idx = next;
      code[current] = $opt($value);
      current += 1;
    }};
  }
  macro_rules! _normal_ir {
    () => {{
      code[current] = code[idx];
      current += 1;
      idx += 1;
    }};
  }

  use BfIR::*;
  while idx < len {
    match code[idx] {
      PtrForward(mut val) => _merge_ir!(PtrForward, val),
      PtrBack(mut val) => _merge_ir!(PtrBack, val),
      ValueAdd(mut val) => _merge_ir!(ValueAdd, val),
      ValueSub(mut val) => _merge_ir!(ValueSub, val),
      OutputValue => _normal_ir!(),
      InputValue => _normal_ir!(),
      Jz => _normal_ir!(),
      Jnz => _normal_ir!(),
    }
  }
  code.truncate(current);
  code.shrink_to_fit();
}