fn main() {
    is_palindrome(99);
}

pub fn is_palindrome(x: i32) -> bool {
    if x == 0 {
        return true;
    }

    let mut thing: i32 = x;
    let mut reversed: i32 = 0;
    let mut pop: i32;

    while thing > 0 {
        pop = thing % 10;
        thing = thing / 10;

        reversed = (reversed * 10) + pop;
    }
    return reversed == x;
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;
    #[test]
    fn one() {
        assert_eq!(true, is_palindrome(121))
    }

    #[test]
    fn two() {
        assert_eq!(false, is_palindrome(-121))
    }

    #[test]
    fn three() {
        assert_eq!(false, is_palindrome(10))
    }

    #[test]
    fn four() {
        assert_eq!(true, is_palindrome(0))
    }

    #[test]
    fn five() {
        assert_eq!(true, is_palindrome(1111))
    }
}
