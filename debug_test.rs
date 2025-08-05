// THIS IS A SCRATCH FILE
use std::path::PathBuf;
use tempfile::TempDir;
use swissarmyhammer::workflow::{MemoryWorkflowStorage, WorkflowResolver};
use swissarmyhammer_cli::validate::{Validator, ValidationResult};

fn main() {
    // Create a temporary test environment
    let temp_dir = TempDir::new().unwrap();
    let current_dir = temp_dir.path();

    // Create workflows in standard locations
    let local_dir = current_dir.join(".swissarmyhammer").join("workflows");
    std::fs::create_dir_all(&local_dir).unwrap();

    // Create a valid workflow
    std::fs::write(
        local_dir.join("test-workflow.md"),
        r#"---
name: test-workflow
description: Test workflow for validation
---

stateDiagram-v2
    [*] --> Start
    Start --> Process
    Process --> End
    End --> [*]
"#,
    )
    .unwrap();

    let original_dir = std::env::current_dir().ok();
    std::env::set_current_dir(current_dir).unwrap();

    // Run validation
    let mut validator = Validator::new(false);
    let mut validation_result = ValidationResult::new();
    let _ = validator.validate_all_workflows(&mut validation_result);

    println!("Validation files_checked: {}", validation_result.files_checked);

    // Load workflows using WorkflowResolver (same as flow list)
    let mut storage = MemoryWorkflowStorage::new();
    let mut resolver = WorkflowResolver::new();
    let flow_res = resolver.load_all_workflows(&mut storage);

    if flow_res.is_ok() {
        let flow_workflows = storage.list_workflows().unwrap();
        println!("Flow list found workflows: {}", flow_workflows.len());
        
        for wf in &flow_workflows {
            println!("  - {}", wf.name.as_str());
        }
    }

    // Restore original directory
    if let Some(original) = original_dir {
        let _ = std::env::set_current_dir(original);
    }
}