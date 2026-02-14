# Setting Up Auto-Review Assignment with Gemini Bot

This guide explains how to automatically notify about Gemini AI code reviews when pull requests are opened.

## 📋 Overview

When someone opens a PR, the following will happen automatically:
1. ✅ Gemini AI bot reviews the code automatically (via GitHub Actions)
2. ✅ Welcome comment notifies contributor about the AI review
3. ✅ Human reviewers are assigned (if configured)
4. ✅ Review results are posted as comments

## ⚠️ Important Note: Bots Cannot Be Reviewers

**GitHub limitation:** Bots (including `gemini-code-assist[bot]`) cannot be added as official "reviewers" on pull requests. However, the Gemini bot will still:
- ✅ Automatically review every PR via GitHub Actions
- ✅ Post review comments with feedback
- ✅ Appear in the PR timeline

The bot works through the `gemini-review.yml` workflow, not through GitHub's reviewer system.

## 🚀 Setup Instructions

### Prerequisites

Make sure you have the Gemini review workflow set up first:

1. **Gemini API Key configured:**
   ```bash
   # Add GEMINI_API_KEY to GitHub Secrets
   # Repository Settings → Secrets and variables → Actions → New secret
   ```

2. **Gemini review workflow in place:**
   ```bash
   # File should exist: .github/workflows/gemini-review.yml
   ls .github/workflows/gemini-review.yml
   ```

If you don't have the Gemini workflow yet, see [GEMINI_SETUP.md](GEMINI_SETUP.md) first.

### Option 1: Auto-Assignment Workflow (Recommended)

This adds a comment notifying about Gemini review and assigns human reviewers.

1. **Copy the workflow file:**
   ```bash
   mkdir -p .github/workflows
   cp auto-assign-reviewers.yml .github/workflows/
   ```

2. **The workflow automatically:**
   - Posts a welcome comment mentioning Gemini AI will review
   - Adds a contributor checklist
   - Assigns human reviewers (if you configure them)

3. **Edit the workflow** to add human reviewers (optional):
   ```yaml
   # In .github/workflows/auto-assign-reviewers.yml, line ~25
   const reviewers = [
     'yourusername',      # Replace with your GitHub username
     'teammate1',         # Add team members
     'teammate2',
   ];
   ```

   **Note:** Don't add `gemini-code-assist[bot]` here - it won't work. The Gemini bot reviews via the `gemini-review.yml` workflow automatically.

4. **Commit and push:**
   ```bash
   git add .github/workflows/auto-assign-reviewers.yml
   git commit -m "ci: add auto-reviewer assignment and Gemini notification"
   git push
   ```

**Result:** When a PR opens, contributors see:
```
👋 Thank you for your contribution!

🤖 Gemini AI Code Review will automatically review this PR shortly.

📋 Checklist:
- [ ] All tests pass
- [ ] Code follows project style guidelines
...
```

### Option 2: CODEOWNERS File

The CODEOWNERS file automatically requests reviews from specific users based on which files are changed.

**Note:** CODEOWNERS can only assign human users and teams, NOT bots. The Gemini bot will still review automatically via GitHub Actions.

1. **Copy the CODEOWNERS file:**
   ```bash
   cp CODEOWNERS .github/CODEOWNERS
   ```

2. **Edit CODEOWNERS** to add your human reviewers:
   ```
   # Review all changes
   * @yourusername
   
   # Review Rust code
   *.rs @yourusername @rust-expert
   
   # Review tests
   tests/ @yourusername @qa-team
   
   # NOTE: Do NOT add bots here - they won't work
   # ❌ * gemini-code-assist[bot]  # This will NOT work
   # ✅ * @yourusername             # This works
   ```

3. **Enable CODEOWNERS in branch protection (optional):**
   - Go to Settings → Branches → Branch protection rules
   - Edit rule for `main` branch
   - Check "Require review from Code Owners"

4. **Commit and push:**
   ```bash
   git add .github/CODEOWNERS
   git commit -m "ci: add CODEOWNERS for automatic review assignment"
   git push
   ```

**Result:** Human reviewers are auto-requested based on which files are changed.

### Option 3: Enhanced PR Template

Use the enhanced PR template that includes Gemini bot information.

1. **Copy the template:**
   ```bash
   mkdir -p .github
   cp PULL_REQUEST_TEMPLATE_ENHANCED.md .github/PULL_REQUEST_TEMPLATE.md
   ```

2. **Commit and push:**
   ```bash
   git add .github/PULL_REQUEST_TEMPLATE.md
   git commit -m "docs: add enhanced PR template with AI review info"
   git push
   ```

### Combining All Options (Recommended)

For the best experience, use all three:

```bash
# Setup directory
mkdir -p .github/workflows

# Copy files
cp auto-assign-reviewers.yml .github/workflows/
cp CODEOWNERS .github/
cp PULL_REQUEST_TEMPLATE_ENHANCED.md .github/PULL_REQUEST_TEMPLATE.md

# Make sure gemini-review.yml is also present
ls .github/workflows/gemini-review.yml  # Should exist

# Edit as needed
# - auto-assign-reviewers.yml: Add human reviewers (line ~25)
# - CODEOWNERS: Set file-specific human owners
# - PR template: Customize as needed
# - gemini-review.yml: Should already be configured with GEMINI_API_KEY

# Commit
git add .github/
git commit -m "ci: setup auto-review assignment and PR template"
git push
```

