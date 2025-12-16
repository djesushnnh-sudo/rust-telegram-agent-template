# GitHub Issues SOP - Using Kiro IDE and AI Tools

A step-by-step guide for handling GitHub issues from identification to pull request completion.

## Overview

This SOP covers the complete workflow for contributing to GitHub projects using Kiro IDE and AI assistance. Follow these steps to efficiently identify, reproduce, fix, and submit solutions for GitHub issues.

## Prerequisites

- Kiro IDE installed and configured
- Git configured with your GitHub credentials
- Access to AI tools (ChatGPT, Claude, etc.)
- Basic understanding of the command line

## Step 1: Identify and Understand the Issue

### 1.1 Find the Issue
- **Get tagged** in a GitHub issue, or
- **Browse the repository** issues tab to find unassigned issues
- **Look for labels** like "good first issue", "help wanted", or "bug"

### 1.2 Read the Project Context
Before diving into the issue:
1. **Navigate to the main repository**
2. **Read the README.md thoroughly** - understand:
   - What the project does
   - How it's structured
   - Key technologies used
   - Setup requirements
3. **Check for additional docs** in `/docs` folder or wiki

### 1.3 Analyze the Issue
With project context in mind:
1. **Read the issue description carefully**
2. **Look at any screenshots or error logs**
3. **Check the comments** for additional context
4. **Note any reproduction steps** provided
5. **Identify the expected vs actual behavior**

## Step 2: Set Up Local Development

### 2.1 Clone the Repository
```bash
# In Kiro's integrated terminal
git clone [repository-url]
cd [project-name]
```

### 2.2 Create a Branch for the Issue
**Important: Do this BEFORE making any changes**

Option A - Using GitHub UI:
1. Go to the issue page on GitHub
2. Look for "Development" sidebar
3. Click "Create a branch for this issue"
4. Copy the branch name provided

Option B - Manual branch creation:
```bash
git checkout -b fix/issue-[number]-brief-description
# Example: git checkout -b fix/issue-123-login-button-styling
```

### 2.3 Switch to Your Branch Locally
```bash
git fetch origin
git checkout [branch-name]
```

## Step 3: Get the Project Running

### 3.1 Look for Run Instructions
Check these locations in order:
1. **README.md** - look for "Getting Started" or "Development" sections
2. **package.json** - for Node.js projects, check the "scripts" section
3. **.vscode/launch.json** - for projects with VS Code debug configs
4. **start.sh** or similar shell scripts
5. **Makefile** - for projects using Make
6. **docker-compose.yml** - for containerized setups

### 3.2 Try to Run the Project
Common commands to try:
```bash
# Node.js projects
npm install && npm start
npm run dev

# Python projects
pip install -r requirements.txt
python main.py
python manage.py runserver  # Django

# Rust projects
cargo run

# Go projects
go run main.go

# Docker projects
docker-compose up
```

### 3.3 Handle Startup Failures
**Expected**: The project likely won't start on first try.

1. **Read the terminal output carefully**
2. **Copy the entire error log** (from command to exit)
3. **Look for obvious issues**:
   - Missing environment variables
   - Missing dependencies
   - Port conflicts
   - Database connection issues

## Step 4: Debug with AI Assistance

### 4.1 Prepare Your AI Prompt
Create a comprehensive prompt:
```
I'm trying to run [project-name] locally and getting this error:

[Command I ran]: 
[Full terminal output from command to error]

The project is a [brief description from README]. 
Can you help me understand what's wrong and how to fix it?
```

### 4.2 Common Solutions
Based on AI feedback, you might need to:
- **Set up environment variables** (create `.env` file)
- **Install missing dependencies**
- **Start required services** (database, Redis, etc.)
- **Use different Node/Python versions**
- **Contact the project maintainer** for secrets/credentials

### 4.3 Iterate Until Running
- Apply AI suggestions one at a time
- Test after each change
- If stuck after 3-4 attempts, ask for help in the issue comments

## Step 5: Reproduce the Bug

### 5.1 Follow Reproduction Steps
Once the project runs:
1. **Follow the exact steps** described in the issue
2. **Take screenshots** if it's a visual bug
3. **Copy any error messages** from browser console or terminal
4. **Note the exact behavior** you observe

### 5.2 Confirm the Issue Exists
- **Verify you can reproduce** the problem consistently
- **Document any variations** from the reported behavior
- **Note your environment** (OS, browser, versions) if relevant

### 5.3 If You Can't Reproduce
- **Comment on the issue** with your findings
- **Ask for clarification** or additional reproduction steps
- **Provide details** about your setup and what you tried

## Step 6: Develop the Fix

### 6.1 Analyze the Problem with AI
Create a detailed prompt:
```
I'm working on fixing this GitHub issue: [link to issue]

The problem is: [description of the bug]
I can reproduce it by: [reproduction steps]
The error/behavior I see: [specific details]

Here's the relevant code I think might be involved:
[paste relevant code sections]

Can you help me understand what's causing this and suggest a fix?
```

### 6.2 Implement the Solution
- **Make minimal changes** - fix only what's broken
- **Follow the project's coding style**
- **Add comments** explaining your changes if complex
- **Test your fix** by reproducing the original issue

### 6.3 Verify the Fix
1. **Restart the application**
2. **Test the original reproduction steps**
3. **Verify the issue is resolved**
4. **Test related functionality** to ensure no regressions
5. **Run any existing tests** if available

