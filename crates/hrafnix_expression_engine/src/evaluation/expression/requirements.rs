use crate::expression::ast::translator::{Expression, Literal};
use hrafnix_shareable_string::ShareableString;
use std::collections::HashSet;

/// Holds the names of required globals, parameters, variables, and functions that
/// are referenced by an expression but absent from the available context.
pub(crate) struct MissingRequirements {
    /// Missing global variable names (prefixed `g_`).
    pub globals: Vec<ShareableString>,
    /// Missing parameter names (prefixed `p_`).
    pub parameters: Vec<ShareableString>,
    /// Missing variable names (prefixed `v_`).
    pub variables: Vec<ShareableString>,
    /// Missing function names.
    pub functions: Vec<ShareableString>,
}

impl MissingRequirements {
    /// Analyzes `expression` against the provided key sets and returns any missing requirements.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub(crate) fn new(
        expression: &Expression,
        item_keys: &HashSet<ShareableString>,
        function_keys: &HashSet<ShareableString>,
    ) -> Self {
        let mut missing_globals = Vec::new();
        let mut missing_parameters = Vec::new();
        let mut missing_variables = Vec::new();
        let mut missing_functions = Vec::new();

        let requirements = ExpressionRequirements::new(expression);
        let is_available = |name: &ShareableString| {
            item_keys.contains(name)
                || (requirements.indexed.contains(name)
                    && Self::has_map_entry(name.as_str(), item_keys))
        };

        for global in &requirements.globals {
            if !is_available(global) {
                missing_globals.push(global.clone());
            }
        }

        for parameter in &requirements.parameters {
            if !is_available(parameter) {
                missing_parameters.push(parameter.clone());
            }
        }

        for variable in &requirements.variables {
            if !is_available(variable) {
                missing_variables.push(variable.clone());
            }
        }

        for function in requirements.functions {
            if !function_keys.contains(&function) {
                missing_functions.push(function);
            }
        }

        Self {
            globals: missing_globals,
            parameters: missing_parameters,
            variables: missing_variables,
            functions: missing_functions,
        }
    }

    /// Returns whether `item_keys` contains a flattened map entry for `name`
    /// (i.e., a key of the form `name[entry][field]`).
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn has_map_entry(name: &str, item_keys: &HashSet<ShareableString>) -> bool {
        let prefix = format!("{name}[");
        item_keys
            .iter()
            .any(|key| key.as_str().starts_with(prefix.as_str()))
    }

    /// Returns `true` if any required globals, parameters, variables, or functions are missing.
    pub(crate) const fn missing_requirements_exist(&self) -> bool {
        !self.globals.is_empty()
            || !self.parameters.is_empty()
            || !self.variables.is_empty()
            || !self.functions.is_empty()
    }

    /// Returns the names of missing globals.
    pub(crate) const fn globals(&self) -> &[ShareableString] {
        self.globals.as_slice()
    }

    /// Returns `true` if any globals are missing.
    pub(crate) const fn missing_globals(&self) -> bool {
        !self.globals.is_empty()
    }

    /// Returns the names of missing parameters.
    pub(crate) const fn parameters(&self) -> &[ShareableString] {
        self.parameters.as_slice()
    }

    /// Returns `true` if any parameters are missing.
    pub(crate) const fn missing_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Returns the names of missing variables.
    pub(crate) const fn variables(&self) -> &[ShareableString] {
        self.variables.as_slice()
    }

    /// Returns `true` if any variables are missing.
    pub(crate) const fn missing_variables(&self) -> bool {
        !self.variables.is_empty()
    }

    /// Returns the names of missing functions.
    pub(crate) const fn functions(&self) -> &[ShareableString] {
        self.functions.as_slice()
    }

    /// Returns `true` if any functions are missing.
    pub(crate) const fn missing_functions(&self) -> bool {
        !self.functions.is_empty()
    }
}