**Important files needed:**
- ✅ `.github/workflows/gemini-review.yml` - Makes Gemini bot review automatically
- ✅ `.github/workflows/auto-assign-reviewers.yml` - Posts notification and assigns humans
- ✅ `.github/CODEOWNERS` - Auto-assigns human reviewers based on files
- ✅ `.github/PULL_REQUEST_TEMPLATE.md` - Shows contributor checklist

## 🔧 Configuration Details

### Auto-Assignment Workflow

**Triggers on:**
- Pull request opened
- Pull request marked as ready for review (from draft)

**What it does:**
1. Assigns configured human reviewers
2. Posts welcome comment mentioning Gemini review
3. Adds checklist for contributors

**Customization:**
```javascript
// In auto-assign-reviewers.yml
const reviewers = [
  'maintainer1',
  'maintainer2',
];

// Modify the welcome comment
const comment = `Your custom message here...`;
```

### CODEOWNERS Configuration

**Syntax:**
```
# Pattern-based assignment
*.rs @rust-team
src/core/ @core-team
docs/ @docs-team

# Specific files
src/main.rs @lead-dev
Cargo.toml @build-team

# Using teams (requires GitHub organization)
* @org/maintainers
src/ @org/core-team
```

**Features:**
- Automatic review requests based on changed files
- Can require code owner approval before merge
- Supports GitHub teams and individual users

**Limitations:**
- ❌ Cannot assign bots (like Gemini) as code owners - this is a GitHub limitation
- ❌ Bots don't appear in the "Reviewers" list (they comment instead)
- ✅ Gemini bot still reviews automatically via `gemini-review.yml` workflow
- ✅ Use `auto-assign-reviewers.yml` workflow to notify about bot reviews
- ✅ Human reviewers work normally in CODEOWNERS

## 📊 How It Works Together

```
┌─────────────────────────────────────────────────────────────┐
│                     PR Opened/Updated                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
         ┌─────────────┼─────────────┐
         │             │             │
         ▼             ▼             ▼
┌────────────┐  ┌────────────┐  ┌──────────────┐
│  CODEOWNERS│  │Auto-Assign │  │Gemini Review │
│   Checks   │  │  Workflow  │  │   Workflow   │
│   Files    │  │    Runs    │  │     Runs     │
└─────┬──────┘  └─────┬──────┘  └──────┬───────┘
      │               │                 │
      │               │                 │
      ▼               ▼                 ▼
┌──────────┐  ┌────────────┐  ┌──────────────┐
│ Requests │  │   Posts    │  │  Analyzes    │
│  Human   │  │  Welcome   │  │     Code     │
│ Reviewers│  │  Comment   │  │              │
└─────┬────┘  └─────┬──────┘  └──────┬───────┘
      │             │                 │
      │             │                 │
      └─────────────┼─────────────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │   PR Timeline Shows:  │
        │                       │
        │ ✅ Welcome Comment    │
        │ ✅ Human Reviewers    │
        │ ✅ Gemini AI Comment  │
        │ ✅ PR Template        │
        └───────────────────────┘

Note: Gemini bot does NOT appear in "Reviewers" section
      It appears as a comment from gemini-code-assist[bot]
```

**Key Points:**
1. **Gemini review** happens via GitHub Actions (gemini-review.yml)
2. **Not a GitHub reviewer** - appears as a bot comment instead
3. **Human reviewers** assigned via CODEOWNERS or auto-assign workflow
4. **All automatic** - no manual intervention needed

## 🎯 Best Practices

### For Auto-Assignment

1. **Keep reviewer list small** (2-3 people)
2. **Use teams** instead of individuals when possible
3. **Don't over-notify** - too many reviewers = slower reviews
4. **Consider time zones** when assigning reviewers

### For CODEOWNERS

1. **Start broad, get specific:**
   ```
   * @default-reviewer
   src/security/ @security-team
   ```

2. **Balance coverage and noise:**
   - Too specific = too many notifications
   - Too broad = missing important reviews

3. **Document ownership:**
   ```
   # Security Team owns auth code
   src/auth/ @security-team
   
   # Frontend Team owns UI
   src/ui/ @frontend-team
   ```

4. **Test your patterns:**
   ```bash
   # See who would be requested for a file
   gh api repos/:owner/:repo/codeowners/errors
   ```

### For PR Template

1. **Keep it concise** - long templates get ignored
2. **Use checkboxes** - makes it interactive
3. **Explain the AI review** - set expectations
4. **Link to guidelines** - don't repeat everything

## ⚙️ Advanced Configuration

### Conditional Assignment

Assign different reviewers based on PR labels:

