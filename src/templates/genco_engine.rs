//! Genco-based template engine for type-safe code generation

use crate::core::{Error, Result};
use crate::generator::{TemplateContext, TemplateEngine};
use async_trait::async_trait;
use genco::prelude::*;
use std::collections::HashMap;

/// Template renderer function type
type TemplateRenderer = Box<dyn Fn(&TemplateContext) -> Result<String> + Send + Sync>;

/// Genco-based template engine
pub struct GencoTemplateEngine {
    /// Registered templates
    templates: HashMap<String, TemplateRenderer>,
}

impl GencoTemplateEngine {
    /// Create a new genco template engine
    pub fn new() -> Self {
        Self { templates: HashMap::new() }
    }

    /// Register a template with a renderer function
    ///
    /// # Arguments
    ///
    /// * `name` - Template name/identifier
    /// * `renderer` - Function that renders the template given context
    pub fn register_template<F>(&mut self, name: String, renderer: F)
    where
        F: Fn(&TemplateContext) -> Result<String> + Send + Sync + 'static,
    {
        self.templates.insert(name, Box::new(renderer));
    }

    /// Format TypeScript code using genco
    pub fn format_typescript(tokens: &js::Tokens) -> Result<String> {
        tokens
            .to_file_string()
            .map_err(|e| Error::Generator(format!("Failed to format TypeScript: {}", e)))
    }
}

impl Default for GencoTemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemplateEngine for GencoTemplateEngine {
    async fn render(&self, template_name: &str, context: &TemplateContext) -> Result<String> {
        let renderer = self
            .templates
            .get(template_name)
            .ok_or_else(|| Error::Generator(format!("Template not found: {}", template_name)))?;

        renderer(context)
    }

    fn has_template(&self, template_name: &str) -> bool {
        self.templates.contains_key(template_name)
    }

    fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
}

/// Helper functions for common TypeScript patterns
pub mod helpers {
    use super::*;

    /// Generate a JSDoc comment block
    pub fn jsdoc_comment(lines: &[String]) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        if lines.is_empty() {
            return tokens;
        }

        tokens.push();
        tokens.append("/**");
        for line in lines {
            tokens.push();
            tokens.append(" * ");
            tokens.append(line);
        }
        tokens.push();
        tokens.append(" */");

