use std::fmt;

// 将Brainfuck代码用中间代码表示

// https://zh.wikipedia.org/wiki/Brainfuck
// Brainfuck 支持八种运算符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BfIR {
  AddVal(u8),  // + 指针指向的字节的值加一
  SubVal(u8),  // - 指针指向的字节的值减一
  AddPtr(u32), // > 指针加一
  SubPtr(u32), // < 指针减一
  PutByte,     // . 输入内容到指针指向的单元（ASCII码）
  GetByte,     // , 输出指针指向的单元内容（ASCII码）
  Jz,          // [ 如果指针指向的单元值为零，向后跳转到对应的]指令的次一指令处
  Jnz,         // ] 如果指针指向的单元值不为零，向前跳转到对应的[指令的次一指令处
}

// 定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum CompileErrorKind {
  #[error("Unclosed left bracket")]
  UnclosedLeftBracket,
  #[error("Unexpected right bracket")]
  UnexpectedRightBracket,
}

// 定义抛出错误信息格式
#[derive(Debug)]
pub struct CompileError {
  line: u32,
  col: u32,
  kind: CompileErrorKind,
}

impl fmt::Display for CompileError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
  }
}

impl std::error::Error for CompileError {}

// 解析代码
pub fn compile(src: &str) -> Result<Vec<BfIR>, CompileError> {
  let mut code: Vec<BfIR> = vec![]; // 已解析的IR
  let mut skt: Vec<(u32, u32, u32)> = vec![]; // 左括号IR位置、源码行位置、源码列位置
  let mut line = 1;
  let mut col = 0;
  for ch in src.chars() {
    col += 1;
    match ch {
      '\n' => {
        line += 1;
        col = 0;
      },
      '+' => code.push(BfIR::AddVal(1)),
      '-' => code.push(BfIR::SubVal(1)),
      '>' => code.push(BfIR::AddPtr(1)),
      '<' => code.push(BfIR::SubPtr(1)),
      '.' => code.push(BfIR::PutByte),
      ',' => code.push(BfIR::GetByte),
      '[' => {
        let pos = code.len() as u32;
        skt.push((pos, line, col));
        code.push(BfIR::Jz);
      },
      ']' => {
        skt.pop().ok_or(CompileError {
          line,
          col,
          kind: CompileErrorKind::UnexpectedRightBracket,
        })?;
        code.push(BfIR::Jnz);
      },
      _ => {}
    }
   }
   if let Some((_, line, col)) = skt.pop() {
    return Err(CompileError {
      line,
      col,
      kind: CompileErrorKind::UnclosedLeftBracket,
    });
   }
   Ok(code)
}

pub fn optimize(code: &mut Vec<BfIR>) {
  let len = code.len();
  let mut i = 0;
  let mut pc = 0;

  macro_rules! _mergr_ir {
    ($variant:ident, $x:ident) => {{
      let mut j = i + 1;
      while j < len {
        if let $variant(d) = code[j] {
          $x = $x.wrapping_add(d);
        } else {
          break;
        }
        j += 1;
      }
      i = j;
      code[pc] = $variant($x);
      pc += 1;
    }};
  }

  macro_rules! _normal_ir {
    () => {{
      code[pc] = code[i];
      pc += 1;
      i += 1;
    }};
  }

  use BfIR::*;
  while i < len {
    match code[i] {
      AddPtr(mut x) => _mergr_ir!(AddPtr, x),
      SubPtr(mut x) => _mergr_ir!(SubPtr, x),
      AddVal(mut x) => _mergr_ir!(AddVal, x),
      SubVal(mut x) => _mergr_ir!(SubVal, x),
      GetByte => _normal_ir!(),
      PutByte => _normal_ir!(),
      Jz => _normal_ir!(),
      Jnz => _normal_ir!(),
    }
  }
  code.truncate(pc);
  code.shrink_to_fit();
}

#[test]
fn test_brif() {
    assert_eq!(
      compile("+-><[,.]").unwrap(),
      vec![
        BfIR::AddVal(1),
        BfIR::SubVal(1),
        BfIR::AddPtr(1),
        BfIR::SubPtr(1),
        BfIR::Jz,
        BfIR::GetByte,
        BfIR::PutByte,
        BfIR::Jnz,
      ]
    );

    match compile("[").unwrap_err().kind {
      CompileErrorKind::UnclosedLeftBracket => {}
      _ => panic!(),
    };

    match compile("]").unwrap_err().kind {
      CompileErrorKind::UnexpectedRightBracket => {}
      _ => panic!(),
    };

    match compile("[,.]]").unwrap_err().kind {
        CompileErrorKind::UnexpectedRightBracket => {}
        _ => panic!(),
    }

    let mut code = compile("[+++++]").unwrap();
    optimize(&mut code);
    assert_eq!(code, vec![BfIR::Jz, BfIR::AddVal(5), BfIR::Jnz]);
}

