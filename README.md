# LLM Context Builder

Generate LLM Context for local or files in a GitHub repository.
It can filter by file extensions, ignore specified directories, and optionally print file contents.

## Features

- Search files locally or in GitHub repositories
- Filter by file extensions
- Ignore specified directories
- Print file contents
- Support for downloading and extracting GitHub repositories
- Configurable logging levels

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from source

1. Clone the repository:
```shell
git clone https://github.com/namuan/llm-context-builder.git
cd llm-context-builder
```

1. Build the project:

```shell
make build      # Development build
# or
make release    # Release build
```

1. (Optional) Install the binary system-wide:

```shell
make install
```

## Usage

### Basic Commands
```text
# Search local files
llm-context-builder --extensions .json .py --ignored_dirs build dist --ignored_files package-lock.json --print_contents

# Search GitHub repository
llm-context-builder --github_url https://github.com/username/repo/tree/main/path --extensions .md .mdx --print_contents
```

### Command Line Arguments
```text
--github_url: GitHub URL to download and search
--extensions: List of file extensions to search for
--ignored_dirs: List of directories to ignore
--ignored_files: List of files to ignore
--print_contents: Flag to print file contents
--verbose: Increase output verbosity
```

### Examples

```text
# Search for Python and JSON files locally, ignoring a specific file
llm-context-builder --extensions .py .json --ignored_files config.json --print_contents

# Search for Markdown files in a GitHub repository
llm-context-builder --github_url https://github.com/user/repo --extensions .md --print_contents

# Search with ignored directories and files
llm-context-builder --extensions .rs --ignored-dirs target node_modules --ignored_files main.rs --print_contents
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.