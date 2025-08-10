mod lsp_definitions;

use std::collections::HashMap;
use lsp_definitions::{*};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn testthing() -> ProgressToken {
    ProgressToken::Integer(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        match testthing() {
            lsp_definitions::ProgressToken::Integer(4) => (),
            _ => {
                panic!("Test");
            }
        }
    }
}
