use std::{fs, path::Path};

/// Exact context supplied to an independent oracle. A verdict is only promotable
/// when it explicitly verifies this claim and this response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRequest {
    pub task_id: String,
    pub claim: String,
    pub response: String,
    pub artifacts: Vec<String>,
    pub criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub passed: bool,
    pub oracle: String,
    pub artifact: String,
    pub verified_claim: Option<String>,
    pub summary: String,
}

impl Verdict {
    pub fn supports(&self, request: &VerificationRequest) -> bool {
        self.passed && self.verified_claim.as_deref() == Some(request.claim.as_str())
    }
}

pub trait Verifier {
    fn verify(&self, request: &VerificationRequest) -> Verdict;
}

pub struct StaticVerifier(pub Verdict);
impl Verifier for StaticVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        self.0.clone()
    }
}

/// Availability check only. It can diagnose a missing artifact, but cannot
/// establish that a model response proves the user's claim.
pub struct FileExistsVerifier {
    pub path: String,
}
impl Verifier for FileExistsVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        let passed = Path::new(&self.path).is_file();
        Verdict {
            passed,
            oracle: "file_exists".into(),
            artifact: self.path.clone(),
            verified_claim: None,
            summary: "artifact availability only; not claim verification".into(),
        }
    }
}

/// Exact-content oracle for tasks whose success criterion is the full expected
/// response. This verifier deliberately refuses a matching file when the LLM
/// response itself differs.
pub struct ExactFileVerifier {
    pub path: String,
    pub expected: String,
}

/// A deliberately narrow oracle for explicit arithmetic requests.  It is not
/// a general fact checker: callers must opt in with the `numeric-answer`
/// criterion, and the generated response must contain the independently
/// computed value.
pub struct CalculatorVerifier {
    pub expression: String,
}

impl Verifier for CalculatorVerifier {
    fn verify(&self, request: &VerificationRequest) -> Verdict {
        if !request
            .criteria
            .iter()
            .any(|criterion| criterion == "numeric-answer")
        {
            return Verdict {
                passed: false,
                oracle: "calculator".into(),
                artifact: self.expression.clone(),
                verified_claim: None,
                summary:
                    "calculator refused: the request did not declare a numeric-answer criterion"
                        .into(),
            };
        }
        let value = match evaluate_expression(&self.expression) {
            Ok(value) => value,
            Err(error) => {
                return Verdict {
                    passed: false,
                    oracle: "calculator".into(),
                    artifact: self.expression.clone(),
                    verified_claim: None,
                    summary: format!("calculator refused: {error}"),
                };
            }
        };
        let expected = format_number(value);
        let passed = contains_numeric_answer(&request.response, value);
        Verdict {
            passed,
            oracle: "calculator".into(),
            artifact: format!("{} = {}", self.expression, expected),
            verified_claim: passed.then(|| request.claim.clone()),
            summary: if passed {
                format!("independent calculation confirmed the numeric answer: {expected}")
            } else {
                format!(
                    "independent calculation is {expected}, but that value was not found in the response"
                )
            },
        }
    }
}

