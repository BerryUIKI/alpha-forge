#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextChunk {
    pub ordinal: usize,
    pub content: String,
}

pub fn chunk_text(text: &str, max_chars: usize) -> Vec<TextChunk> {
    if max_chars == 0 { return Vec::new(); }
    let words = text.split_whitespace();
    let mut chunks = Vec::new();
    let mut current = String::new();

    for word in words {
        let separator = usize::from(!current.is_empty());
        if !current.is_empty() && current.len() + separator + word.len() > max_chars {
            chunks.push(TextChunk { ordinal: chunks.len(), content: current });
            current = String::new();
        }
        if !current.is_empty() { current.push(' '); }
        current.push_str(word);
    }
    if !current.is_empty() { chunks.push(TextChunk { ordinal: chunks.len(), content: current }); }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_words_intact_and_assigns_ordinals() {
        assert_eq!(chunk_text("alpha beta gamma", 10), vec![
            TextChunk { ordinal: 0, content: "alpha beta".into() },
            TextChunk { ordinal: 1, content: "gamma".into() },
        ]);
    }
}
