use crate::core::Result;
use crate::core::ir::ResourceType;
use crate::languages::typescript::backend::TypeScriptBackend;
use crate::templates::genco_engine::GencoTemplateEngine;
use genco::prelude::*;
use heck::ToLowerCamelCase;

/// Generator for TypeScript helper methods and utilities
#[allow(dead_code)]
pub struct HelpersGenerator {
    backend: TypeScriptBackend,
}

impl HelpersGenerator {
    /// Create a new helpers generator
    pub fn new(backend: TypeScriptBackend) -> Self {
        Self { backend }
    }

    /// Generate the extractPrimitiveValue utility function
    ///
    /// This function extracts the actual value from a FhirPrimitive<T> wrapper
    pub fn generate_extract_primitive_value(&self) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // JSDoc
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Extract primitive value from FhirPrimitive<T> wrapper");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * @param primitive - The FhirPrimitive<T> or undefined");
        tokens.push();
        tokens.append(" * @returns The unwrapped value or undefined");
        tokens.push();
        tokens.append(" */");
        tokens.push();

        // Function declaration
        tokens.append("export function extractPrimitiveValue<T>(primitive: FhirPrimitive<T> | undefined): T | undefined {");
        tokens.push();
        tokens.indent();

        tokens.append("if (primitive === undefined) return undefined;");
        tokens.push();
        tokens.append("if (typeof primitive === \"object\" && \"value\" in primitive) {");
        tokens.indent();
        tokens.push();
        tokens.append("return primitive.value;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.append("return primitive as T;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate helper namespace for a resource
    ///
    /// Creates resource-specific helper methods like getOfficialName, getFullName, etc.
    pub fn generate_resource_helpers(&self, resource: &ResourceType) -> Result<String> {
        let resource_name = &resource.name;
        let namespace_name = format!("{}Helpers", resource_name);

        let mut tokens = js::Tokens::new();

        // JSDoc for namespace
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Helper methods for {} resource", resource_name));
        tokens.push();
        tokens.append(" */");
        tokens.push();

        // Namespace declaration
        tokens.append(format!("export namespace {} {{", namespace_name));
        tokens.push();
        tokens.indent();

        // Generate resource-specific helpers based on the resource type
        match resource_name.as_str() {
            "Patient" => self.generate_patient_helpers(&mut tokens)?,
            "Observation" => self.generate_observation_helpers(&mut tokens)?,
            _ => {
                // Generic helpers for other resources
                self.generate_generic_helpers(&mut tokens, resource)?;
            }
        }

        tokens.unindent();
        tokens.push();
        tokens.append("}");

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate Patient-specific helper methods
    fn generate_patient_helpers(&self, tokens: &mut js::Tokens) -> Result<()> {
        // getOfficialName
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get official name if available");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function getOfficialName(patient: Patient): HumanName | undefined {");
        tokens.indent();
        tokens.push();
        tokens.append("return patient.name?.find(n => n.use === \"official\");");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // getFullName
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get formatted full name");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function getFullName(patient: Patient): string | undefined {");
        tokens.indent();
        tokens.push();
        tokens.append("const name = getOfficialName(patient) ?? patient.name?.[0];");
        tokens.push();
        tokens.append("if (!name) return undefined;");
        tokens.push();
        tokens.push();
        tokens.append("return [name.prefix?.join(\" \"), name.given?.join(\" \"), name.family]");
        tokens.indent();
        tokens.push();
        tokens.append(".filter(Boolean)");
        tokens.push();
        tokens.append(".join(\" \");");
        tokens.unindent();
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // isDeceased
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Check if patient is deceased");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function isDeceased(patient: Patient): boolean {");
        tokens.indent();
        tokens.push();
        tokens.append("if (typeof patient.deceased === \"boolean\") {");
        tokens.indent();
        tokens.push();
        tokens.append("return patient.deceased;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.append("return patient.deceased !== undefined;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // getAge
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get age in years (requires birthDate)");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function getAge(patient: Patient, asOf: Date = new Date()): number | undefined {");
        tokens.indent();
        tokens.push();
        tokens.append("if (!patient.birthDate) return undefined;");
        tokens.push();
        tokens.push();
        tokens
            .append("const birth = new Date(extractPrimitiveValue(patient.birthDate) as string);");
        tokens.push();
        tokens.append("let age = asOf.getFullYear() - birth.getFullYear();");
        tokens.push();
        tokens.append("const monthDiff = asOf.getMonth() - birth.getMonth();");
        tokens.push();
        tokens.push();
        tokens.append(
            "if (monthDiff < 0 || (monthDiff === 0 && asOf.getDate() < birth.getDate())) {",
        );
        tokens.indent();
        tokens.push();
        tokens.append("age--;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();
        tokens.append("return age;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        Ok(())
    }

    /// Generate Observation-specific helper methods
    fn generate_observation_helpers(&self, tokens: &mut js::Tokens) -> Result<()> {
        // getValue
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get the observation value if present");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function getValue(observation: Observation): any | undefined {");
        tokens.indent();
        tokens.push();
        tokens.append("// Handle different value[x] variants");
        tokens.push();
        tokens.append("if (observation.valueQuantity) return observation.valueQuantity;");
        tokens.push();
        tokens.append(
            "if (observation.valueCodeableConcept) return observation.valueCodeableConcept;",
        );
        tokens.push();
        tokens.append("if (observation.valueString) return observation.valueString;");
        tokens.push();
        tokens
            .append("if (observation.valueBoolean !== undefined) return observation.valueBoolean;");
        tokens.push();
        tokens
            .append("if (observation.valueInteger !== undefined) return observation.valueInteger;");
        tokens.push();
        tokens.append("if (observation.valueRange) return observation.valueRange;");
        tokens.push();
        tokens.append("if (observation.valueRatio) return observation.valueRatio;");
        tokens.push();
        tokens.append("if (observation.valueSampledData) return observation.valueSampledData;");
        tokens.push();
        tokens.append("if (observation.valueTime) return observation.valueTime;");
        tokens.push();
        tokens.append("if (observation.valueDateTime) return observation.valueDateTime;");
        tokens.push();
        tokens.append("if (observation.valuePeriod) return observation.valuePeriod;");
        tokens.push();
        tokens.append("return undefined;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // hasValue
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Check if observation has a value");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append("export function hasValue(observation: Observation): boolean {");
        tokens.indent();
        tokens.push();
        tokens.append("return getValue(observation) !== undefined;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        Ok(())
    }

    /// Generate generic helper methods for any resource
    fn generate_generic_helpers(
        &self,
        tokens: &mut js::Tokens,
        resource: &ResourceType,
    ) -> Result<()> {
        // getId helper
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get the resource ID if present");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append(format!(
            "export function getId(resource: {}): string | undefined {{",
            resource.name
        ));
        tokens.indent();
        tokens.push();
        tokens.append("return resource.id;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");
        tokens.push();
        tokens.push();

        // getMeta helper
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Get the resource meta if present");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.append(format!(
            "export function getMeta(resource: {}): Meta | undefined {{",
            resource.name
        ));
        tokens.indent();
        tokens.push();
        tokens.append("return resource.meta;");
        tokens.unindent();
        tokens.push();
        tokens.append("}");

        Ok(())
    }

    /// Generate a complete helpers module for a resource
    pub fn generate_helpers_module(&self, resource: &ResourceType) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // File header
        tokens.append("/**");
        tokens.push();
        tokens.append(format!(" * Helper utilities for {} resource", resource.name));
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * This file is auto-generated. Do not edit manually.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Imports
        tokens.append(format!(
            "import {{ {} }} from './{}';",
            resource.name,
            resource.name.to_lower_camel_case()
        ));
        tokens.push();

        // Import specific types based on resource
        match resource.name.as_str() {
            "Patient" => {
                tokens.append("import { HumanName } from './humanName';");
                tokens.push();
            }
            "Observation" => {
                // No additional imports needed
            }
            _ => {
                tokens.append("import { Meta } from './meta';");
                tokens.push();
            }
        }

        tokens.append("import { extractPrimitiveValue } from './utilities';");
        tokens.push();
        tokens.push();

        // Generate helpers
        tokens.append(self.generate_resource_helpers(resource)?);

        GencoTemplateEngine::format_typescript(&tokens)
    }

    /// Generate the main utilities file with common helpers
    pub fn generate_utilities_module(&self) -> Result<String> {
        let mut tokens = js::Tokens::new();

        // File header
        tokens.append("/**");
        tokens.push();
        tokens.append(" * Common FHIR utility functions");
        tokens.push();
        tokens.append(" * ");
        tokens.push();
        tokens.append(" * This file is auto-generated. Do not edit manually.");
        tokens.push();
        tokens.append(" */");
        tokens.push();
        tokens.push();

        // Import FhirPrimitive type
        tokens.append("import { FhirPrimitive } from './primitives';");
        tokens.push();
        tokens.push();

        // extractPrimitiveValue
        tokens.append(self.generate_extract_primitive_value()?);

        GencoTemplateEngine::format_typescript(&tokens)
    }
}
