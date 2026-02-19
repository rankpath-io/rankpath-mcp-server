# RankPath MCP Server


[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](LICENSE)

An [MCP (Model Context Protocol)](https://modelcontextprotocol.io) server that connects AI assistants to [RankPath](https://rankpath.io), giving them read access to your projects, crawl results, and SEO issues.

## Tools

| Tool | Description |
|------|-------------|
| `list_projects` | List all projects for the authenticated user |
| `get_project` | Get details for a specific project by UUID |
| `get_crawl_history` | Get paginated crawl history for a project |
| `get_latest_crawl` | Get the latest crawl result with full SEO analysis |
| `get_issues` | Get SEO issues, optionally filtered by severity or status |

## Requirements

- Rust 1.75+
- A [RankPath](https://rankpath.io) account and API key

## Installation

### Quick install (Linux & macOS)

Requires `git` and Rust (`cargo`). Installs the binary to `/usr/local/bin` or `~/.local/bin`:

```bash
curl -fsSL https://raw.githubusercontent.com/rankpath-io/rankpath-mcp-server/main/bin/install-mcp.sh | bash
```

### Manual build

```bash
git clone https://github.com/rankpath/rankpath-mcp-server
cd rankpath-mcp-server
cargo build --release
# binary at: target/release/rankpath-mcp-server
```

## Setup

Set your API key:

```bash
export RANKPATH_API_KEY=your_api_key_here
```

### Claude Desktop

Edit `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "rankpath": {
      "command": "/path/to/rankpath-mcp-server",
      "env": {
        "RANKPATH_API_KEY": "your_api_key_here"
      }
    }
  }
}
```

### Claude Code

```bash
claude mcp add rankpath /path/to/rankpath-mcp-server --env RANKPATH_API_KEY=your_api_key_here
```

## Security

Never commit your `RANKPATH_API_KEY` to source control. The server runs as a local stdio process and does not open any network ports. See [SECURITY.md](SECURITY.md) for the vulnerability reporting policy.

## License

[Mozilla Public License 2.0](LICENSE)
