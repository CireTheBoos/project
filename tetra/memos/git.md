# Git

## Branches

```bash
# Basics

git branch                    # List branches
git branch feature/foo        # Create branch
git branch -d feature/foo     # Delete branch
git switch feature/foo        # Switch to branch

# Merging & Rebasing

git merge feature/foo         # Merge branch into current one
git rebase main               # NOT SURE. Reapply commits on top of another base
```

## Stage

```bash
# Status

git status                    # View changes

# Stage & Unstage

git add file.rs               # Stage file
git add .                     # Stage all changes
git restore --staged file.rs  # Unstage file
git restore --staged .        # Unstage all changes

# Discard local changes

git restore file.rs           # Discard file changes
git restore .                 # Discard all changes
```

## Commits

```bash
# Commit

git commit                    # Commit staged changes
git commit --amend            # Replace last commit

# Logs & Diffs

git log                       # Full commit history
git log --oneline --graph     # Compact, visual history
git diff                      # NOT SURE. Unstaged file changes
git diff --staged             # NOT SURE. Changes staged for commit
```
