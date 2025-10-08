use octofhir_canonical_manager::{CanonicalManager, FcmConfig};
use octofhir_codegen::build_type_graph;
use octofhir_codegen::core::graph_builder::TypeGraphBuilder;
use octofhir_codegen::core::ir::FhirVersion;
use std::sync::Arc;
use tempfile::TempDir;

async fn setup_test_manager() -> Arc<CanonicalManager> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = FcmConfig::default();
    config.storage.packages_dir = temp_dir.path().join("packages");
    config.storage.cache_dir = temp_dir.path().join("cache");
    config.storage.index_dir = temp_dir.path().join("index");
    Arc::new(CanonicalManager::new(config).await.unwrap())
}

#[tokio::test]
async fn test_builder_creation() {
    let manager = setup_test_manager().await;
    let _builder = TypeGraphBuilder::new(manager, FhirVersion::R4);
    // Just verify it creates successfully
}

#[tokio::test]
async fn test_empty_graph() {
    let manager = setup_test_manager().await;
    let builder = TypeGraphBuilder::new(manager, FhirVersion::R4);

    // Building with no packages should return empty graph
    let graph = builder.build().await.unwrap();

    assert_eq!(graph.resources.len(), 0);
    assert_eq!(graph.datatypes.len(), 0);
    assert_eq!(graph.primitives.len(), 0);
    assert_eq!(graph.fhir_version, FhirVersion::R4);
}

#[tokio::test]
async fn test_builder_different_versions() {
    let manager = setup_test_manager().await;

    let builder_r4 = TypeGraphBuilder::new(Arc::clone(&manager), FhirVersion::R4);
    let graph_r4 = builder_r4.build().await.unwrap();
    assert_eq!(graph_r4.fhir_version, FhirVersion::R4);

    let builder_r5 = TypeGraphBuilder::new(Arc::clone(&manager), FhirVersion::R5);
    let graph_r5 = builder_r5.build().await.unwrap();
    assert_eq!(graph_r5.fhir_version, FhirVersion::R5);
}

#[tokio::test]
async fn test_build_type_graph_convenience() {
    let manager = setup_test_manager().await;

    // Test the convenience function
    // Will fail to install package but shouldn't panic
    let result =
        build_type_graph(Arc::clone(&manager), FhirVersion::R4, "nonexistent.package", "1.0.0")
            .await;

    // Should fail gracefully
    assert!(result.is_err());
}

#[tokio::test]
async fn test_graph_metadata() {
    let manager = setup_test_manager().await;
    let builder = TypeGraphBuilder::new(manager, FhirVersion::R4);

    let graph = builder.build().await.unwrap();

    // Check metadata is populated
    assert_eq!(graph.metadata.source_packages.len(), 0); // No packages installed
    assert!(!graph.metadata.generated_at.is_empty());
}

#[tokio::test]
async fn test_graph_total_types() {
    let manager = setup_test_manager().await;
    let builder = TypeGraphBuilder::new(manager, FhirVersion::R4);

    let graph = builder.build().await.unwrap();

    // With no packages, should have 0 types
    assert_eq!(graph.total_types(), 0);
}

#[tokio::test]
#[ignore] // Requires network access and R4 package installation
async fn test_build_r4_core_graph() {
    let manager = setup_test_manager().await;

    // This test requires hl7.fhir.r4.core to be installed
    // In CI, we'd pre-install or skip this test

    let result = build_type_graph(manager, FhirVersion::R4, "hl7.fhir.r4.core", "4.0.1").await;

    if let Ok(graph) = result {
        // Should have standard R4 resources
        assert!(graph.resources.contains_key("Patient"));
        assert!(graph.resources.contains_key("Observation"));

        // Should have datatypes
        assert!(graph.datatypes.contains_key("HumanName"));
        assert!(graph.datatypes.contains_key("Address"));

        // Should have primitives
        assert!(graph.primitives.contains_key("string"));
        assert!(graph.primitives.contains_key("boolean"));

        // Check total types is reasonable
        assert!(graph.total_types() > 100); // R4 has hundreds of types
    }
}

#[tokio::test]
async fn test_categorize_structures() {
    let manager = setup_test_manager().await;
    let builder = TypeGraphBuilder::new(manager, FhirVersion::R4);

    // Test with sample structures
    let structures = vec![
        serde_json::json!({
            "resourceType": "StructureDefinition",
            "name": "Patient",
            "kind": "resource",
            "url": "http://hl7.org/fhir/StructureDefinition/Patient"
        }),
        serde_json::json!({
            "resourceType": "StructureDefinition",
            "name": "HumanName",
            "kind": "complex-type",
            "url": "http://hl7.org/fhir/StructureDefinition/HumanName"
        }),
        serde_json::json!({
            "resourceType": "StructureDefinition",
            "name": "string",
            "kind": "primitive-type",
            "url": "http://hl7.org/fhir/StructureDefinition/string"
        }),
    ];

    let (resources, datatypes, primitives) =
        builder.categorize_structures(&structures).await.unwrap();

    assert_eq!(resources.len(), 1);
    assert!(resources.contains_key("Patient"));

    assert_eq!(datatypes.len(), 1);
    assert!(datatypes.contains_key("HumanName"));

    assert_eq!(primitives.len(), 1);
    assert!(primitives.contains_key("string"));
}

#[tokio::test]
async fn test_categorize_with_unknown_kind() {
    let manager = setup_test_manager().await;
    let builder = TypeGraphBuilder::new(manager, FhirVersion::R4);

    let structures = vec![
        serde_json::json!({
            "resourceType": "StructureDefinition",
            "name": "Unknown",
            "kind": "unknown-type"
        }),
        serde_json::json!({
            "resourceType": "StructureDefinition",
            "name": "NoKind"
        }),
    ];

    let (resources, datatypes, primitives) =
        builder.categorize_structures(&structures).await.unwrap();

    // Unknown kinds should be skipped
    assert_eq!(resources.len(), 0);
    assert_eq!(datatypes.len(), 0);
    assert_eq!(primitives.len(), 0);
}