/// Represents the requirements of an expression in terms of the global variables,
/// parameters, variables, and functions it references. This struct is used to analyze
/// an expression and determine its dependencies, which can be useful for validation,
/// optimization, and ensuring that all necessary resources are available for evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExpressionRequirements {
    /// The global variables referenced by the expression.
    globals: Vec<ShareableString>,
    /// The parameters referenced by the expression.
    parameters: Vec<ShareableString>,
    /// The variables referenced by the expression.
    variables: Vec<ShareableString>,
    /// The functions referenced by the expression.
    functions: Vec<ShareableString>,
    /// Names referenced as the target of an index expression (e.g. `p_map` in
    /// `p_map[entry][field]`), which may be satisfied by flattened map entries.
    indexed: HashSet<ShareableString>,
    /// Names already recorded, used to avoid duplicates.
    seen_names: HashSet<ShareableString>,
    /// Function names already recorded, used to avoid duplicates.
    seen_functions: HashSet<ShareableString>,
}

impl ExpressionRequirements {
    /// Walks `expression` and collects all referenced globals, parameters, variables, and functions.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub(crate) fn new(expression: &Expression) -> Self {
        let mut requirements = Self::default();
        requirements.collect(expression);
        requirements
    }

    /// Classifies a name (e.g. `g_foo`, `p_bar`, `v_baz`) into the appropriate
    /// requirement bucket based on its prefix, recording it only the first time
    /// it is encountered.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn record_name(&mut self, name: &str) {
        let key = ShareableString::from(name);
        if !self.seen_names.insert(key.clone()) {
            return;
        }

        if name.starts_with("g_") {
            self.globals.push(key);
        } else if name.starts_with("p_") {
            self.parameters.push(key);
        } else if name.starts_with("v_") {
            self.variables.push(key);
        }
    }

    /// Recursively walks an `Expression` tree, collecting the globals,
    /// parameters, variables, and function names it references.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn collect(&mut self, expression: &Expression) {
        match expression {
            Expression::Literal(_, Literal::Identifier(name)) => self.record_name(name),
            Expression::Literal(_, _) => {}
            Expression::BinaryOperation { left, right, .. } => {
                self.collect(left);
                self.collect(right);
            }
            Expression::UnaryOperation { operand, .. } => self.collect(operand),
            Expression::FunctionCall {
                name, arguments, ..
            } => {
                let key = ShareableString::from(name.as_str());
                if self.seen_functions.insert(key.clone()) {
                    self.functions.push(key);
                }
                for argument in arguments {
                    self.collect(argument);
                }
            }
            Expression::Index { name, index, .. } => {
                self.record_name(name);
                self.indexed.insert(ShareableString::from(name.as_str()));
                for index_expression in index {
                    // A bare identifier used as an index (e.g., the `col` in `t[0][col]`) may be
                    // a literal field name rather than a variable reference, so it is not treated
                    // as a requirement here.
                    if matches!(
                        index_expression,
                        Expression::Literal(_, Literal::Identifier(_))
                    ) {
                        continue;
                    }
                    self.collect(index_expression);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::ast::ast_helper::string_to_expression;

    fn missing(expression: &str, item_keys: &[&str]) -> MissingRequirements {
        let expression = string_to_expression(&ShareableString::from(expression)).unwrap();
        let item_keys = item_keys
            .iter()
            .map(|key| ShareableString::from(*key))
            .collect();
        MissingRequirements::new(&expression, &item_keys, &HashSet::new())
    }

    #[test]
    fn indexed_map_reference_is_satisfied_by_flattened_entries() {
        // Why: Map items are flattened to `name[entry][field]` keys, so `name` itself never exists.
        let result = missing("p_map[entry][field] + 1.0", &["p_map[entry][field]"]);
        assert!(!result.missing_requirements_exist());
    }

    #[test]
    fn bare_map_reference_is_not_satisfied_by_flattened_entries() {
        let result = missing("p_map + 1.0", &["p_map[entry][field]"]);
        assert_eq!(result.parameters(), [ShareableString::from("p_map")]);
    }

    #[test]
    fn missing_indexed_reference_is_reported() {
        let result = missing("p_table[0][a]", &["p_other"]);
        assert_eq!(result.parameters(), [ShareableString::from("p_table")]);
    }
}
