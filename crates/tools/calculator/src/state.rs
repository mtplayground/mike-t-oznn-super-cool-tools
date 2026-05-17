#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn apply(self, lhs: f64, rhs: f64) -> Result<f64, CalculationError> {
        match self {
            Self::Add => Ok(lhs + rhs),
            Self::Subtract => Ok(lhs - rhs),
            Self::Multiply => Ok(lhs * rhs),
            Self::Divide => {
                if rhs == 0.0 {
                    Err(CalculationError::DivisionByZero)
                } else {
                    Ok(lhs / rhs)
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Digit(u8),
    DecimalPoint,
    Operator(Operator),
    Equals,
    Clear,
    Backspace,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalculatorState {
    current: String,
    stored: Option<f64>,
    pending_operator: Option<Operator>,
    replace_current: bool,
    error: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            current: "0".to_owned(),
            stored: None,
            pending_operator: None,
            replace_current: false,
            error: false,
        }
    }
}

impl CalculatorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self) -> &str {
        &self.current
    }

    pub fn clear_label(&self) -> &'static str {
        if self.is_all_clear_state() {
            "AC"
        } else {
            "C"
        }
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Digit(digit) if digit <= 9 => self.apply_digit(digit),
            Action::Digit(_) => {}
            Action::DecimalPoint => self.apply_decimal_point(),
            Action::Operator(operator) => self.apply_operator(operator),
            Action::Equals => self.apply_equals(),
            Action::Clear => self.apply_clear(),
            Action::Backspace => self.apply_backspace(),
        }
    }

    fn apply_digit(&mut self, digit: u8) {
        if self.error {
            self.reset_all();
        }

        let digit = char::from(b'0' + digit);
        if self.replace_current || self.current == "0" {
            self.current = digit.to_string();
            self.replace_current = false;
            return;
        }

        self.current.push(digit);
    }

    fn apply_decimal_point(&mut self) {
        if self.error {
            self.reset_all();
        }

        if self.replace_current {
            self.current = "0.".to_owned();
            self.replace_current = false;
            return;
        }

        if !self.current.contains('.') {
            self.current.push('.');
        }
    }

    fn apply_operator(&mut self, operator: Operator) {
        if self.error {
            return;
        }

        let current_value = match self.current_value() {
            Ok(value) => value,
            Err(_) => {
                self.set_error();
                return;
            }
        };

        match self.pending_operator {
            Some(pending) if !self.replace_current => {
                let lhs = self.stored.unwrap_or(current_value);
                match pending.apply(lhs, current_value) {
                    Ok(result) => {
                        self.current = format_number(result);
                        self.stored = Some(result);
                    }
                    Err(_) => {
                        self.set_error();
                        return;
                    }
                }
            }
            Some(_) => {}
            None => {
                self.stored = Some(current_value);
            }
        }

        self.pending_operator = Some(operator);
        self.replace_current = true;
    }

    fn apply_equals(&mut self) {
        if self.error {
            return;
        }

        let Some(operator) = self.pending_operator else {
            return;
        };

        let current_value = match self.current_value() {
            Ok(value) => value,
            Err(_) => {
                self.set_error();
                return;
            }
        };

        let lhs = self.stored.unwrap_or(current_value);
        match operator.apply(lhs, current_value) {
            Ok(result) => {
                self.current = format_number(result);
                self.stored = Some(result);
                self.pending_operator = None;
                self.replace_current = true;
            }
            Err(_) => self.set_error(),
        }
    }

    fn apply_clear(&mut self) {
        if self.is_all_clear_state() || self.error {
            self.reset_all();
            return;
        }

        self.current = "0".to_owned();
        self.replace_current = false;
    }

    fn apply_backspace(&mut self) {
        if self.error {
            self.reset_all();
            return;
        }

        if self.replace_current {
            self.current = "0".to_owned();
            self.replace_current = false;
            return;
        }

        if self.current.len() <= 1 {
            self.current = "0".to_owned();
            return;
        }

        self.current.pop();
        if self.current == "-" || self.current.is_empty() {
            self.current = "0".to_owned();
        }
    }

    fn current_value(&self) -> Result<f64, CalculationError> {
        self.current
            .parse::<f64>()
            .map_err(|_| CalculationError::InvalidNumber)
    }

    fn is_all_clear_state(&self) -> bool {
        !self.error
            && self.current == "0"
            && self.stored.is_none()
            && self.pending_operator.is_none()
            && !self.replace_current
    }

    fn reset_all(&mut self) {
        *self = Self::default();
    }

    fn set_error(&mut self) {
        self.current = "Error".to_owned();
        self.stored = None;
        self.pending_operator = None;
        self.replace_current = true;
        self.error = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalculationError {
    InvalidNumber,
    DivisionByZero,
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_owned();
    }

    let mut formatted = format!("{value:.10}");
    while formatted.contains('.') && formatted.ends_with('0') {
        formatted.pop();
    }
    if formatted.ends_with('.') {
        formatted.pop();
    }
    if formatted == "-0" {
        "0".to_owned()
    } else {
        formatted
    }
}