```yaml
# In auto-assign-reviewers.yml
- name: Assign based on labels
  uses: actions/github-script@v6
  with:
    script: |
      const labels = context.payload.pull_request.labels.map(l => l.name);
      
      let reviewers = [];
      if (labels.includes('security')) {
        reviewers.push('security-expert');
      }
      if (labels.includes('performance')) {
        reviewers.push('perf-expert');
      }
      
      if (reviewers.length > 0) {
        await github.rest.pulls.requestReviewers({
          owner: context.repo.owner,
          repo: context.repo.repo,
          pull_number: context.payload.pull_request.number,
          reviewers: reviewers
        });
      }
```

### Team Rotation

Rotate reviewers from a team:

```yaml
- name: Assign rotating reviewer
  uses: kentaro-m/auto-assign-action@v1.2.1
  with:
    configuration-path: '.github/auto-assign.yml'
```

`.github/auto-assign.yml`:
```yaml
addReviewers: true
addAssignees: false
reviewers:
  - reviewer1
  - reviewer2
  - reviewer3
numberOfReviewers: 1  # Assign only 1 person
```

## 🐛 Troubleshooting

### "Why isn't Gemini showing as a reviewer?"

**Answer:** This is expected! Gemini cannot be added as an official GitHub reviewer because:
- GitHub's reviewer API only works with users and teams
- Bots/GitHub Apps cannot be assigned as reviewers
- This is a GitHub platform limitation, not something we can fix

**What Gemini actually does:**
- ✅ Reviews code automatically via GitHub Actions
- ✅ Posts comments on the PR
- ✅ Appears in the timeline as `gemini-code-assist[bot]`
- ✅ Provides the same feedback (just not in the "Reviewers" box)

**What you'll see:**
```
┌─────────────────────────────┐
│ Reviewers                   │
│ @yourusername   ✓ Approved  │  ← Human reviewer
│ @teammate      ⏳ Requested │  ← Human reviewer
└─────────────────────────────┘

┌─────────────────────────────┐
│ Timeline                    │
│ gemini-code-assist[bot]     │  ← Gemini bot comment
│ commented 2 minutes ago     │
│                             │
│ 🤖 Gemini AI Code Review    │
│ [Review feedback here...]   │
└─────────────────────────────┘
```

### Reviewers Not Being Assigned

**Check:**
1. Workflow has correct permissions:
   ```yaml
   permissions:
     pull-requests: write
   ```

2. Users have access to the repository
3. PR is not from a fork (security limitation)
4. Reviewer exists and is not the PR author
5. You're adding HUMAN reviewers, not bots

**Example - What works and what doesn't:**
```yaml
# ✅ This works - human reviewer
const reviewers = ['yourusername', 'teammate'];

# ❌ This does NOT work - bot
const reviewers = ['gemini-code-assist[bot]'];  // Will fail

# ✅ This works - mix of humans
const reviewers = ['alice', 'bob', 'charlie'];
```

### CODEOWNERS Not Working

**Check:**
1. File is at `.github/CODEOWNERS` (correct location)
2. Syntax is correct (no typos in usernames)
3. Branch protection requires code owner review
4. CODEOWNERS errors: 
   ```bash
   gh api repos/:owner/:repo/codeowners/errors
   ```

### Bot Not Commenting

**Check:**
1. `GEMINI_API_KEY` secret is set
2. Gemini review workflow is enabled
3. PR contains reviewable files (code, not images)
4. API quota not exceeded

## 📚 Resources

- [GitHub CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- [Auto-assign Action](https://github.com/marketplace/actions/auto-assign-action)
- [GitHub Actions Script](https://github.com/actions/github-script)
- [Pull Request Templates](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/creating-a-pull-request-template-for-your-repository)

## 🎉 Result

After setup, when someone opens a PR:

1. ✅ **PR Template** guides contributor through checklist
2. ✅ **Auto-assign workflow** posts welcome comment:
   ```
   👋 Thank you for your contribution!
   🤖 Gemini AI Code Review will automatically review this PR shortly.
   ```
3. ✅ **CODEOWNERS** assigns human reviewers
4. ✅ **Gemini workflow** reviews code and posts feedback comment
5. ✅ **Human reviewers** get notified

**Visual Example:**

```
Pull Request #42: feat: add new feature
════════════════════════════════════════

┌─ Reviewers ────────────────┐
│ @maintainer   ⏳ Requested │  ← From CODEOWNERS
│ @teammate     ⏳ Requested │  ← From auto-assign
└────────────────────────────┘

┌─ Timeline ─────────────────┐
│ 🤖 auto-assign-bot         │
│ Welcome comment posted     │
│                            │
│ 🤖 gemini-code-assist[bot] │  ← AI Review
│ Posted review              │
│                            │
│ 👤 @maintainer             │
│ Approved changes           │
└────────────────────────────┘
```

This creates a smooth, professional contribution experience! 🚀

**Remember:** 
- Gemini is NOT in the "Reviewers" box (GitHub limitation)
- Gemini DOES comment automatically (via GitHub Actions)
- Human reviewers ARE in the "Reviewers" box (as expected)

---

**Questions?** Open an issue or check the [Contributing Guide](CONTRIBUTING.md)