pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(double_input(2), 4);
    }
}
