#[cfg(test)]
pub mod tests {
    use rand::seq::IndexedRandom;
    use std::{fs::create_dir_all, process::Command};
    use tempfile::TempDir;

    /// Helper function to create a git repository with a specific branch checked out
    fn create_git_repo_with_branch(
        path: &std::path::PathBuf,
        branch_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize git repo
        use std::process::Command;

        {
            Command::new("git")
                .args(["init"])
                .current_dir(path)
                .spawn()
                .expect("ls command failed to start");
        }

        {
            // Configure git user
            Command::new("git")
                .args(["config", "user.email", "test@example.com"])
                .current_dir(path)
                .spawn()
                .expect("ls command failed to start");
        }

        {
            Command::new("git")
                .args(["config", "user.name", "Test User"])
                .current_dir(path)
                .spawn()
                .expect("ls command failed to start");
        }

        // Create initial commit on main
        std::fs::write(path.join("README.md"), "# Test Repo").expect("write 1 wrong");
        {
            Command::new("git")
                .args(["add", "."])
                .current_dir(path)
                .spawn()
                .expect("ls command failed to start");
        }

        {
            Command::new("git")
                .args(["commit", "-m", "Initial commit"])
                .current_dir(path)
                .spawn()
                .expect("ls command failed to start");
        }
        // Create and checkout branch if not main/master

        if branch_name != "main" && branch_name != "master" {
            {
                Command::new("git")
                    .args(["checkout", "-b", branch_name])
                    .current_dir(path)
                    .spawn()
                    .expect("ls command failed to start");
            }
            // Add a commit on the new branch

            std::fs::write(path.join(&format!("{}.txt", branch_name)), "test")
                .expect(format!("write 1 wrong ({})", branch_name.to_string().as_str()).as_str());
            {
                Command::new("git")
                    .args(["add", "."])
                    .current_dir(path)
                    .spawn()
                    .expect("ls command failed to start");
            }
            {
                Command::new("git")
                    .args(["commit", "-m", &format!("Add {} file", branch_name)])
                    .current_dir(path)
                    .spawn()
                    .expect("ls command failed to start");
            }
        }

        println!("OK");

        Ok(())
    }

    #[test]
    fn test_glimpse_shows_all_branches() {
        // Create temporary directory for test
        let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
        println!("PATH: {:?}", temp_dir.path().to_str());
        let base_path = temp_dir.path();

        // Define test branches
        let branches = vec![
            ("repo1", "user-auth"),
            ("repo2", "memory-leak"),
            ("repo3", "develop"),
            ("repo4", "v2026.2.14"),
        ];

        // Create 4 directories with different branches
        for (dir_name, branch_name) in &branches {
            let repo_path = base_path.join(dir_name);
            std::fs::create_dir_all(&repo_path).expect("Failed to create repo directory");

            create_git_repo_with_branch(&repo_path, branch_name)
                .expect(&format!("Failed to create git repo in {}", dir_name));

            println!("✓ Created {} with branch {}", dir_name, branch_name);
        }

        // Run glimpse command
        let output = Command::new("cargo")
            .args(["run", "--bin", "glimpse", "--", base_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute glimpse");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("=== STDOUT ===");
        println!("{}", stdout);
        println!("=== STDERR ===");
        println!("{}", stderr);

        // Assert that the command succeeded
        assert!(output.status.success(), "glimpse command failed");

        // Check that all branches are present in output
        for (dir_name, branch_name) in &branches {
            assert!(
                stdout.contains(branch_name)
                    || stdout.contains(&format!("{}/{}", dir_name, branch_name)),
                "Output should contain branch '{}' from directory '{}'",
                branch_name,
                dir_name
            );
        }

        // Additional checks
        assert!(stdout.contains("repo1"), "Output should contain repo1");
        assert!(stdout.contains("repo2"), "Output should contain repo2");
        assert!(stdout.contains("repo3"), "Output should contain repo3");
        assert!(stdout.contains("repo4"), "Output should contain repo4");

        println!("✅ All branches found in output!");
    }

    #[test]
    fn test_glimpse_with_random_branches() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let base_path = temp_dir.path();

        // Pool of possible branch names
        let possible_branches = vec![
            "main",
            "develop",
            "new-ui",
            "api-refactor",
            "login-error",
            "crash-on-startup",
            "v1.0.0",
            "v2.0.0",
            "security-patch",
            "ai-integration",
        ];

        let mut rng = rand::rng();
        let mut selected_branches = Vec::new();

        // Create 4 repos with random branches
        for i in 1..=4 {
            let dir_name = format!("repo{}", i);
            let branch_name = possible_branches.choose(&mut rng).unwrap();

            let repo_path = base_path.join(&dir_name);
            create_dir_all(&repo_path).expect("Failed to create repo directory");

            create_git_repo_with_branch(&repo_path, branch_name)
                .expect(&format!("Failed to create git repo in {}", dir_name));

            selected_branches.push((dir_name.clone(), branch_name.to_string()));
            println!("✓ Created {} with random branch {}", dir_name, branch_name);
        }

        // Run glimpse
        let output = Command::new("cargo")
            .args(["run", "--bin", "glimpse", "--", base_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute glimpse");

        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("=== Output ===");
        println!("{}", stdout);

        assert!(output.status.success(), "glimpse command failed");

        // Verify all selected branches appear in output
        for (dir_name, branch_name) in &selected_branches {
            assert!(
                stdout.contains(branch_name),
                "Output should contain branch '{}' from directory '{}'",
                branch_name,
                dir_name
            );
        }

        println!(
            "✅ All {} random branches found in output!",
            selected_branches.len()
        );
    }
}
