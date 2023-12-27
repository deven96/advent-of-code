/// 
///
///1abc2
///pqr3stu8vwx
///a1b2c3d4e5f
///treb7uchet
///
/// 12 + 38 + 15 + 77
///
/// split by newlines
/// create FoundDigits from each line
/// get output from every FoundDigits
/// sum across FoundDigits
fn main() {
    println!("{}", parse_all_lines("1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            abcdefg
            treb7uchet"));
}

// Result<T, E>
fn parse_all_lines(input: &str) -> u32 {
    input.split("\n")
        .map(|line| FoundDigits::new(line))
        .flat_map(|found_digit| found_digit.output())
        .reduce(|acc, found_output| acc + found_output)
        .unwrap_or_default()
}


/// None, Some(T)
#[derive(Debug)]
struct FoundDigits {
    first: Option<u32>,
    last: Option<u32>,
}

/// Iterators -> map, reduce, fold, collect
impl FoundDigits {
    fn new(input: &str) -> Self {
        Self {
            first: Self::parse_with_iterator(input.chars()),
            // DoubleEnded Iterator
            last: Self::parse_with_iterator(input.chars().rev()),
        }
    }

    /// abcde
    /// 77 
    /// FoundDigits { first: None ... }
    fn output(&self) -> Option<u32> {
        if let (Some(first), Some(last)) = (self.first, self.last){ 
            return Some(first * 10 + last)
        }
        None
    }

    fn parse_with_iterator(mut iter: impl Iterator<Item = char>) -> Option<u32> {
        let mut output = None;
        while let Some(found) = iter.next() {
            if output.is_some() {
                break;
            };
            // char -> u32
            output = found.to_digit(10);
        }
        output
    }
}
