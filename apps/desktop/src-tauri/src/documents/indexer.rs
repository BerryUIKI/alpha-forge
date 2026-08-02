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

/// Ranks local research chunks with a small, explainable investment vocabulary.
/// Exact terms remain dominant; related terms contribute a lower score.
pub fn semantic_rank_chunks(chunks: &[TextChunk], query: &str, limit: usize) -> Vec<SearchMatch> {
    let query_terms = tokenize(query);
    if query_terms.is_empty() || limit == 0 { return Vec::new(); }
    let mut matches: Vec<SearchMatch> = chunks.iter().filter_map(|chunk| {
        let terms = tokenize(&chunk.content);
        let score = query_terms.iter().map(|query_term| score_term(&terms, query_term)).sum();
        (score > 0).then(|| SearchMatch { ordinal: chunk.ordinal, content: chunk.content.clone(), score })
    }).collect();
    matches.sort_by(|left, right| right.score.cmp(&left.score).then(left.ordinal.cmp(&right.ordinal)));
    matches.truncate(limit);
    matches
}

fn tokenize(input: &str) -> Vec<String> {
    input.split(|character: char| !character.is_alphanumeric()).filter(|term| !term.is_empty()).map(|term| term.to_ascii_lowercase()).collect()
}

fn score_term(chunk_terms: &[String], query_term: &str) -> usize {
    let exact = chunk_terms.iter().filter(|term| term.as_str() == query_term).count();
    let stem_matches = chunk_terms.iter().filter(|term| stem(term) == stem(query_term) && term.as_str() != query_term).count();
    let related = related_terms(query_term).iter().map(|related| chunk_terms.iter().filter(|term| term.as_str() == *related).count()).sum::<usize>();
    exact * 5 + stem_matches * 3 + related * 2
}

fn stem(term: &str) -> &str {
    term.strip_suffix("ing").or_else(|| term.strip_suffix("ed")).or_else(|| term.strip_suffix("es")).or_else(|| term.strip_suffix('s')).unwrap_or(term)
}

fn related_terms(term: &str) -> &'static [&'static str] {
    match term {
        "revenue" | "sales" | "turnover" => &["revenue", "sales", "turnover"],
        "earnings" | "profit" | "income" | "eps" => &["earnings", "profit", "income", "eps"],
        "growth" | "expansion" | "increase" => &["growth", "expansion", "increase"],
        "risk" | "volatility" | "uncertainty" | "downside" => &["risk", "volatility", "uncertainty", "downside"],
        "margin" | "profitability" => &["margin", "profitability"],
        "debt" | "leverage" | "liability" => &["debt", "leverage", "liability"],
        "competition" | "competitor" | "rival" => &["competition", "competitor", "rival"],
        "market" | "industry" | "sector" => &["market", "industry", "sector"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_matching_chunks_case_insensitively() {
        let chunks = vec![TextChunk { ordinal: 0, content: "AI data center demand".into() }, TextChunk { ordinal: 1, content: "AI AI infrastructure".into() }];
        assert_eq!(rank_chunks(&chunks, "ai", 1), vec![SearchMatch { ordinal: 1, content: "AI AI infrastructure".into(), score: 2 }]);
    }

    #[test]
    fn ranks_related_investment_terms_below_exact_matches() {
        let chunks = vec![TextChunk { ordinal: 0, content: "Revenue increased this quarter".into() }, TextChunk { ordinal: 1, content: "Sales increased this quarter".into() }];
        let matches = semantic_rank_chunks(&chunks, "revenue", 2);
        assert_eq!(matches[0].ordinal, 0);
        assert_eq!(matches[1].ordinal, 1);
    }
}
