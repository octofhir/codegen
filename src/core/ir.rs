//! Intermediate Representation (IR) for FHIR types
//!
//! This module defines language-agnostic representations of FHIR resources,
//! datatypes, and primitives that can be transformed into any target language.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete type graph representing all FHIR definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeGraph {
    /// Resource definitions (e.g., Patient, Observation)
    pub resources: IndexMap<String, ResourceType>,

    /// Complex datatype definitions (e.g., HumanName, Address)
    pub datatypes: IndexMap<String, DataType>,

    /// Primitive type definitions (e.g., string, boolean, date)
    pub primitives: IndexMap<String, PrimitiveType>,

    /// Profile definitions (e.g., USCorePatient)
    pub profiles: IndexMap<String, ProfileType>,

    /// FHIR version this graph represents
    pub fhir_version: FhirVersion,

    /// Metadata about generation
    pub metadata: GraphMetadata,
}

impl TypeGraph {
    /// Create a new empty type graph
    pub fn new(fhir_version: FhirVersion) -> Self {
        Self {
            resources: IndexMap::new(),
            datatypes: IndexMap::new(),
            primitives: IndexMap::new(),
            profiles: IndexMap::new(),
            fhir_version,
            metadata: GraphMetadata::default(),
        }
    }

    /// Add a resource to the graph
    pub fn add_resource(&mut self, name: String, resource: ResourceType) {
        self.resources.insert(name, resource);
    }

    /// Add a datatype to the graph
    pub fn add_datatype(&mut self, name: String, datatype: DataType) {
        self.datatypes.insert(name, datatype);
    }

    /// Add a primitive to the graph
    pub fn add_primitive(&mut self, name: String, primitive: PrimitiveType) {
        self.primitives.insert(name, primitive);
    }

    /// Add a profile to the graph
    pub fn add_profile(&mut self, name: String, profile: ProfileType) {
        self.profiles.insert(name, profile);
    }

    /// Get total number of types in graph
    pub fn total_types(&self) -> usize {
        self.resources.len() + self.datatypes.len() + self.primitives.len() + self.profiles.len()
    }
}

/// Metadata about the type graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphMetadata {
    /// When this graph was generated
    pub generated_at: String,

    /// Generator version
    pub generator_version: String,

    /// Source packages
    pub source_packages: Vec<String>,

    /// Custom metadata
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for GraphMetadata {
    fn default() -> Self {
        Self {
            generated_at: chrono::Utc::now().to_rfc3339(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            source_packages: Vec::new(),
            custom: HashMap::new(),
        }
    }
}

/// FHIR version
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FhirVersion {
    /// FHIR R4 (4.0.x)
    R4,
    /// FHIR R4B (4.3.x)
    R4B,
    /// FHIR R5 (5.0.x)
    R5,
    /// FHIR R6 (6.0.x)
    R6,
}

impl std::fmt::Display for FhirVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FhirVersion::R4 => write!(f, "R4"),
            FhirVersion::R4B => write!(f, "R4B"),
            FhirVersion::R5 => write!(f, "R5"),
            FhirVersion::R6 => write!(f, "R6"),
        }
    }
}

/// FHIR Resource type (e.g., Patient, Observation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceType {
    /// Resource name (e.g., "Patient")
    pub name: String,

    /// Base resource this extends (e.g., Some("DomainResource"))
    pub base: Option<String>,

    /// Properties/elements of this resource
    pub properties: Vec<Property>,

    /// Search parameters defined for this resource
    pub search_parameters: Vec<SearchParameter>,

    /// Documentation
    pub documentation: Documentation,

    /// Canonical URL
    pub url: String,

    /// Is this an abstract type?
    pub is_abstract: bool,
}

/// FHIR Complex datatype (e.g., HumanName, Address)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataType {
    /// Datatype name (e.g., "HumanName")
    pub name: String,

    /// Base type this extends (if any)
    pub base: Option<String>,

    /// Properties of this datatype
    pub properties: Vec<Property>,

    /// Documentation
    pub documentation: Documentation,

    /// Canonical URL
    pub url: String,

    /// Is this an abstract type?
    pub is_abstract: bool,
}

/// FHIR Primitive type (e.g., string, boolean, date)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveType {
    /// Primitive name (e.g., "string", "boolean")
    pub name: String,

    /// Base primitive (all primitives extend from Element)
    pub base: Option<String>,

    /// Regex pattern for validation (if any)
    pub pattern: Option<String>,

    /// Documentation
    pub documentation: Documentation,

    /// Canonical URL
    pub url: String,
}

