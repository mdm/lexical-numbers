use std::{collections::VecDeque, fmt};

const GROUP_NAMES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

const TENS_NAMES: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SMALL_NAMES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

pub struct LexicalNumber {
    inner: usize,
}

impl From<LexicalNumber> for usize {
    fn from(value: LexicalNumber) -> Self {
        value.inner
    }
}

impl From<usize> for LexicalNumber {
    fn from(value: usize) -> Self {
        LexicalNumber { inner: value }
    }
}

impl fmt::Display for LexicalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inner == 0 {
            write!(f, "{}", SMALL_NAMES[self.inner])?;
        }

        let mut stack = VecDeque::with_capacity(7);
        let mut value = self.inner;
        while value > 0 {
            let group_value = value % 1000;
            value /= 1000;
            stack.push_front(group_value);
        }

        for (i, group_value) in stack.iter().enumerate() {
            if *group_value == 0 {
                continue;
            }

            if i > 0 {
                write!(f, " ")?;
            }

            let group = stack.len() - i - 1;

            let hundreds = group_value / 100;
            let remainder = group_value % 100;

            if hundreds > 0 {
                write!(f, "{}-hundred", SMALL_NAMES[hundreds])?;

                if remainder > 0 {
                    write!(f, "-")?;
                }
            }

            if remainder < 20 {
                write!(f, "{}", SMALL_NAMES[remainder])?;
            } else {
                let tens = remainder / 10;
                let ones = remainder % 10;
                write!(f, "{}", TENS_NAMES[tens])?;

                if ones > 0 {
                    write!(f, "-{}", SMALL_NAMES[ones])?;
                }
            }

            if group > 0 {
                write!(f, "-{}", GROUP_NAMES[group])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_234_567_890() {
        let value = LexicalNumber::from(1_234_567_890);
        let result = format!("{}", value);
        assert_eq!(&result, "one-billion two-hundred-thirty-four-million five-hundred-sixty-seven-thousand eight-hundred-ninety");
    }

    #[test]
    fn test_1_034_567_890() {
        let value = LexicalNumber::from(1_034_567_890);
        let result = format!("{}", value);
        assert_eq!(&result, "one-billion thirty-four-million five-hundred-sixty-seven-thousand eight-hundred-ninety");
    }

    #[test]
    fn test_1_004_567_890() {
        let value = LexicalNumber::from(1_004_567_890);
        let result = format!("{}", value);
        assert_eq!(&result, "one-billion four-million five-hundred-sixty-seven-thousand eight-hundred-ninety");
    }

    #[test]
    fn test_1_234_000_000() {
        let value = LexicalNumber::from(1_234_000_000);
        let result = format!("{}", value);
        assert_eq!(&result, "one-billion two-hundred-thirty-four-million");
    }
}