pub fn action_from_key(key: &str) -> Option<Action> {
    match key {
        "0" => Some(Action::Digit(0)),
        "1" => Some(Action::Digit(1)),
        "2" => Some(Action::Digit(2)),
        "3" => Some(Action::Digit(3)),
        "4" => Some(Action::Digit(4)),
        "5" => Some(Action::Digit(5)),
        "6" => Some(Action::Digit(6)),
        "7" => Some(Action::Digit(7)),
        "8" => Some(Action::Digit(8)),
        "9" => Some(Action::Digit(9)),
        "." | "," => Some(Action::DecimalPoint),
        "+" => Some(Action::Operator(Operator::Add)),
        "-" => Some(Action::Operator(Operator::Subtract)),
        "*" | "x" | "X" => Some(Action::Operator(Operator::Multiply)),
        "/" => Some(Action::Operator(Operator::Divide)),
        "Enter" | "=" => Some(Action::Equals),
        "Escape" | "Delete" => Some(Action::Clear),
        "Backspace" => Some(Action::Backspace),
        _ => None,
    }
}

pub fn action_from_token(token: &str) -> Option<Action> {
    match token {
        "digit-0" => Some(Action::Digit(0)),
        "digit-1" => Some(Action::Digit(1)),
        "digit-2" => Some(Action::Digit(2)),
        "digit-3" => Some(Action::Digit(3)),
        "digit-4" => Some(Action::Digit(4)),
        "digit-5" => Some(Action::Digit(5)),
        "digit-6" => Some(Action::Digit(6)),
        "digit-7" => Some(Action::Digit(7)),
        "digit-8" => Some(Action::Digit(8)),
        "digit-9" => Some(Action::Digit(9)),
        "decimal" => Some(Action::DecimalPoint),
        "add" => Some(Action::Operator(Operator::Add)),
        "subtract" => Some(Action::Operator(Operator::Subtract)),
        "multiply" => Some(Action::Operator(Operator::Multiply)),
        "divide" => Some(Action::Operator(Operator::Divide)),
        "equals" => Some(Action::Equals),
        "clear" => Some(Action::Clear),
        "backspace" => Some(Action::Backspace),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{action_from_key, Action, CalculatorState, Operator};

    #[test]
    fn performs_basic_addition() {
        let mut state = CalculatorState::new();

        state.apply(Action::Digit(1));
        state.apply(Action::Digit(2));
        state.apply(Action::Operator(Operator::Add));
        state.apply(Action::Digit(3));
        state.apply(Action::Equals);

        assert_eq!(state.display(), "15");
    }

    #[test]
    fn formats_decimal_results_without_trailing_zeroes() {
        let mut state = CalculatorState::new();

        state.apply(Action::Digit(7));
        state.apply(Action::Operator(Operator::Divide));
        state.apply(Action::Digit(2));
        state.apply(Action::Equals);

        assert_eq!(state.display(), "3.5");
    }

    #[test]
    fn toggles_clear_label_between_ac_and_c() {
        let mut state = CalculatorState::new();

        assert_eq!(state.clear_label(), "AC");
        state.apply(Action::Digit(9));
        assert_eq!(state.clear_label(), "C");
        state.apply(Action::Clear);
        assert_eq!(state.display(), "0");
        assert_eq!(state.clear_label(), "AC");
    }

    #[test]
    fn supports_backspace() {
        let mut state = CalculatorState::new();

        state.apply(Action::Digit(4));
        state.apply(Action::Digit(2));
        state.apply(Action::Backspace);

        assert_eq!(state.display(), "4");
    }

    #[test]
    fn maps_keyboard_shortcuts() {
        assert_eq!(action_from_key("*"), Some(Action::Operator(Operator::Multiply)));
        assert_eq!(action_from_key("Backspace"), Some(Action::Backspace));
        assert_eq!(action_from_key("Escape"), Some(Action::Clear));
    }
}
