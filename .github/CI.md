# Continuous Integration (CI) Setup

This document explains the automated testing setup for the Observer project.

## Overview

The project uses GitHub Actions for continuous integration to ensure code changes don't break the application. CI runs automatically on:
- **Pull Requests** to the `main` branch
- **Pushes** to the `main` branch (when app code changes)

## CI Workflow Jobs

The CI pipeline consists of several jobs that run in parallel where possible:

### 1. Lint and Type Check
- **Purpose**: Ensures code quality and type safety
- **Checks**:
  - ESLint for code style and potential errors
  - TypeScript type checking
- **When it runs**: On every PR and push
- **Duration**: ~1-2 minutes

### 2. Build Frontend
- **Purpose**: Verifies the React/Vite frontend builds successfully
- **Checks**:
  - npm build completes without errors
  - dist directory is created
- **When it runs**: After lint checks pass
- **Duration**: ~2-3 minutes

### 3. Build Tauri App
- **Purpose**: Ensures the Tauri application builds on all platforms
- **Platforms tested**:
  - Ubuntu (Linux x86_64)
  - macOS (ARM64/Apple Silicon)
  - Windows (x86_64)
- **Checks**:
  - Full Tauri build completes
  - Binary artifacts are created
- **When it runs**: After lint checks pass
- **Duration**: ~5-10 minutes per platform
- **Note**: Builds in debug mode for faster CI

### 4. Check Rust Code
- **Purpose**: Ensures Rust code quality
- **Checks**:
  - `cargo fmt` - code formatting
  - `cargo clippy` - lint and code analysis
  - `cargo test` - runs all Rust tests
- **When it runs**: On every PR and push
- **Duration**: ~3-5 minutes

### 5. Security Audit
- **Purpose**: Identifies security vulnerabilities in dependencies
- **Checks**:
  - `cargo audit` - checks Rust dependencies
  - `npm audit` - checks npm dependencies
- **When it runs**: On every PR and push
- **Duration**: ~1-2 minutes
- **Note**: Fails build if high severity vulnerabilities found

### 6. Check Wayland Support
- **Purpose**: Verifies Wayland support is correctly implemented
- **Checks**:
  - Wayland dependencies present in `Cargo.lock`
  - Desktop file exists (`Observer.desktop`)
  - Desktop file does NOT set `GDK_BACKEND` (relies on auto-detection)
- **When it runs**: On every PR and push
- **Duration**: ~30 seconds

### 7. Check OpenAI Endpoints UI
- **Purpose**: Verifies UI correctly labels OpenAI-compatible endpoints
- **Checks**:
  - "Custom OpenAI-Compatible API Servers" text present
  - llama-swap mentioned in examples
- **When it runs**: On every PR and push
- **Duration**: ~30 seconds

## Viewing CI Results

### On Pull Requests
1. Open your pull request on GitHub
2. Scroll down to the "Checks" section
3. Click on "Details" next to any job to see logs
4. All checks must pass (green ✓) before merging

### On the Actions Tab
1. Go to the repository's "Actions" tab
2. Click on "CI - Build and Test" workflow
3. View recent runs and their status

## CI Status Badge

Add this badge to your README to show CI status:

```markdown
[![CI](https://github.com/flooryyyy/Observer/actions/workflows/ci.yml/badge.svg)](https://github.com/flooryyyy/Observer/actions/workflows/ci.yml)
```

## What Triggers CI

CI runs when:
- ✅ Pull request is opened or updated
- ✅ Code is pushed to `main` branch
- ✅ Files in `app/**` are changed
- ✅ The CI workflow file itself is changed

CI does NOT run when:
- ❌ Only documentation files are changed (outside `app/`)
- ❌ Only website files are changed (separate workflow)
- ❌ Draft PRs (can be configured if needed)

## Local Testing Before Pushing

To run the same checks locally before pushing:

### Lint and Type Check
```bash
cd app
npm run lint
npx tsc --noEmit
```

### Build Frontend
```bash
cd app
npm run build
```

### Check Rust Code
```bash
cd app/src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

### Security Audit
```bash
cd app/src-tauri
cargo install cargo-audit
cargo audit

cd ../..
npm audit --audit-level=high
```

## Troubleshooting CI Failures

### Lint Errors
- Fix by running `npm run lint` locally
- Auto-fix many issues: `npm run lint -- --fix`

### Type Errors
- Fix TypeScript errors shown in `npx tsc --noEmit`
- Ensure all imports and types are correct

### Build Failures
- Check build logs for specific errors
- Try building locally: `npm run build`
- Clear caches: `rm -rf node_modules dist && npm install`

### Rust Errors
- Format: `cargo fmt --all`
- Fix Clippy warnings: `cargo clippy --fix`
- Run tests locally: `cargo test`

### Platform-Specific Build Failures
- Check if issue is platform-specific
- May need to adjust dependencies or platform-specific code
- Review Tauri documentation for platform requirements

## CI Performance

Typical CI run times:
- **Fast path** (lint only): ~2 minutes
- **Full build** (all platforms): ~15-20 minutes
- **PR update** (cached): ~10-15 minutes

## Caching

CI uses caching to speed up builds:
- npm dependencies cached
- Rust compilation artifacts cached
- Reduces subsequent build times by 50-70%

## Required Status Checks

To protect the `main` branch, configure these as required checks:
1. Go to Settings → Branches → Branch protection rules
2. Check "Require status checks to pass before merging"
3. Select these checks as required:
   - Lint and Type Check
   - Build Frontend
   - Build Tauri (ubuntu-latest)
   - Check Rust
   - Security Audit
   - Check Wayland Support
   - Check OpenAI Endpoints UI

## Future Enhancements

Potential improvements to CI:
- [ ] Add end-to-end tests with Playwright
- [ ] Add visual regression testing
- [ ] Code coverage reporting
- [ ] Performance benchmarking
- [ ] Automated dependency updates (Dependabot)
- [ ] Deploy preview builds for PRs

## Related Files

- `.github/workflows/ci.yml` - Main CI workflow
- `.github/workflows/release.yml` - Release workflow (separate)
- `app/package.json` - npm scripts
- `app/src-tauri/Cargo.toml` - Rust dependencies

## Getting Help

If CI fails and you can't figure out why:
1. Check the detailed logs in the Actions tab
2. Try reproducing the issue locally
3. Search for similar issues in the repository
4. Ask for help in the pull request comments