/// FHIR Profile (constraint on a resource or datatype)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileType {
    /// Profile name (e.g., "USCorePatient")
    pub name: String,

    /// Base type being constrained (e.g., "Patient")
    pub base: String,

    /// Additional constraints on properties
    pub property_constraints: Vec<PropertyConstraint>,

    /// New properties added by this profile
    pub new_properties: Vec<Property>,

    /// Documentation
    pub documentation: Documentation,

    /// Canonical URL
    pub url: String,
}

/// Property constraint in a profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyConstraint {
    /// Path to the property being constrained
    pub path: String,

    /// New cardinality (if changed)
    pub cardinality: Option<CardinalityRange>,

    /// New type constraints
    pub type_constraints: Vec<String>,

    /// New binding (if changed)
    pub binding: Option<ValueSetBinding>,

    /// Must support flag
    pub must_support: bool,
}

/// A property (field/element) in a resource or datatype
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Property {
    /// Property name (e.g., "name", "birthDate")
    pub name: String,

    /// Original FHIR path (e.g., "Patient.name")
    pub path: String,

    /// Type of this property
    pub property_type: PropertyType,

    /// Cardinality (min..max)
    pub cardinality: CardinalityRange,

    /// Is this a choice element? (e.g., value[x])
    pub is_choice: bool,

    /// If choice, what are the possible types?
    pub choice_types: Vec<String>,

    /// Is this a modifier element?
    pub is_modifier: bool,

    /// Is this a summary element?
    pub is_summary: bool,

    /// Terminology binding (if any)
    pub binding: Option<ValueSetBinding>,

    /// Constraints/invariants
    pub constraints: Vec<InvariantRule>,

    /// Short description
    pub short_description: String,

    /// Full definition
    pub definition: String,

    /// Comments
    pub comments: Option<String>,

    /// Example values
    pub examples: Vec<Example>,
}

/// Type of a property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum PropertyType {
    /// Primitive type (e.g., "string", "boolean")
    Primitive {
        /// Type name
        type_name: String,
    },

    /// Complex type (e.g., "HumanName", "Address")
    Complex {
        /// Type name
        type_name: String,
    },

    /// Reference to another resource
    Reference {
        /// Allowed target types (empty = any resource)
        target_types: Vec<String>,
    },

    /// Backbone element (inline complex type)
    BackboneElement {
        /// Properties in this backbone element
        properties: Vec<Property>,
    },

    /// Choice type (one of several types)
    Choice {
        /// Possible types for this choice
        types: Vec<String>,
    },
}

impl PropertyType {
    /// Get the primary type name (for simple cases)
    pub fn type_name(&self) -> Option<&str> {
        match self {
            PropertyType::Primitive { type_name } => Some(type_name),
            PropertyType::Complex { type_name } => Some(type_name),
            _ => None,
        }
    }
}

/// Cardinality range (min..max)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardinalityRange {
    /// Minimum occurrences
    pub min: u32,

    /// Maximum occurrences (None = unbounded)
    pub max: Option<u32>,
}

impl CardinalityRange {
    /// Create a required single value (1..1)
    pub fn required() -> Self {
        Self { min: 1, max: Some(1) }
    }

    /// Create an optional single value (0..1)
    pub fn optional() -> Self {
        Self { min: 0, max: Some(1) }
    }

    /// Create a required array (1..*)
    pub fn required_array() -> Self {
        Self { min: 1, max: None }
    }

    /// Create an optional array (0..*)
    pub fn optional_array() -> Self {
        Self { min: 0, max: None }
    }

    /// Is this property required?
    pub fn is_required(&self) -> bool {
        self.min >= 1
    }

    /// Is this property an array?
    pub fn is_array(&self) -> bool {
        self.max.is_none() || self.max.is_some_and(|max| max > 1)
    }

    /// Is this property optional?
    pub fn is_optional(&self) -> bool {
        self.min == 0
    }
}

/// ValueSet binding for coded elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueSetBinding {
    /// Binding strength
    pub strength: BindingStrength,

    /// ValueSet canonical URL
    pub value_set: String,

    /// Description
    pub description: Option<String>,
}

/// Binding strength
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BindingStrength {
    /// Required (must use)
    Required,
    /// Extensible (should use)
    Extensible,
    /// Preferred (could use)
    Preferred,
    /// Example (may use)
    Example,
}

