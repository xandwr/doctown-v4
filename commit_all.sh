#!/bin/bash

# commit_all.sh - Format and commit all subprojects in the monorepo
# Usage: ./commit_all.sh -m "Your commit message"

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse command line arguments
COMMIT_MSG=""
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--message)
            COMMIT_MSG="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: ./commit_all.sh -m \"Your commit message\""
            echo ""
            echo "Options:"
            echo "  -m, --message    Commit message (required)"
            echo "  -h, --help       Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

# Check if commit message was provided
if [ -z "$COMMIT_MSG" ]; then
    echo -e "${RED}Error: Commit message is required${NC}"
    echo "Usage: ./commit_all.sh -m \"Your commit message\""
    exit 1
fi

# Get the root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Doctown Monorepo Commit Script${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Commit message:${NC} $COMMIT_MSG"
echo ""

# Function to format and commit a Rust project
format_and_commit() {
    local project_name=$1
    local project_path=$2
    
    echo -e "${BLUE}┌─────────────────────────────────────────────────────┐${NC}"
    echo -e "${BLUE}│ Processing: $project_name${NC}"
    echo -e "${BLUE}└─────────────────────────────────────────────────────┘${NC}"
    
    cd "$project_path"
    
    # Check if there are any changes
    if git diff --quiet && git diff --cached --quiet; then
        echo -e "${YELLOW}  ⊘ No changes to commit in $project_name${NC}"
        echo ""
        return 0
    fi
    
    # Format Rust code
    echo -e "${GREEN}  ⚙ Formatting Rust code...${NC}"
    if cargo fmt --all; then
        echo -e "${GREEN}  ✓ Formatting complete${NC}"
    else
        echo -e "${YELLOW}  ⚠ Formatting had some issues (continuing)${NC}"
    fi
    
    # Check for compilation errors
    echo -e "${GREEN}  ⚙ Checking compilation...${NC}"
    if cargo check --quiet 2>/dev/null; then
        echo -e "${GREEN}  ✓ Compilation check passed${NC}"
    else
        echo -e "${RED}  ✗ Compilation check failed${NC}"
        echo -e "${YELLOW}  Would you like to commit anyway? (y/N)${NC}"
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            echo -e "${RED}  Aborting commit for $project_name${NC}"
            exit 1
        fi
    fi
    
    # Stage all changes
    echo -e "${GREEN}  ⚙ Staging changes...${NC}"
    git add -A
    
    # Show what will be committed
    echo -e "${GREEN}  ⚙ Changes to be committed:${NC}"
    git status --short
    
    # Commit
    echo -e "${GREEN}  ⚙ Committing...${NC}"
    if git commit -m "$COMMIT_MSG"; then
        echo -e "${GREEN}  ✓ Committed successfully${NC}"
    else
        echo -e "${YELLOW}  ⚠ Commit failed (possibly nothing to commit)${NC}"
    fi
    
    echo ""
}

# Process Builder
format_and_commit "Builder" "$ROOT_DIR/builder"

# Process CLI
format_and_commit "CLI" "$ROOT_DIR/cli"

# Process main monorepo
echo -e "${BLUE}┌─────────────────────────────────────────────────────┐${NC}"
echo -e "${BLUE}│ Processing: Main Monorepo${NC}"
echo -e "${BLUE}└─────────────────────────────────────────────────────┘${NC}"

cd "$ROOT_DIR"

# Check if there are any changes in the root
if git diff --quiet && git diff --cached --quiet; then
    echo -e "${YELLOW}  ⊘ No changes to commit in main monorepo${NC}"
else
    echo -e "${GREEN}  ⚙ Staging changes...${NC}"
    git add -A
    
    echo -e "${GREEN}  ⚙ Changes to be committed:${NC}"
    git status --short
    
    echo -e "${GREEN}  ⚙ Committing...${NC}"
    if git commit -m "$COMMIT_MSG"; then
        echo -e "${GREEN}  ✓ Committed successfully${NC}"
    else
        echo -e "${YELLOW}  ⚠ Commit failed (possibly nothing to commit)${NC}"
    fi
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✓ All commits complete!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  • Review commits: ${GREEN}git log --oneline -n 3${NC}"
echo -e "  • Push to remote: ${GREEN}git push${NC}"
echo ""
