# StevioCode (sc)

A CLI tool for managing multiple Git projects and worktrees efficiently.

## What it does

StevioCode helps developers who work with many Git repositories by providing a centralized way to discover, navigate, and manage projects. It has first-class support for git worktrees and can perform batch operations across multiple repositories.

## Installation

```bash
cargo install --path .
```

The binary is named `sc` for quick access.

## Core Features (Planned)

- **Project Discovery**: Automatically finds and indexes Git repositories in configured project directory
- **Git Worktree Support**: Full lifecycle management for git worktrees
- **Quick Navigation**: Jump between projects and worktrees with fuzzy matching
- **Status Overview**: See the state of all your projects at a glance
- **Batch Operations**: Run git commands across multiple repositories
- **Context-Aware Actions**: Execute project-specific commands based on detected language/framework

## Basic Usage (Planned)

```bash
# List all managed projects
sc (p)roject list

# Navigate to a project
sc my-project

# Navigate to a project (with fuzzy matching)
sc cd

# Open project in editor
sc my-project (e)dit

# Show status of all projects
sc (s)tatus

# Update all projects
sc (u)pdate

# Work with worktrees
sc my-project --worktree feature-branch
sc my-project (e)dit -w  # interactive worktree selection
```

## Configuration

Configuration is stored in your user config directory. The tool will create a default config on first run.