/// Invariant constraint rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvariantRule {
    /// Invariant key (e.g., "pat-1")
    pub key: String,

    /// Severity
    pub severity: ConstraintSeverity,

    /// Human description
    pub human: String,

    /// FHIRPath expression
    pub expression: Option<String>,

    /// XPath expression (legacy)
    pub xpath: Option<String>,
}

/// Constraint severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintSeverity {
    /// Error (must satisfy)
    Error,
    /// Warning (should satisfy)
    Warning,
}

/// Example value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Example {
    /// Example label
    pub label: String,

    /// Example value (as JSON)
    pub value: serde_json::Value,
}

/// Documentation for types and properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Documentation {
    /// Short description
    pub short: String,

    /// Full definition
    pub definition: String,

    /// Comments
    pub comments: Option<String>,

    /// Requirements
    pub requirements: Option<String>,

    /// Usage notes
    pub usage_notes: Vec<String>,

    /// Canonical URL
    pub url: Option<String>,
}

/// Search parameter definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchParameter {
    /// Parameter name/code
    pub code: String,

    /// Parameter type
    pub param_type: SearchParamType,

    /// Description
    pub description: String,

    /// FHIRPath expression
    pub expression: Option<String>,

    /// Target resource types (for reference params)
    pub target_types: Vec<String>,
}

/// Search parameter type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchParamType {
    /// Number
    Number,
    /// Date/DateTime
    Date,
    /// String
    String,
    /// Token
    Token,
    /// Reference
    Reference,
    /// Composite
    Composite,
    /// Quantity
    Quantity,
    /// URI
    Uri,
    /// Special
    Special,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_graph_creation() {
        let graph = TypeGraph::new(FhirVersion::R4);
        assert_eq!(graph.fhir_version, FhirVersion::R4);
        assert_eq!(graph.total_types(), 0);
    }

    #[test]
    fn test_type_graph_add_resource() {
        let mut graph = TypeGraph::new(FhirVersion::R4);

        let resource = ResourceType {
            name: "Patient".to_string(),
            base: Some("DomainResource".to_string()),
            properties: Vec::new(),
            search_parameters: Vec::new(),
            documentation: Documentation::default(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            is_abstract: false,
        };

        graph.add_resource("Patient".to_string(), resource);
        assert_eq!(graph.total_types(), 1);
        assert!(graph.resources.contains_key("Patient"));
    }

    #[test]
    fn test_cardinality_required() {
        let card = CardinalityRange::required();
        assert_eq!(card.min, 1);
        assert_eq!(card.max, Some(1));
        assert!(card.is_required());
        assert!(!card.is_array());
        assert!(!card.is_optional());
    }

    #[test]
    fn test_cardinality_optional() {
        let card = CardinalityRange::optional();
        assert_eq!(card.min, 0);
        assert_eq!(card.max, Some(1));
        assert!(!card.is_required());
        assert!(!card.is_array());
        assert!(card.is_optional());
    }

    #[test]
    fn test_cardinality_array() {
        let card = CardinalityRange::optional_array();
        assert_eq!(card.min, 0);
        assert_eq!(card.max, None);
        assert!(!card.is_required());
        assert!(card.is_array());
        assert!(card.is_optional());
    }

    #[test]
    fn test_cardinality_required_array() {
        let card = CardinalityRange::required_array();
        assert_eq!(card.min, 1);
        assert_eq!(card.max, None);
        assert!(card.is_required());
        assert!(card.is_array());
        assert!(!card.is_optional());
    }

    #[test]
    fn test_property_type_typename() {
        let prim = PropertyType::Primitive { type_name: "string".to_string() };
        assert_eq!(prim.type_name(), Some("string"));

        let complex = PropertyType::Complex { type_name: "HumanName".to_string() };
        assert_eq!(complex.type_name(), Some("HumanName"));

        let reference = PropertyType::Reference { target_types: vec!["Patient".to_string()] };
        assert_eq!(reference.type_name(), None);
    }

    #[test]
    fn test_fhir_version_display() {
        assert_eq!(FhirVersion::R4.to_string(), "R4");
        assert_eq!(FhirVersion::R4B.to_string(), "R4B");
        assert_eq!(FhirVersion::R5.to_string(), "R5");
        assert_eq!(FhirVersion::R6.to_string(), "R6");
    }

    #[test]
    fn test_graph_metadata_default() {
        let metadata = GraphMetadata::default();
        assert_eq!(metadata.generator_version, env!("CARGO_PKG_VERSION"));
        assert!(metadata.source_packages.is_empty());
        assert!(!metadata.generated_at.is_empty());
    }
}
