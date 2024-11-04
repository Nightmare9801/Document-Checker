use std::fs;

/// The `count_word` function in Rust reads a file and counts the number of words that consist only of
/// alphanumeric characters.
/// 
/// Arguments:
/// 
/// * `path`: The `path` parameter in the `count_word` function is a `String` type that represents the
/// file path from which you want to read the text data to count the words.
/// 
/// Returns:
/// 
/// The `count_word` function returns a `u128` value, which represents the count of words in the file
/// located at the specified `path` after filtering out non-alphanumeric characters.
pub fn count_word(path: String) -> u128 {
    let data: String = fs::read_to_string(path).expect("Unable to read file");

    let mut count: u128 = 0;

    for word in data.split_whitespace() {
        if word.chars().all(|c| c.is_alphanumeric()) {
            count += 1;
        }
    }

    return count;
}

/// This Rust function `count_sentences` reads a file, splits the content into sentences based on
/// periods, and counts the number of sentences in the file.
/// 
/// Arguments:
/// 
/// * `path`: The `path` parameter in the `count_sentences` function is a `String` type that represents
/// the file path from which the function will read the text data to count the number of sentences.
/// 
/// Returns:
/// 
/// The function `count_sentences` returns a `u128` value, which represents the count of sentences in
/// the text file located at the specified `path`.
pub fn count_sentences(path: String) -> u128 {
    let data: String = fs::read_to_string(path).expect("Unable to read file");
    
    let mut count: u128 = 0;

    for sentence in data.split('.') {
        if sentence.chars().next().unwrap_or(' ')!= '.' {
            count += 1;
        }
    }

    return count;
}

/// The `count_syllables` function in Rust reads a file, counts the number of syllables in each word,
/// and returns the total count.
/// 
/// Arguments:
/// 
/// * `path`: The `path` parameter in the `count_syllables` function is a `String` type that represents
/// the file path to the text file from which you want to count the syllables.
/// 
/// Returns:
/// 
/// The `count_syllables` function returns the total count of syllables found in the words read from the
/// file specified by the `path` parameter.
pub fn count_syllables(path: String) -> u128 {
    let data: String = fs::read_to_string(path).expect("Unable to read file");

    let mut count: u128 = 0;

    for word in data.split_whitespace() {
        if word.chars().all(|c| c.is_alphanumeric()) {
            count += syllables(word) as u128;
        }
    }

    return count;
}

/// The function `syllables` in Rust calculates the number of syllables in a given word based on
/// specific rules.
/// 
/// Arguments:
/// 
/// * `word`: The code you provided is a function that calculates the number of syllables in a given
/// word based on certain rules. It checks for vowels, silent 'e', and special cases like words ending
/// in "le".
/// 
/// Returns:
/// 
/// The function `syllables` returns the number of syllables in the input word as a `u32` unsigned
/// integer.
fn syllables(word: &str) -> u32 {
    let mut syllable_count: u32 = 0;
    let vowels: &str = "aeiouy";

    if word.chars().next().unwrap().is_lowercase() && vowels.contains(word.chars().next().unwrap()) {
        syllable_count += 1;
    }

    for (index, char) in word.chars().enumerate().skip(1) {
        if vowels.contains(char) && !vowels.contains(word.chars().nth(index - 1).unwrap()) {
            syllable_count += 1;
        }
    }

    if word.ends_with('e') {
        syllable_count -= 1;
    }

    if word.ends_with("le") && word.len() > 2 && !vowels.contains(word.chars().nth(word.len() - 3).unwrap()) {
        syllable_count += 1;
    }

    if syllable_count == 0 {
        syllable_count += 1;
    }

    syllable_count
}

/// This Rust function calculates the Flesch Reading Ease score based on word count, sentence count, and
/// syllable count in a given text file.
/// 
/// Arguments:
/// 
/// * `path`: The `path` parameter in your function `get_flesch_reading_test` is a `String` type that
/// represents the file path to the text file you want to analyze for the Flesch Reading Ease test. This
/// function calculates the Flesch Reading Ease score based on the word count,
/// 
/// Returns:
/// 
/// The function `get_flesch_reading_test` returns a `f64` value, which represents the Flesch Reading
/// Ease score calculated based on the word count, sentence count, and syllable count of the text
/// provided in the file specified by the `path` parameter.
pub fn get_flesch_reading_test(path: String) -> f64 {
    let word_count: u128 = count_word(path.clone());
    let sentence_count: u128 = count_sentences(path.clone());
    let syllable_count: u128 = count_syllables(path.clone());

    let flesch_reading_ease: f64 = 206.835 - (1.015 * divide_u128_to_f64(word_count, sentence_count)) - (84.6 * divide_u128_to_f64(syllable_count, word_count));

    return flesch_reading_ease;
}

/// The function `divide_u128_to_f64` takes two `u128` integers, converts them to `f64`, and returns the
/// result of dividing the first number by the second.
/// 
/// Arguments:
/// 
/// * `y`: The `y` parameter is a 128-bit unsigned integer (u128) that represents the numerator in the
/// division operation.
/// * `z`: The parameter `z` in the function `divide_u128_to_f64` is a 128-bit unsigned integer
/// (`u128`).
/// 
/// Returns:
/// 
/// The function `divide_u128_to_f64` takes two `u128` integers as input, converts them to `f64`
/// floating-point numbers, divides the first number by the second number, and returns the result as an
/// `f64`.
fn divide_u128_to_f64(y: u128, z: u128) -> f64{
    y as f64 / z as f64
}

