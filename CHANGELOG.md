# Changelog

> Crisp uses [keep a changelog](https://keepachangelog.com/en/1.0.0/) for separation of resposibility between the changelog itself and git logs, and [semantic versioning](https://semver.org/) for software version management.

**0.0.4:**
- Cleaned up codebase to make crisp more resilient to crashing (this is done in preparation for the codebase re-write coming soon)
- Added `do` function

**0.0.3:**
- Added String parsing (without whitespace support for now)
- Added `quote` function
- Added `print` function
- Removed outdated command-line mode (prioritizing REPL-mode for the time being)

**0.0.2:**
- MVP implemented for REPL-mode
- Added:
  - Integers
  - Floats
  - Booleans
  - Lambda functions
  - Binary conditionals
  - Lexical analysis (tokenizer)
  - Semantic parser (is a layer on top of the tokenizer)
  - Basic runtime environment
  - Evaluation functions

**0.0.1:**
- Added initial files to repository and setup GitHub repository CI/CD, settings, bots, etc.
