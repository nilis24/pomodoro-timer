# Contributing

Thank you for your interest in contributing to Pomodoro Timer.

This project follows a task-first contribution process. Please do not open a pull request without an approved and assigned GitHub issue.

## Contribution Workflow

1. Create a GitHub issue

   Before starting any work, create an issue using the appropriate configured GitHub issue template:

   - Use the bug report template for reproducible bugs or unexpected behavior.
   - Use the feature request template for new features or improvements.

   The project owner will review the issue, approve it if it fits the project, and assign it to the person who will work on it.

   No task, no pull request.

2. Fork the repository

   Once the issue has been approved and assigned to you, fork the repository to your own GitHub account.

3. Create a branch from `develop`

   Create your working branch from the latest `develop` branch.

   Branch names should follow Conventional Branching and include the issue ID.

   Examples:

   ```text
   feature/123-add-cycle-planning
   fix/124-correct-timer-reset
   docs/125-update-readme
   refactor/126-simplify-plan-calculation
   ```

4. Make your changes

   Keep your changes focused on the assigned issue. Avoid unrelated refactors, formatting churn, or extra features that were not discussed in the issue.

   When possible, add or update tests for behavior changes.

5. Open a pull request to `develop`

   When your work is ready, open a pull request from your branch into `develop`.

   In the pull request description, reference the assigned issue ID (with for example: "Closes #{ISSUE_ID}") and summarize what changed.

6. Review and merge

   The project owner will review the pull request.

   The CI/CD workflow must pass.

   Changes may be requested before the pull request can be merged. Once everything is correct, the project owner will merge it.

## Pull Request Guidelines

- Open pull requests only for approved and assigned issues.
- Target the `develop` branch.
- Keep the scope limited to the assigned task.
- Include the issue ID in the branch name and pull request description.
- Explain any design decisions that may need reviewer attention.

## Code of Conduct

All contributors are expected to follow the project's [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
