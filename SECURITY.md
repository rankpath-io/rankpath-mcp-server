# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |

## Reporting a Vulnerability

Please **do not** report security vulnerabilities through public GitHub issues.

Instead, email **security@rankpath.io** with:

- A description of the vulnerability and its potential impact
- Steps to reproduce or a proof-of-concept
- Any relevant versions or configurations

You should receive a response within **48 hours**. We will keep you informed as we work on a fix and will credit you in the release notes unless you prefer to remain anonymous.

## Scope

This policy covers the `rankpath-mcp-server` binary and its source code. Issues in dependencies should be reported to those projects directly; however, please let us know if a dependency vulnerability directly affects this project.

## Notes

- This server communicates with `https://rankpath.io/api` over HTTPS using your API key. Never share your `RANKPATH_API_KEY` or commit it to source control.
- The server runs as a local stdio process; it does not open any network ports itself.
