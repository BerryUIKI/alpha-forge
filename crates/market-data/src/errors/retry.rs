/// Classification for determining retry behavior after a provider error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    /// Never retry — the error is terminal.
    Never,
    /// Failover with circuit breaker penalty — exponential backoff.
    FailoverWithPenalty,
    /// Try the next provider in the chain.
    NextProvider,
    /// Provider circuit is open — skip it.
    CircuitOpen,
}
