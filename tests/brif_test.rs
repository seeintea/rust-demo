use bfrs::bfir::*;

#[test]
fn test_bfir() {

  assert_eq!(
    compile("+-><[,.]").unwrap(),
    vec![
      BfIR::ValueAdd(1),
      BfIR::ValueSub(1),
      BfIR::PtrForward(1),
      BfIR::PtrBack(1),
      BfIR::Jz,
      BfIR::InputValue,
      BfIR::OutputValue,
      BfIR::Jnz,
    ]
  );

  match compile("[").unwrap_err().kind {
    CompileErrorKind::UnclosedLeftOperator => {}
    _ => panic!(),
  };

  match compile("[,.]]").unwrap_err().kind {
    CompileErrorKind::UnexpectedRightOperator => {}
    _ => panic!(),
  };

  let mut code = compile("[++++ ++++ ++++ +]").unwrap();
  optimize(&mut code);
  assert_eq!(code, vec![BfIR::Jz, BfIR::ValueAdd(13), BfIR::Jnz]);
}