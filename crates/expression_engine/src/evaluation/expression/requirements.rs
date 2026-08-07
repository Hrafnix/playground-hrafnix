use crate::expression::ast::translator::{Expression, Literal};
use shareable_string::ShareableString;
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

        for global in requirements.globals {
            if !item_keys.contains(&global) {
                missing_globals.push(global);
            }
        }

        for parameter in requirements.parameters {
            if !item_keys.contains(&parameter) {
                missing_parameters.push(parameter);
            }
        }

        for variable in requirements.variables {
            if !item_keys.contains(&variable) {
                missing_variables.push(variable);
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

    /// Returns `true` if any required globals, parameters, variables, or functions are missing.
    pub(crate) fn missing_requirements_exist(&self) -> bool {
        !self.globals.is_empty()
            || !self.parameters.is_empty()
            || !self.variables.is_empty()
            || !self.functions.is_empty()
    }

    /// Returns the names of missing globals.
    pub(crate) fn globals(&self) -> &[ShareableString] {
        &self.globals
    }

    /// Returns `true` if any globals are missing.
    pub(crate) fn missing_globals(&self) -> bool {
        !self.globals.is_empty()
    }

    /// Returns the names of missing parameters.
    pub(crate) fn parameters(&self) -> &[ShareableString] {
        &self.parameters
    }

    /// Returns `true` if any parameters are missing.
    pub(crate) fn missing_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Returns the names of missing variables.
    pub(crate) fn variables(&self) -> &[ShareableString] {
        &self.variables
    }

    /// Returns `true` if any variables are missing.
    pub(crate) fn missing_variables(&self) -> bool {
        !self.variables.is_empty()
    }

    /// Returns the names of missing functions.
    pub(crate) fn functions(&self) -> &[ShareableString] {
        &self.functions
    }

    /// Returns `true` if any functions are missing.
    pub(crate) fn missing_functions(&self) -> bool {
        !self.functions.is_empty()
    }
}

/// Represents the requirements of an expression in terms of the global variables,
/// parameters, variables, and functions it references. This struct is used to analyze
/// an expression and determine its dependencies, which can be useful for validation,
/// optimization, and ensuring that all necessary resources are available for evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionRequirements {
    /// The global variables referenced by the expression.
    globals: Vec<ShareableString>,
    /// The parameters referenced by the expression.
    parameters: Vec<ShareableString>,
    /// The variables referenced by the expression.
    variables: Vec<ShareableString>,
    /// The functions referenced by the expression.
    functions: Vec<ShareableString>,
}

impl ExpressionRequirements {
    /// Walks `expression` and collects all referenced globals, parameters, variables, and functions.
    pub(crate) fn new(expression: &Expression) -> Self {
        let mut global = Vec::new();
        let mut parameters = Vec::new();
        let mut variables = Vec::new();
        let mut functions = Vec::new();
        let mut seen_names = HashSet::new();
        let mut seen_functions = HashSet::new();

        Self::collect_requirements(
            expression,
            &mut global,
            &mut parameters,
            &mut variables,
            &mut functions,
            &mut seen_names,
            &mut seen_functions,
        );

        Self {
            globals: global,
            parameters,
            variables,
            functions,
        }
    }

    /// Classifies a name (e.g. `g_foo`, `p_bar`, `v_baz`) into the appropriate
    /// requirement bucket based on its prefix, recording it only the first time
    /// it is encountered.
    fn record_name(
        name: &str,
        global: &mut Vec<ShareableString>,
        parameters: &mut Vec<ShareableString>,
        variables: &mut Vec<ShareableString>,
        seen_names: &mut HashSet<ShareableString>,
    ) {
        let key = ShareableString::from(name);
        if !seen_names.insert(key.clone()) {
            return;
        }

        if name.starts_with("g_") {
            global.push(key);
        } else if name.starts_with("p_") {
            parameters.push(key);
        } else if name.starts_with("v_") {
            variables.push(key);
        }
    }

    /// Recursively walks an `Expression` tree, collecting the globals,
    /// parameters, variables, and function names it references.
    fn collect_requirements(
        expression: &Expression,
        global: &mut Vec<ShareableString>,
        parameters: &mut Vec<ShareableString>,
        variables: &mut Vec<ShareableString>,
        functions: &mut Vec<ShareableString>,
        seen_names: &mut HashSet<ShareableString>,
        seen_functions: &mut HashSet<ShareableString>,
    ) {
        match expression {
            Expression::Literal(_, Literal::Identifier(name)) => {
                Self::record_name(name, global, parameters, variables, seen_names);
            }
            Expression::Literal(_, _) => {}
            Expression::BinaryOperation { left, right, .. } => {
                Self::collect_requirements(
                    left,
                    global,
                    parameters,
                    variables,
                    functions,
                    seen_names,
                    seen_functions,
                );
                Self::collect_requirements(
                    right,
                    global,
                    parameters,
                    variables,
                    functions,
                    seen_names,
                    seen_functions,
                );
            }
            Expression::UnaryOperation { operand, .. } => {
                Self::collect_requirements(
                    operand,
                    global,
                    parameters,
                    variables,
                    functions,
                    seen_names,
                    seen_functions,
                );
            }
            Expression::FunctionCall {
                name, arguments, ..
            } => {
                let key = ShareableString::from(name.as_str());
                if seen_functions.insert(key.clone()) {
                    functions.push(key);
                }
                for argument in arguments {
                    Self::collect_requirements(
                        argument,
                        global,
                        parameters,
                        variables,
                        functions,
                        seen_names,
                        seen_functions,
                    );
                }
            }
            Expression::Index { name, index, .. } => {
                Self::record_name(name, global, parameters, variables, seen_names);
                for index_expression in index {
                    // A bare string literal used as an index (e.g., the `col` in
                    // `t[0][col]`) is a literal field name, not a reference to a
                    // variable, so it is not treated as a requirement here.
                    if matches!(
                        index_expression,
                        Expression::Literal(_, Literal::Identifier(_))
                    ) {
                        continue;
                    }
                    Self::collect_requirements(
                        index_expression,
                        global,
                        parameters,
                        variables,
                        functions,
                        seen_names,
                        seen_functions,
                    );
                }
            }
        }
    }
}
