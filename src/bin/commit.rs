use dialoguer::{Confirm, Editor, Input, Select, theme::ColorfulTheme};
use std::process::Command;

#[derive(Debug)]
struct CommitInfo {
    commit_type: String,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    breaking: bool,
    issues: Vec<String>,
}

impl CommitInfo {
    fn to_message(&self) -> String {
        let mut message = String::new();

        // Type und Scope
        message.push_str(&self.commit_type);
        if let Some(scope) = &self.scope {
            message.push_str(&format!("({})", scope));
        }
        if self.breaking {
            message.push('!');
        }
        message.push_str(": ");
        message.push_str(&self.description);

        // Body
        if let Some(body) = &self.body {
            if !body.trim().is_empty() {
                message.push_str("\n\n");
                message.push_str(body.trim());
            }
        }

        // Breaking Change Notice
        if self.breaking {
            message.push_str("\n\nBREAKING CHANGE: ");
            message.push_str(&self.description);
        }

        // Issues
        if !self.issues.is_empty() {
            message.push_str("\n\n");
            for issue in &self.issues {
                message.push_str(&format!("Closes #{}\n", issue));
            }
        }

        message
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    println!("🚀 Semantic Commit Generator\n");

    // 1. Commit Type
    let types = vec![
        ("feat", "✨ Neue Features"),
        ("fix", "🐛 Bug Fix"),
        ("docs", "📚 Dokumentation"),
        ("style", "💄 Code Style (Formatting)"),
        ("refactor", "♻️  Code Refactoring"),
        ("perf", "⚡ Performance Verbesserung"),
        ("test", "✅ Tests hinzufügen/ändern"),
        ("build", "🔧 Build System oder Dependencies"),
        ("ci", "👷 CI/CD Änderungen"),
        ("chore", "🔨 Maintenance Tasks"),
        ("revert", "⏪ Revert eines Commits"),
    ];

    let type_selection = Select::with_theme(&theme)
        .with_prompt("Commit Type")
        .items(&types.iter().map(|(_, desc)| *desc).collect::<Vec<_>>())
        .default(0)
        .interact()?;

    let commit_type = types[type_selection].0.to_string();

    // 2. Scope (optional)
    let scope: String = Input::with_theme(&theme)
        .with_prompt("Scope (optional, z.B. api, auth, ui)")
        .allow_empty(true)
        .interact_text()?;

    let scope = if scope.trim().is_empty() {
        None
    } else {
        Some(scope.trim().to_string())
    };

    // 3. Description
    let description: String = Input::with_theme(&theme)
        .with_prompt("Kurze Beschreibung (imperative Form)")
        .interact_text()?;

    // 4. Body (optional)
    let add_body = Confirm::with_theme(&theme)
        .with_prompt("Möchtest du eine längere Beschreibung hinzufügen?")
        .default(false)
        .interact()?;

    let body = if add_body {
        if let Some(text) = Editor::new().edit("")? {
            Some(text)
        } else {
            None
        }
    } else {
        None
    };

    // 5. Breaking Change
    let breaking = Confirm::with_theme(&theme)
        .with_prompt("Ist das ein Breaking Change?")
        .default(false)
        .interact()?;

    // 6. Issues
    let add_issues = Confirm::with_theme(&theme)
        .with_prompt("Issue-Nummern hinzufügen?")
        .default(false)
        .interact()?;

    let mut issues = Vec::new();
    if add_issues {
        loop {
            let issue: String = Input::with_theme(&theme)
                .with_prompt("Issue Nummer (leer für fertig)")
                .allow_empty(true)
                .interact_text()?;

            if issue.trim().is_empty() {
                break;
            }

            // Remove # if present
            let issue = issue.trim().trim_start_matches('#').to_string();
            issues.push(issue);

            if !Confirm::with_theme(&theme)
                .with_prompt("Weitere Issue hinzufügen?")
                .default(false)
                .interact()?
            {
                break;
            }
        }
    }

    // Commit Info erstellen
    let commit_info = CommitInfo {
        commit_type,
        scope,
        description,
        body,
        breaking,
        issues,
    };

    // Preview
    println!("\n📝 Commit Message Preview:\n");
    println!("─────────────────────────────────────");
    println!("{}", commit_info.to_message());
    println!("─────────────────────────────────────\n");

    // Bestätigung
    let confirm = Confirm::with_theme(&theme)
        .with_prompt("Commit erstellen?")
        .default(true)
        .interact()?;

    if !confirm {
        println!("❌ Abgebrochen");
        return Ok(());
    }

    // Git Commit ausführen
    let message = commit_info.to_message();
    let output = Command::new("git")
        .args(&["commit", "-m", &message])
        .output()?;

    if output.status.success() {
        println!("✅ Commit erfolgreich erstellt!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Git Fehler:\n{}", error);
        std::process::exit(1);
    }

    Ok(())
}
