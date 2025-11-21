# localdoc CLI

A command-line tool for inspecting and querying `.docpack` files created by the Doctown Builder.

## Installation

```bash
cargo build --release
# Binary will be at target/release/localdoc
```

## Docpack Location

Docpacks are stored in `~/.localdoc/docpacks/` by default. The CLI can reference docpacks by:
- Full path: `localdoc info /path/to/myproject.docpack`
- Just the name: `localdoc info myproject` (automatically looks in `~/.localdoc/docpacks/`)

The builder automatically places generated `.docpack` files in `~/.localdoc/docpacks/`.

## Usage

### Generate a Docpack

Generate a new docpack from a source zip file:

```bash
localdoc generate myproject.zip
```

This command:
- Runs the builder pipeline on your source code
- Generates a `.docpack` file in `~/.localdoc/docpacks/`
- Automatically finds the builder binary (checks release/debug builds)

The builder must be compiled first:
```bash
cd builder && cargo build --release
```

### List Installed Docpacks

Show all docpacks installed in `~/.localdoc/docpacks/`:

```bash
localdoc list
```

Example output:
```
Installed Docpacks (3 total)
Location: "/home/user/.localdoc/docpacks"
================================================================================
myproject (245.32 KB)
       Modified: 2 days ago
another-project (512.10 KB)
       Modified: Yesterday
example (128.45 KB)
       Modified: Today

Use 'localdoc info <name>' to inspect a docpack
```

### Quick Info

Show a summary of what's in a docpack:

```bash
localdoc info myproject
# or with full path
localdoc info path/to/file.docpack
```

Example output:
```
Package
  Source:     test_sources/localdoc-main.zip
  Generated:  2025-11-21T08:40:06Z
  Size:       372.99 KB

Graph Contents
  Total Nodes:   78
  Total Edges:   222
  Functions:     30
  Types:         15
  Modules:       3
  Clusters:      13

Documentation
  Symbol docs:   45
  Module docs:   3
  Tokens used:   22965
```

### Detailed Statistics

View comprehensive stats including complexity analysis and public API surface:

```bash
localdoc stats myproject
```

### List Nodes in a Docpack

List all nodes in the graph with optional filtering:

```bash
# List all functions
localdoc nodelist myproject --kind function

# List only public nodes
localdoc nodelist myproject --public

# Limit results
localdoc nodelist myproject --limit 20
```

Node kinds: `function`, `type`, `module`, `file`, `cluster`, `constant`, `trait`, `macro`, `package`

### Search

Search for nodes by name:

```bash
# Case-insensitive search (default)
localdoc search file.docpack "McpServer"

# Case-sensitive search
localdoc search file.docpack "McpServer" --case-sensitive
```

### Inspect

Deep dive into a specific node:

```bash
localdoc inspect file.docpack "path/to/file.rs::function::my_function"
```

Shows:
- Basic info (kind, visibility, location)
- Type/function details (signatures, parameters, fields)
- Metadata (complexity, fan-in/fan-out)
- Edges (calls, imports, references)
- Source snippet

### View Documentation

Show AI-generated documentation for a node:

```bash
localdoc docs file.docpack "path/to/file.rs::function::my_function"
```

Displays:
- Purpose and explanation
- Complexity notes
- Usage hints
- Caller/callee references
- Semantic cluster membership

### Extract Files

Extract the raw JSON files from a docpack:

```bash
# Extract to current directory
localdoc extract file.docpack

# Extract to specific directory
localdoc extract file.docpack --output ./extracted
```

Extracts:
- `graph.json` - Complete code graph
- `documentation.json` - AI-generated docs
- `metadata.json` - Package metadata
- `README.md` - Docpack info

## Examples

```bash
# Get a quick overview
localdoc info myproject.docpack

# Find all public functions
localdoc list myproject.docpack --kind function --public

# Search for authentication-related code
localdoc search myproject.docpack "auth"

# Inspect a specific function
localdoc inspect myproject.docpack "src/main.rs::function::main"

# View docs for a complex function
localdoc docs myproject.docpack "src/api.rs::function::handle_request"

# Analyze code complexity
localdoc stats myproject.docpack
```

## Tips

- Use `search` to find node IDs, then `inspect` or `docs` to examine them
- The `--public` flag helps identify your public API surface
- Sort by complexity in `stats` output to find refactoring candidates
- Extract files to process the JSON data with custom tools

## Data Format

Docpacks contain:
- **Graph**: Nodes (functions, types, modules) and edges (calls, imports, references)
- **Metrics**: Complexity, fan-in/fan-out, public API detection
- **Documentation**: AI-generated explanations and insights
- **Clusters**: Semantic groupings of related code

## Building from Source

```bash
cargo build --release
cargo install --path .
```
