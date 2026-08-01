use super::chunker::TextChunk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    pub ordinal: usize,
    pub content: String,
    pub score: usize,
}

pub fn rank_chunks(chunks: &[TextChunk], query: &str, limit: usize) -> Vec<SearchMatch> {
    let terms: Vec<String> = query.split_whitespace().map(|term| term.to_ascii_lowercase()).filter(|term| !term.is_empty()).collect();
    if terms.is_empty() || limit == 0 { return Vec::new(); }
    let mut matches: Vec<SearchMatch> = chunks.iter().filter_map(|chunk| {
        let content = chunk.content.to_ascii_lowercase();
        let score = terms.iter().map(|term| content.matches(term).count()).sum();
        (score > 0).then(|| SearchMatch { ordinal: chunk.ordinal, content: chunk.content.clone(), score })
    }).collect();
    matches.sort_by(|left, right| right.score.cmp(&left.score).then(left.ordinal.cmp(&right.ordinal)));
    matches.truncate(limit);
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_matching_chunks_case_insensitively() {
        let chunks = vec![TextChunk { ordinal: 0, content: "AI data center demand".into() }, TextChunk { ordinal: 1, content: "AI AI infrastructure".into() }];
        assert_eq!(rank_chunks(&chunks, "ai", 1), vec![SearchMatch { ordinal: 1, content: "AI AI infrastructure".into(), score: 2 }]);
    }
}
