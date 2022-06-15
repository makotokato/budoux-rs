#![cfg_attr(not(any(test)), no_std)]

extern crate alloc;

pub mod parser;
mod unicode_block;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
