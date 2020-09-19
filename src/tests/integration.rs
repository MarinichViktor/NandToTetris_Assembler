use crate::parser::compiler::compile;
use super::fixtures::*;

#[test]
fn test_compiler_for_add_asm() {
    assert_eq!(compile(String::from(ADD_ASM)), ADD_HACK);
}

#[test]
fn test_compiler_for_max_asm() {
    assert_eq!(compile(String::from(MAX_ASM)), MAX_HACK);
}

#[test]
fn test_compiler_for_maxl_asm() {
    assert_eq!(compile(String::from(MAXL_ASM)), MAXL_HACK);
}

#[test]
fn test_compiler_for_rect_asm() {
    assert_eq!(compile(String::from(RECT_ASM)), RECT_HACK);
}

#[test]
fn test_compiler_for_pongl_asm() {
  assert_eq!(compile(String::from(PONGL_ASM)), PONGL_HACK);
}

#[test]
fn test_compiler_for_pong_asm() {
    assert_eq!(compile(String::from(PONG_ASM)), PONG_HACK);
}