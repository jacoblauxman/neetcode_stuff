// 273. Integer to English Words
// note: `daily` for 8/7/24

pub const DIGITS: [&'static str; 20] = [
    "",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

pub const TENS: [&'static str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

pub const PLACE_VALUES: [&'static str; 5] =
    [" ", " Hundred ", " Thousand ", " Million ", " Billion "];

pub fn number_to_words(num: i32) -> String {
    if num == 0 {
        "Zero".to_string()
    } else {
        nums_words_helper(num as usize)
    }
}

pub fn nums_words_helper(num: usize) -> String {
    let mut num_str = String::new();

    match num {
        _ if num < 20 => num_str.push_str(DIGITS[num]),
        _ if num < 100 => {
            num_str.push_str(TENS[num / 10]);
            // if num % 10 != 0 {
            //     num_str.push_str(PLACE_VALUES[0]);
            // } // attempting to get rid of ending white space / extra alloc (trim() + to_string())
            num_str.push_str(PLACE_VALUES[0]);
            num_str.push_str(DIGITS[num % 10])
        }
        _ if num < 1_000 => {
            num_str.push_str(DIGITS[num / 100]);
            num_str.push_str(PLACE_VALUES[1]);
            num_str.push_str(&nums_words_helper(num % 100))
        }

        _ if num < 1_000_000 => {
            num_str.push_str(&nums_words_helper(num / 1_000));
            num_str.push_str(PLACE_VALUES[2]);
            num_str.push_str(&nums_words_helper(num % 1_000))
        }

        _ if num < 1_000_000_000 => {
            num_str.push_str(&nums_words_helper(num / 1_000_000));
            num_str.push_str(PLACE_VALUES[3]);
            num_str.push_str(&nums_words_helper(num % 1_000_000))
        }
        _ => {
            num_str.push_str(&nums_words_helper(num / 1_000_000_000));
            num_str.push_str(PLACE_VALUES[4]);
            num_str.push_str(&nums_words_helper(num % 1_000_000_000))
        }
    }

    num_str.trim().to_string()
    // Note: would otherwise require conditionals within match + an alt arr of `PLACE_VALUES` containing 'trimmed' str versions
}
