use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reversed = input.graphemes(true).rev().collect();
    reversed
}
