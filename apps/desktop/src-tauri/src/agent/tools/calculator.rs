// Arithmetic calculation tool.
//
// A small recursive-descent parser keeps this dependency-free and avoids
// handing model-generated input to a general-purpose evaluator.

use serde::Deserialize;
use serde_json::{json, Value};

use crate::agent::tools::AgentTool;
use crate::error::AppError;

const MAX_EXPRESSION_CHARS: usize = 200;

/// Evaluates basic arithmetic with `+`, `-`, `*`, `/`, and parentheses.
#[derive(Debug, Clone, Default)]
pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct CalculateArgs {
    expression: String,
}

#[async_trait::async_trait]
impl AgentTool for CalculatorTool {
    fn name(&self) -> &'static str {
        "calculate"
    }

    fn description(&self) -> &'static str {
        "Evaluate a basic arithmetic expression with +, -, *, /, and parentheses."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Arithmetic expression, for example (12.5 * 3) / 2"
                }
            },
            "required": ["expression"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: CalculateArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid calculate arguments: {error}"))
        })?;

        let result = evaluate(&args.expression)?;

        Ok(json!({ "result": result }))
    }
}

/// Evaluates an arithmetic expression.
pub fn evaluate(expression: &str) -> Result<f64, AppError> {
    if expression.trim().is_empty() {
        return Err(AppError::Validation(
            "expression cannot be empty".to_string(),
        ));
    }
    if expression.chars().count() > MAX_EXPRESSION_CHARS {
        return Err(AppError::Validation("expression is too long".to_string()));
    }

    let mut parser = Parser::new(expression);
    let value = parser.parse_expression()?;

    if !parser.at_end() {
        return Err(AppError::Validation(format!(
            "unexpected character at position {}",
            parser.position
        )));
    }
    if !value.is_finite() {
        return Err(AppError::Validation(
            "result is not a finite number".to_string(),
        ));
    }

    Ok(value)
}

struct Parser {
    chars: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.position += 1;
        }
    }

    fn at_end(&mut self) -> bool {
        self.skip_whitespace();
        self.position >= self.chars.len()
    }

    fn parse_expression(&mut self) -> Result<f64, AppError> {
        let mut value = self.parse_term()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    self.position += 1;
                    value += self.parse_term()?;
                }
                Some('-') => {
                    self.position += 1;
                    value -= self.parse_term()?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_term(&mut self) -> Result<f64, AppError> {
        let mut value = self.parse_factor()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    self.position += 1;
                    value *= self.parse_factor()?;
                }
                Some('/') => {
                    self.position += 1;
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err(AppError::Validation("division by zero".to_string()));
                    }
                    value /= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_factor(&mut self) -> Result<f64, AppError> {
        self.skip_whitespace();

        match self.peek() {
            Some('-') => {
                self.position += 1;
                Ok(-self.parse_factor()?)
            }
            Some('+') => {
                self.position += 1;
                self.parse_factor()
            }
            Some('(') => {
                self.position += 1;
                let value = self.parse_expression()?;
                self.skip_whitespace();
                if self.peek() != Some(')') {
                    return Err(AppError::Validation(
                        "missing closing parenthesis".to_string(),
                    ));
                }
                self.position += 1;
                Ok(value)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) => Err(AppError::Validation(format!(
                "unexpected character '{}'",
                c
            ))),
            None => Err(AppError::Validation(
                "unexpected end of expression".to_string(),
            )),
        }
    }

    fn parse_number(&mut self) -> Result<f64, AppError> {
        let start = self.position;
        let mut seen_dot = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.position += 1;
            } else if c == '.' && !seen_dot {
                seen_dot = true;
                self.position += 1;
            } else {
                break;
            }
        }

        let text: String = self.chars[start..self.position].iter().collect();
        text.parse::<f64>()
            .map_err(|_| AppError::Validation(format!("invalid number '{}'", text)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn test_evaluates_precedence() {
        assert_approx(evaluate("1 + 2 * 3").expect("should evaluate"), 7.0);
        assert_approx(evaluate("10 - 4 / 2").expect("should evaluate"), 8.0);
    }

    #[test]
    fn test_evaluates_parentheses_and_unary_minus() {
        assert_approx(evaluate("(1 + 2) * 3").expect("should evaluate"), 9.0);
        assert_approx(evaluate("-4 + 10").expect("should evaluate"), 6.0);
        assert_approx(evaluate("2 * -(3 + 1)").expect("should evaluate"), -8.0);
    }

    #[test]
    fn test_evaluates_decimals() {
        assert_approx(evaluate("1.5 * 4").expect("should evaluate"), 6.0);
    }

    #[test]
    fn test_rejects_division_by_zero() {
        assert!(matches!(evaluate("1 / 0"), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_rejects_invalid_expressions() {
        assert!(matches!(evaluate("2 +"), Err(AppError::Validation(_))));
        assert!(matches!(evaluate("abc"), Err(AppError::Validation(_))));
        assert!(matches!(evaluate("(1 + 2"), Err(AppError::Validation(_))));
        assert!(matches!(evaluate("1 2"), Err(AppError::Validation(_))));
        assert!(matches!(evaluate(""), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_rejects_long_expressions() {
        let expression = "1+".repeat(MAX_EXPRESSION_CHARS);

        assert!(matches!(
            evaluate(&expression),
            Err(AppError::Validation(_))
        ));
    }

    #[tokio::test]
    async fn test_tool_executes_expression() {
        let result = CalculatorTool::new()
            .execute(json!({"expression": "2 * (3 + 4)"}))
            .await
            .expect("tool should execute");

        assert_eq!(result["result"], 14.0);
    }
}
