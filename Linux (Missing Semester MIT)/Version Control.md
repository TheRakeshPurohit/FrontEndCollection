# Lecture 6: Version Control (git)


## Git command-line interface 
### Basics 

- git help <command>: get help for a git command 
- git init: creates a new git repo, with data stored in the .git directory 
- git status: tells you what’s going on 
- git add <filename>: adds files to staging area 
- git commit: creates a new commit 
- git log: shows a flattened log of history 
- git log --all --graph --decorate: visualizes history as a DAG 
- git log --all --graph --decorate --oneline: shows the history in one line also 
- git diff <filename>: show changes you made relative to the staging area 
- git diff <revision> <filename>: shows differences in a file between snapshots 
- git checkout <revision>: updates HEAD and current branch 

### Branching and merging 
- git branch: shows branches 
- git branch <name>: creates a branch 
- git checkout -b <name>: creates a branch and switches to it - same as git branch <name>; git checkout <name> 
- git merge <revision>: merges into current branch 
- git mergetool: use a fancy tool to help resolve merge conflicts 
- git rebase: rebase set of patches onto a new base 

### Remotes 
- git remote: list remotes 
- git remote add <name> <url>: add a remote 
- git push <remote> <local branch>:<remote branch>: send objects to remote, and update remote reference 
- git branch --set-upstream-to=<remote>/<remote branch>: set up correspondence between local and remote branch 
- git fetch <remote>: retrieve objects/references from a remote 
- git pull: same as git fetch; git merge 
- git clone <url> <folder>: download repository from remote 
### Undo 
- git commit --amend: edit a commit’s contents/message 
- git reset HEAD <file>: unstage a file 
- git checkout -- <file>: discard changes 

### Advanced Git 
- git config: Change configurations of git 
- git clone --depth=1: shallow clone, without entire version history 
- git add -p: interactive staging 
- git diff --cached: shows changes staged for commit 
- git rebase -i: interactive rebasing 
- git blame: show who last edited which line 
- git stash: temporarily remove modifications to working directory - git stash pop: undo the changes by git stash 
- git bisect: binary search history (e.g. for regressions) 
- .gitignore: specify intentionally untracked files to ignore




## Git Concepts (Core Ideas)

- Git stores project history as a directed acyclic graph (DAG) of commits
- A commit is a snapshot of the entire project state
- Commits are immutable; history edits create new commits and move references
- blob: file contents
- tree: directory structure
- commit: snapshot + metadata + parent commits
- branch: a movable pointer to a commit
- HEAD: pointer to the current branch or commit
- Git objects are content-addressed using SHA-1 hashes
- The staging area allows selecting which changes go into the next commit