/// Evaluates `+`, `-`, `*`, `/` and parentheses without delegating arithmetic
/// to the LLM.  This parser is intentionally small and has no variables,
/// functions, file access, or command execution.
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    struct Parser<'a> {
        input: &'a [u8],
        position: usize,
    }
    impl<'a> Parser<'a> {
        fn skip_spaces(&mut self) {
            while self
                .input
                .get(self.position)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.position += 1;
            }
        }
        fn consume(&mut self, token: u8) -> bool {
            self.skip_spaces();
            if self.input.get(self.position) == Some(&token) {
                self.position += 1;
                true
            } else {
                false
            }
        }
        fn number(&mut self) -> Result<f64, String> {
            self.skip_spaces();
            let start = self.position;
            while self
                .input
                .get(self.position)
                .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'.' || *byte == b',')
            {
                self.position += 1;
            }
            if start == self.position {
                return Err("number expected".into());
            }
            std::str::from_utf8(&self.input[start..self.position])
                .map_err(|_| "expression is not UTF-8".to_string())?
                .replace(',', ".")
                .parse::<f64>()
                .map_err(|_| "invalid number".into())
        }
        fn factor(&mut self) -> Result<f64, String> {
            self.skip_spaces();
            if self.consume(b'+') {
                return self.factor();
            }
            if self.consume(b'-') {
                return Ok(-self.factor()?);
            }
            if self.consume(b'(') {
                let value = self.sum()?;
                if !self.consume(b')') {
                    return Err("missing closing parenthesis".into());
                }
                return Ok(value);
            }
            self.number()
        }
        fn product(&mut self) -> Result<f64, String> {
            let mut value = self.factor()?;
            loop {
                if self.consume(b'*') {
                    value *= self.factor()?;
                } else if self.consume(b'/') {
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("division by zero".into());
                    }
                    value /= divisor;
                } else {
                    return Ok(value);
                }
            }
        }
        fn sum(&mut self) -> Result<f64, String> {
            let mut value = self.product()?;
            loop {
                if self.consume(b'+') {
                    value += self.product()?;
                } else if self.consume(b'-') {
                    value -= self.product()?;
                } else {
                    return Ok(value);
                }
            }
        }
    }
    let mut parser = Parser {
        input: expression.as_bytes(),
        position: 0,
    };
    let value = parser.sum()?;
    parser.skip_spaces();
    if parser.position != parser.input.len() {
        return Err("unexpected character in expression".into());
    }
    if value.is_finite() {
        Ok(value)
    } else {
        Err("non-finite result".into())
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn contains_numeric_answer(response: &str, expected: f64) -> bool {
    response
        .split(|character: char| {
            !(character.is_ascii_digit()
                || character == '.'
                || character == ','
                || character == '-')
        })
        .filter(|token| !token.is_empty() && *token != "-")
        .filter_map(|token| token.replace(',', ".").parse::<f64>().ok())
        .any(|value| (value - expected).abs() <= 1e-9_f64.max(expected.abs() * 1e-9))
}

#[cfg(test)]
mod calculator_tests {
    use super::*;

    fn request(response: &str) -> VerificationRequest {
        VerificationRequest {
            task_id: "calculation".into(),
            claim: "calculate the expression".into(),
            response: response.into(),
            artifacts: Vec::new(),
            criteria: vec!["numeric-answer".into()],
        }
    }

    #[test]
    fn calculator_handles_precedence_and_parentheses() {
        assert_eq!(evaluate_expression("(2 + 3) * 4 / 2"), Ok(10.0));
    }

    #[test]
    fn calculator_verifies_only_the_matching_generated_value() {
        let verifier = CalculatorVerifier {
            expression: "7 * 6".into(),
        };
        assert!(verifier.verify(&request("Le résultat est 42.")).passed);
        assert!(!verifier.verify(&request("Le résultat est 41.")).passed);
    }

    #[test]
    fn calculator_rejects_unsafe_or_invalid_syntax() {
        assert!(evaluate_expression("2 + command()").is_err());
        assert!(evaluate_expression("1 / 0").is_err());
    }
}
impl Verifier for ExactFileVerifier {
    fn verify(&self, request: &VerificationRequest) -> Verdict {
        let content = fs::read_to_string(&self.path).unwrap_or_default();
        let passed = content == self.expected && request.response.trim() == self.expected.trim();
        Verdict {
            passed,
            oracle: "file_exact_response".into(),
            artifact: self.path.clone(),
            verified_claim: passed.then(|| request.claim.clone()),
            summary: if passed {
                "artifact and response match the declared criterion".into()
            } else {
                "artifact or response does not match the declared criterion".into()
            },
        }
    }
}
