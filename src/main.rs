fn main() {


    //numbers_to_words(number: u32);
    println!("Hello, world!");
}

fn number_as_word_is_one_word(number: u32) -> bool {
    if number < 20 {
        true
    } else if number > 100 {
        false
    } else if number % 10 == 0 {
        true
    } else {
        false
    }
}

fn numbers_to_words(number: u32) -> &'static str {
    match number {
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => "one"
    }
}

#[cfg(test)]
mod tests {
    use crate::numbers_to_words;
    use crate::number_as_word_is_one_word;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    macro_rules! numbers_to_words_tests {

        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                let (entry, expected) = $value;
                    let word = numbers_to_words(entry);
                    assert_eq!(word, expected);
                }
            )*
         }
    }


    numbers_to_words_tests! {
        GIVEN_1_WHEN_numbers_to_words_THEN_returns_one: (1, "one"),
        GIVEN_2_WHEN_numbers_to_words_THEN_returns_two: (2, "two"),
        GIVEN_3_WHEN_numbers_to_words_THEN_returns_three: (3, "three"),
        GIVEN_4_WHEN_numbers_to_words_THEN_returns_four: (4, "four"),
        GIVEN_5_WHEN_numbers_to_words_THEN_returns_five: (5, "five"),
        GIVEN_6_WHEN_numbers_to_words_THEN_returns_six: (6, "six"),
        GIVEN_7_WHEN_numbers_to_words_THEN_returns_seven: (7, "seven"),
        GIVEN_8_WHEN_numbers_to_words_THEN_returns_eight: (8, "eight"),
        GIVEN_9_WHEN_numbers_to_words_THEN_returns_nine: (9, "nine"),
        GIVEN_10_WHEN_numbers_to_words_THEN_returns_ten: (10, "ten"),
        GIVEN_11_WHEN_numbers_to_words_THEN_returns_eleven: (11, "eleven"),
        GIVEN_12_WHEN_numbers_to_words_THEN_returns_twelve: (12, "twelve"),
        GIVEN_13_WHEN_numbers_to_words_THEN_returns_thirteen: (13, "thirteen"),
        GIVEN_14_WHEN_numbers_to_words_THEN_returns_fourteen: (14, "fourteen"),
        GIVEN_15_WHEN_numbers_to_words_THEN_returns_fifteen: (15, "fifteen"),
        GIVEN_16_WHEN_numbers_to_words_THEN_returns_sixteen: (16, "sixteen"),
        GIVEN_17_WHEN_numbers_to_words_THEN_returns_seventeen: (17, "seventeen"),
        GIVEN_18_WHEN_numbers_to_words_THEN_returns_eighteen: (18, "eighteen"),
        GIVEN_19_WHEN_numbers_to_words_THEN_returns_nineteen: (19, "nineteen"),
        GIVEN_20_WHEN_numbers_to_words_THEN_returns_twenty: (20, "twenty"),
    }

    macro_rules! number_as_word_is_one_word {

        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                let (entry, expected) = $value;
                    let word = number_as_word_is_one_word(entry);
                    assert_eq!(word, expected);
                }
            )*
         }
    }

    number_as_word_is_one_word! {
        GIVEN_number_1_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (1, true),
        GIVEN_number_2_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (2, true),
        GIVEN_number_3_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (3, true),
        GIVEN_number_4_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (4, true),
        GIVEN_number_5_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (5, true),
        GIVEN_number_6_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (6, true),
        GIVEN_number_7_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (7, true),
        GIVEN_number_8_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (8, true),
        GIVEN_number_9_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (9, true),
        GIVEN_number_10_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (10, true),
        GIVEN_number_11_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (11, true),
        GIVEN_number_12_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (12, true),
        GIVEN_number_13_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (13, true),
        GIVEN_number_14_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (14, true),
        GIVEN_number_15_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (15, true),
        GIVEN_number_16_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (16, true),
        GIVEN_number_17_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (17, true),
        GIVEN_number_18_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (18, true),
        GIVEN_number_19_less_than_20_WHEN_number_as_word_is_one_word_THEN_returns_true: (19, true),
        GIVEN_number_110_over_100_WHEN_number_as_word_is_one_word_THEN_returns_false: (110, false),
        GIVEN_number_20_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (20, true),
        GIVEN_number_30_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (30, true),
        GIVEN_number_40_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (40, true),
        GIVEN_number_50_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (50, true),
        GIVEN_number_60_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (60, true),
        GIVEN_number_70_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (70, true),
        GIVEN_number_80_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (80, true),
        GIVEN_number_90_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (90, true),
        GIVEN_number_100_divisable_by_10_WHEN_number_as_word_is_one_word_THEN_returns_true: (100, true),
        OTHERWISE_CASES_number_37_WHEN_number_as_word_is_one_word_THEN_returns_false: (37, false),
        OTHERWISE_CASES_number_153_WHEN_number_as_word_is_one_word_THEN_returns_false: (153, false),
    }
}
