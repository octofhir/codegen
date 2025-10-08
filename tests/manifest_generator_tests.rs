use octofhir_codegen::languages::typescript::{
    ManifestGenerator, PackageConfig, TypeScriptBackend,
};

fn create_test_config() -> PackageConfig {
    PackageConfig {
        name: "@test/fhir-r4".to_string(),
        version: "1.0.0".to_string(),
        description: "Test FHIR R4 SDK".to_string(),
        fhir_version: "R4".to_string(),
        repository_url: Some("https://github.com/test/fhir-r4".to_string()),
        license: "MIT".to_string(),
    }
}

#[test]
fn test_generate_package_json() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_package_json().unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify key fields
    assert_eq!(parsed["name"], "@test/fhir-r4");
    assert_eq!(parsed["version"], "1.0.0");
    assert_eq!(parsed["description"], "Test FHIR R4 SDK");
    assert_eq!(parsed["license"], "MIT");
    assert_eq!(parsed["type"], "module");
    assert_eq!(parsed["main"], "./dist/index.js");
    assert_eq!(parsed["types"], "./dist/index.d.ts");
}

#[test]
fn test_package_json_has_scripts() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_package_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify scripts
    assert!(parsed["scripts"]["build"].is_string());
    assert!(parsed["scripts"]["test"].is_string());
    assert!(parsed["scripts"]["lint"].is_string());
    assert!(parsed["scripts"]["format"].is_string());
    assert!(parsed["scripts"]["typecheck"].is_string());
}

#[test]
fn test_package_json_has_exports() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_package_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify ESM-only exports
    assert!(parsed["exports"]["."]["default"].is_string());
    assert!(parsed["exports"]["."]["types"].is_string());
    assert!(parsed["exports"]["./types/*"]["default"].is_string());
    assert!(parsed["exports"]["./types/*"]["types"].is_string());

    // Verify no CommonJS exports
    assert!(parsed["exports"]["."]["require"].is_null());
}

#[test]
fn test_package_json_has_repository() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_package_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["repository"]["type"], "git");
    assert_eq!(parsed["repository"]["url"], "https://github.com/test/fhir-r4");
}

#[test]
fn test_generate_tsconfig_json() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_tsconfig_json().unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify compiler options
    assert_eq!(parsed["compilerOptions"]["target"], "ES2022");
    assert_eq!(parsed["compilerOptions"]["module"], "ESNext");
    assert_eq!(parsed["compilerOptions"]["strict"], true);
    assert_eq!(parsed["compilerOptions"]["outDir"], "./dist");
    assert_eq!(parsed["compilerOptions"]["rootDir"], "./src");
}

#[test]
fn test_tsconfig_strict_mode() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_tsconfig_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify strict mode flags
    assert_eq!(parsed["compilerOptions"]["strict"], true);
    assert_eq!(parsed["compilerOptions"]["noUncheckedIndexedAccess"], true);
    assert_eq!(parsed["compilerOptions"]["noUnusedLocals"], true);
    assert_eq!(parsed["compilerOptions"]["noUnusedParameters"], true);
    assert_eq!(parsed["compilerOptions"]["noImplicitReturns"], true);
    assert_eq!(parsed["compilerOptions"]["noFallthroughCasesInSwitch"], true);
}

#[test]
fn test_generate_readme() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_readme().unwrap();

    // Verify key sections
    assert!(result.contains("# @test/fhir-r4"));
    assert!(result.contains("Test FHIR R4 SDK"));
    assert!(result.contains("## Installation"));
    assert!(result.contains("## Features"));
    assert!(result.contains("## Usage"));
    assert!(result.contains("## API Reference"));
    assert!(result.contains("## License"));
}

#[test]
fn test_readme_has_examples() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_readme().unwrap();

    // Verify examples
    assert!(result.contains("```typescript"));
    assert!(result.contains("import { Patient"));
    assert!(result.contains("validatePatient"));
    assert!(result.contains("PatientHelpers"));
}

#[test]
fn test_readme_includes_fhir_version() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_readme().unwrap();

    assert!(result.contains("FHIR R4"));
    assert!(result.contains("FHIR Version: R4"));
}

#[test]
fn test_generate_gitignore() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_gitignore().unwrap();

    // Verify common entries
    assert!(result.contains("node_modules/"));
    assert!(result.contains("dist/"));
    assert!(result.contains(".env"));
    assert!(result.contains(".DS_Store"));
}

#[test]
fn test_generate_npmignore() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_npmignore().unwrap();

    // Verify entries
    assert!(result.contains("src/"));
    assert!(result.contains("tsconfig.json"));
    assert!(result.contains("*.test.ts"));
}

#[test]
fn test_generate_biome_json() {
    let backend = TypeScriptBackend::new();
    let config = create_test_config();
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_biome_json().unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["linter"]["enabled"], true);
    assert_eq!(parsed["formatter"]["enabled"], true);
    assert_eq!(parsed["formatter"]["indentWidth"], 2);
    assert_eq!(parsed["formatter"]["lineWidth"], 100);
    assert_eq!(parsed["javascript"]["formatter"]["quoteStyle"], "single");
    assert_eq!(parsed["javascript"]["formatter"]["semicolons"], "always");
}

#[test]
fn test_with_defaults() {
    let backend = TypeScriptBackend::new();
    let generator = ManifestGenerator::with_defaults(backend);

    let result = generator.generate_package_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify defaults
    assert_eq!(parsed["name"], "@octofhir/fhir-r4");
    assert_eq!(parsed["license"], "Apache-2.0");
}

#[test]
fn test_package_json_without_repository() {
    let backend = TypeScriptBackend::new();
    let mut config = create_test_config();
    config.repository_url = None;
    let generator = ManifestGenerator::new(backend, config);

    let result = generator.generate_package_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Repository should not be present
    assert!(parsed["repository"].is_null());
}
