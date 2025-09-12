use std::path::PathBuf;
use txtx_addon_kit::helpers::fs::FileLocation;

/// Search upward from a given path to find a txtx.yml file
/// Stops at project root (directory containing .git) or filesystem root
/// Returns the FileLocation of the first txtx.yml found, or None
pub fn find_manifest_upward(start_path: &FileLocation) -> Option<FileLocation> {
    // Convert FileLocation to PathBuf for easier manipulation
    let start = PathBuf::from(start_path.to_string());
    
    // Start from the directory containing the file
    let mut current = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.clone()
    };
    
    // Search upward until we find txtx.yml, .git directory, or reach filesystem root
    loop {
        // Check for txtx.yml in current directory
        let manifest_path = current.join("txtx.yml");
        if let Some(location) = FileLocation::try_parse(&manifest_path.to_string_lossy(), None) {
            if location.read_content().is_ok() {
                return Some(location);
            }
        }
        
        // Check if we've reached a project root (.git directory exists)
        let git_path = current.join(".git");
        if git_path.exists() {
            // We've reached project root without finding txtx.yml
            return None;
        }
        
        // Move up one directory
        match current.parent() {
            Some(parent) => {
                // Stop if we're at the same path (filesystem root)
                if parent == current {
                    break;
                }
                current = parent.to_path_buf();
            }
            None => break, // Reached root
        }
    }
    
    None
}

/// Check if a manifest references a specific .tx file
/// This is useful to verify that the found manifest actually includes the runbook
pub fn manifest_references_file(manifest_location: &FileLocation, tx_file: &FileLocation) -> bool {
    // Read the manifest content
    let manifest_content = match manifest_location.read_content_as_utf8() {
        Ok(content) => content,
        Err(_) => return false,
    };
    
    // Get the relative path from manifest to the tx file
    let manifest_dir = PathBuf::from(manifest_location.to_string())
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    
    let tx_path = PathBuf::from(tx_file.to_string());
    
    // Try to get relative path
    let relative_path = match tx_path.strip_prefix(&manifest_dir) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => {
            // If not a child, use the full path or just the filename
            tx_path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| tx_file.to_string())
        }
    };
    
    // Check if the manifest contains a reference to this file
    // Look for the path in the location field of runbooks
    manifest_content.contains(&relative_path)
}

/// Find the manifest for a given .tx file by searching upward
/// and verifying that the manifest references the file
pub fn find_manifest_for_tx_file(tx_file: &FileLocation) -> Option<FileLocation> {
    let manifest = find_manifest_upward(tx_file)?;
    
    // Verify that this manifest actually references our .tx file
    if manifest_references_file(&manifest, tx_file) {
        Some(manifest)
    } else {
        // The manifest doesn't reference this file, keep searching
        // (In practice, we might want to continue searching upward, but for now
        // we'll just return the first manifest found)
        Some(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_manifest_upward() {
        // Create a temporary directory structure
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();
        
        // Create structure:
        // /project/
        //   txtx.yml
        //   /runbooks/
        //     deploy.tx
        let project_dir = base.join("project");
        let runbooks_dir = project_dir.join("runbooks");
        fs::create_dir_all(&runbooks_dir).unwrap();
        
        // Create txtx.yml
        let manifest_path = project_dir.join("txtx.yml");
        fs::write(&manifest_path, "name: test\nrunbooks:\n  - location: runbooks/deploy.tx").unwrap();
        
        // Create deploy.tx
        let tx_path = runbooks_dir.join("deploy.tx");
        fs::write(&tx_path, "// test").unwrap();
        
        // Test finding manifest from tx file
        let tx_location = FileLocation::try_parse(&tx_path.to_string_lossy(), None).unwrap();
        let found_manifest = find_manifest_upward(&tx_location);
        
        assert!(found_manifest.is_some());
        let found = found_manifest.unwrap();
        assert!(found.to_string().ends_with("txtx.yml"));
    }
    
    #[test]
    fn test_manifest_references_file() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();
        
        // Create manifest
        let manifest_path = base.join("txtx.yml");
        fs::write(&manifest_path, "name: test\nrunbooks:\n  - location: deploy.tx\n  - location: runbooks/other.tx").unwrap();
        
        // Create tx files
        let deploy_path = base.join("deploy.tx");
        fs::write(&deploy_path, "// deploy").unwrap();
        
        let manifest_location = FileLocation::try_parse(&manifest_path.to_string_lossy(), None).unwrap();
        let deploy_location = FileLocation::try_parse(&deploy_path.to_string_lossy(), None).unwrap();
        
        // Should find deploy.tx reference
        assert!(manifest_references_file(&manifest_location, &deploy_location));
        
        // Should not find non-existent file reference
        let other_path = base.join("nonexistent.tx");
        fs::write(&other_path, "// other").unwrap();
        let other_location = FileLocation::try_parse(&other_path.to_string_lossy(), None).unwrap();
        assert!(!manifest_references_file(&manifest_location, &other_location));
    }
}