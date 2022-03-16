static mut CONTER: u64 = 0;

#[ic_cdk_macros::query]
fn get() -> u64 {
    unsafe { CONTER }
}

#[ic_cdk_macros::update]
fn increment() {
    unsafe {
        CONTER += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_increment_validity() {
        increment();
        assert_eq!(unsafe {CONTER}, 1);
    }

    #[test]
    fn check_increment_validity_with_specific_status() {
        unsafe { CONTER = 100; }
        increment();
        assert_eq!(unsafe {CONTER}, 101);
    }
}