        tokens
    }

    /// Generate an interface declaration
    pub fn interface(
        name: &str,
        extends: Option<&str>,
        properties: &[(String, String, bool)], // (name, type, optional)
        doc: Option<&[String]>,
    ) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        // Documentation
        if let Some(doc_lines) = doc {
            tokens.append(jsdoc_comment(doc_lines));
            tokens.push();
        }

        // Interface declaration
        tokens.append("export interface ");
        tokens.append(name);

        if let Some(base) = extends {
            tokens.append(" extends ");
            tokens.append(base);
        }

        tokens.append(" {");
        tokens.push();

        // Properties
        for (prop_name, prop_type, optional) in properties {
            tokens.append("  ");
            tokens.append(prop_name);
            if *optional {
                tokens.append("?");
            }
            tokens.append(": ");
            tokens.append(prop_type);
            tokens.append(";");
            tokens.push();
        }

        tokens.append("}");

        tokens
    }

    /// Generate a type alias
    pub fn type_alias(name: &str, type_def: &str, doc: Option<&[String]>) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        // Documentation
        if let Some(doc_lines) = doc {
            tokens.append(jsdoc_comment(doc_lines));
            tokens.push();
        }

        tokens.append("export type ");
        tokens.append(name);
        tokens.append(" = ");
        tokens.append(type_def);
        tokens.append(";");

        tokens
    }

    /// Generate import statements
    pub fn imports(modules: &[(Vec<String>, String)]) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        for (types, from) in modules {
            if types.is_empty() {
                continue;
            }

            tokens.append("import { ");
            tokens.append(types.join(", "));
            tokens.append(" } from '");
            tokens.append(from);
            tokens.append("';");
            tokens.push();
        }

        tokens
    }

    /// Generate a union type
    pub fn union_type(types: &[String]) -> String {
        types.join(" | ")
    }

    /// Generate an array type
    pub fn array_type(element_type: &str) -> String {
        format!("{}[]", element_type)
    }

    /// Generate a string literal type
    pub fn string_literal(value: &str) -> String {
        format!("\"{}\"", value)
    }

    /// Generate an object type inline
    pub fn object_type(properties: &[(String, String, bool)]) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        tokens.append("{");
        tokens.push();

        for (prop_name, prop_type, optional) in properties {
            tokens.append("  ");
            tokens.append(prop_name);
            if *optional {
                tokens.append("?");
            }
            tokens.append(": ");
            tokens.append(prop_type);
            tokens.append(";");
            tokens.push();
        }

        tokens.append("}");

        tokens
    }

    /// Generate a constant declaration
    pub fn constant(name: &str, value: &str, type_annotation: Option<&str>) -> js::Tokens {
        let mut tokens = js::Tokens::new();

        tokens.append("export const ");
        tokens.append(name);

        if let Some(type_def) = type_annotation {
            tokens.append(": ");
            tokens.append(type_def);
        }

        tokens.append(" = ");
        tokens.append(value);
        tokens.append(";");

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_engine_creation() {
        let engine = GencoTemplateEngine::new();
        assert_eq!(engine.list_templates().len(), 0);
    }

    #[test]
    fn test_register_template() {
        let mut engine = GencoTemplateEngine::new();

        engine.register_template("test".to_string(), |_ctx| Ok("Hello, World!".to_string()));

        assert!(engine.has_template("test"));
        assert!(!engine.has_template("missing"));
    }

    #[tokio::test]
    async fn test_render_template() {
        let mut engine = GencoTemplateEngine::new();

        engine.register_template("greeting".to_string(), |ctx| {
            let name = ctx.data.get("name").and_then(|v| v.as_str()).unwrap_or("World");
            Ok(format!("Hello, {}!", name))
        });

        let ctx = TemplateContext::new(json!({"name": "Rust"}));
        let result = engine.render("greeting", &ctx).await.unwrap();
        assert_eq!(result, "Hello, Rust!");
    }

    #[tokio::test]
    async fn test_render_missing_template() {
        let engine = GencoTemplateEngine::new();
        let ctx = TemplateContext::new(json!({}));
        let result = engine.render("missing", &ctx).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_jsdoc_comment() {
        let lines =
            vec!["Patient resource".to_string(), "Contains patient demographics".to_string()];

        let tokens = helpers::jsdoc_comment(&lines);
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("/**"));
        assert!(output.contains("Patient resource"));
        assert!(output.contains("*/"));
    }

    #[test]
    fn test_interface_generation() {
        let props = vec![
            ("id".to_string(), "string".to_string(), true),
            ("name".to_string(), "string".to_string(), false),
        ];

        let doc = vec!["User interface".to_string()];

        let tokens = helpers::interface("User", None, &props, Some(&doc));
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("export interface User"));
        assert!(output.contains("id?: string;"));
        assert!(output.contains("name: string;"));
        assert!(output.contains("User interface"));
    }

    #[test]
    fn test_interface_with_extends() {
        let props = vec![("resourceType".to_string(), "\"Patient\"".to_string(), false)];

        let tokens = helpers::interface("Patient", Some("DomainResource"), &props, None);
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("extends DomainResource"));
    }

    #[test]
    fn test_type_alias() {
        let tokens = helpers::type_alias(
            "Status",
            "\"active\" | \"inactive\"",
            Some(&vec!["Status type".to_string()]),
        );
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("export type Status"));
        assert!(output.contains("\"active\" | \"inactive\""));
    }

    #[test]
    fn test_imports() {
        let modules = vec![
            (vec!["Patient".to_string(), "Observation".to_string()], "./resources".to_string()),
            (vec!["HumanName".to_string()], "./datatypes".to_string()),
        ];

        let tokens = helpers::imports(&modules);
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("import { Patient, Observation } from './resources'"));
        assert!(output.contains("import { HumanName } from './datatypes'"));
    }

    #[test]
    fn test_union_type() {
        let types = vec!["string".to_string(), "number".to_string(), "boolean".to_string()];
        let result = helpers::union_type(&types);
        assert_eq!(result, "string | number | boolean");
    }

    #[test]
    fn test_array_type() {
        assert_eq!(helpers::array_type("string"), "string[]");
        assert_eq!(helpers::array_type("Patient"), "Patient[]");
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(helpers::string_literal("Patient"), "\"Patient\"");
    }

    #[test]
    fn test_constant_with_type() {
        let tokens = helpers::constant("MAX_SIZE", "1000", Some("number"));
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("export const MAX_SIZE: number = 1000"));
    }

    #[test]
    fn test_constant_without_type() {
        let tokens = helpers::constant("VERSION", "\"1.0.0\"", None);
        let output = GencoTemplateEngine::format_typescript(&tokens).unwrap();

        assert!(output.contains("export const VERSION = \"1.0.0\""));
    }
}
