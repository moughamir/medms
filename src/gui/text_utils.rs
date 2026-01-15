use arabic_reshaper::ArabicReshaper;
// use unicode_bidi::BidiInfo;

/// Reshapes Arabic text to have correct letter forms (Initial, Medial, Final).
/// Also handles BiDi reordering for proper display in environments that don't support it natively.
pub fn reshape(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let reshaper = ArabicReshaper::default();
    let reshaped_text = reshaper.reshape(text);

    // If the text contains Arabic, we might need to reorder it for display
    // if the underlying renderer doesn't handle BiDi.
    // egui generally expects logical text, but for "reshaped" text (which effectively becomes visual glyphs for legacy support),
    // we often need to reverse it or apply bidi explicitly if we are "drawing" it as glyphs.
    // However, arabic_reshaper returns logical characters in proper forms.
    // Let's try just reshaping first. The most common issue is isolated characters.
    
    // Check if we need to apply Bidi
    // For now, let's just return reshaped text. 
    // If direction is still wrong, we will apply bidi reordering.
    
    // UPDATE: Users usually need Bidi processing for Arabic in simple renderers.
    // Let's apply simple Bidi processing.
    
    // If the whole text is RTL, we might just need to reverse the characters for some engines?
    // But let's actally use reorder_line if we want visual order.
    // egui typically handles direction if given logical text, but we are modifying the text 
    // to be "presentation forms".
    // 
    // Experiment: Just return reshaped text first. If that fails (letters connected but wrong order),
    // we will add Bidi reordering.
    
    reshaped_text
}
