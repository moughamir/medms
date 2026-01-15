use arabic_reshaper::ArabicReshaper;
use unicode_bidi::BidiInfo;

/// Reshapes Arabic text to have correct letter forms (Initial, Medial, Final).
/// Also handles BiDi reordering for proper display in environments that don't support it natively.
pub fn reshape(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let reshaper = ArabicReshaper::default();
    let reshaped_text = reshaper.reshape(text);

    // Apply BiDi reordering to get visual order
    let bidi_info = BidiInfo::new(&reshaped_text, Some(unicode_bidi::Level::rtl()));
    
    // We want to reorder the whole line to visual order
    if !bidi_info.paragraphs.is_empty() {
        let para = &bidi_info.paragraphs[0];
        let line = para.range.clone();
        let display = bidi_info.reorder_line(para, line);
        display.to_string()
    } else {
        reshaped_text
    }
}