## Step 7: Review and Commit Changes

### 7.1 Review Your Changes
Read through the changed lines of code, either through the built is features in your IDE or by running:

```bash
git status
git diff
```


Check that:
- **All changes relate to the issue** you're fixing
- **No unintended files** are modified
- **No sensitive information** (passwords, keys) is included
- **Code changes make sense** for the problem being solved

### 7.2 Use AI to Review Changes
If unsure about any changes or you don't know how exactally the changed code relates to the fix. Start a freash chat and ask:
```
I made these changes to fix [issue description]:

[paste git diff output]

Can you review these changes and confirm they:
1. Actually fix the reported issue
2. Don't introduce new problems
3. Follow good coding practices
```

### 7.3 Commit Your Changes
```bash
git add .
git commit -m "Fix: [brief description of what you fixed]

Resolves #[issue-number]

- [bullet point of main change]
- [bullet point of other changes if any]"
```

Example:
```bash
git commit -m "Fix: Login button styling on mobile devices

Resolves #123

- Added responsive CSS for login button
- Fixed button text overflow on small screens"
```

## Step 8: Create Pull Request

### 8.1 Push Your Branch
```bash
git push origin [your-branch-name]
```

### 8.2 Open Pull Request
1. **Go to the repository** on GitHub
2. **Click "Compare & pull request"** (should appear automatically)
3. **Fill out the PR template** if one exists
4. **Reference the issue** with "Fixes #[issue-number]"
5. **Describe your changes** clearly
6. **Add screenshots** if it's a visual fix

### 8.3 PR Description Template
```markdown
## Description
Brief description of what this PR does.

## Fixes
Fixes #[issue-number]

## Changes Made
- [List the main changes]
- [Be specific about what was modified]

## Testing
- [x] Reproduced the original issue
- [x] Verified the fix resolves the issue
- [x] Tested related functionality
- [ ] Added/updated tests (if applicable)

## Screenshots (if applicable)
Before: [screenshot]
After: [screenshot]
```

## Step 9: Handle Code Review Feedback

### 9.1 Wait for Automated Reviews
- **Gemini Code Review** or similar tools will comment within a few minutes
- **CI/CD checks** may run automatically

### 9.2 Address Critical Feedback
1. **Copy all critical/high-priority comments** from the review
2. **Paste them into Kiro** or your AI tool
3. **Ask for help implementing** the suggested changes:

```
I got this code review feedback on my pull request:

[paste review comments]

Can you help me implement these changes? Here's my current code:
[paste relevant code sections]
```

### 9.3 Make Additional Commits
```bash
# Make the suggested changes
git add .
git commit -m "Address code review feedback

- [specific change made]
- [other changes]"
git push origin [your-branch-name]
```

**Note**: New commits automatically update the existing pull request.

## Step 10: Follow Up

### 10.1 Monitor Your PR
- **Check for additional comments** from maintainers
- **Respond to questions** promptly
- **Make requested changes** quickly

### 10.2 If Stuck After Multiple Attempts
Create a comprehensive summary:
```markdown
## Summary of Attempts

I've been working on issue #[number] and have tried the following approaches:

### Attempt 1: [Brief description]
- What I tried: [details]
- Result: [what happened]
- Why it didn't work: [analysis]

### Attempt 2: [Brief description]
- What I tried: [details]
- Result: [what happened]
- Why it didn't work: [analysis]

### Current Status
- [What's working]
- [What's still broken]
- [Specific help needed]

I'd appreciate guidance on [specific question] or if someone else would like to take over this issue.
```

Post this as a comment on the original issue.

## Tips for Success

### General Best Practices
- **Start with simple issues** to learn the workflow
- **Read error messages carefully** before asking for help
- **Make small, focused changes** rather than large refactors
- **Test thoroughly** before submitting
- **Be patient** - some projects take time to review PRs

### Working with AI Tools
- **Be specific** in your prompts - include context, error messages, and code
- **Ask for explanations** not just solutions
- **Verify AI suggestions** by testing them
- **Use AI to review your own work** before submitting

### Communication
- **Be polite and professional** in all GitHub interactions
- **Ask questions** if anything is unclear
- **Provide context** when asking for help
- **Thank reviewers** for their time and feedback

## Troubleshooting Common Issues

### "I can't get the project to run"
1. Check if you need specific versions of tools (Node, Python, etc.)
2. Look for setup scripts or installation guides
3. Ask in the issue comments for help with environment setup
4. Try using Docker if available

### "I can't reproduce the issue"
1. Make sure you're using the same browser/OS as reported
2. Check if you need specific data or user accounts
3. Ask the issue reporter for more details
4. Try different approaches to trigger the bug

### "My fix works but breaks other things"
1. Run the project's test suite if available
2. Test the main user flows manually
3. Ask AI to review your changes for potential side effects
4. Consider a more targeted fix

### "Code review feedback is confusing"
1. Ask for clarification in the PR comments
2. Use AI to help understand the feedback
3. Look at other PRs in the project for examples
4. Don't be afraid to ask questions

## Conclusion

This workflow gets easier with practice. Start with small issues to build confidence, and don't hesitate to ask for help when stuck. The open source community is generally very supportive of new contributors who show effort and follow good practices.

Remember: Every expert was once a beginner. Focus on learning and improving with each issue you tackle.
