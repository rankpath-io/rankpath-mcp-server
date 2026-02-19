use std::{env, sync::Arc};

use rmcp::{
    handler::server::tool::ToolCallContext,
    model::{
        CallToolRequestParam, CallToolResult, Content, Implementation, ListToolsResult,
        PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars,
    service::RequestContext,
    tool, Error as McpError, RoleServer, ServerHandler, ServiceExt,
};
use serde::Deserialize;

mod client;
mod models;

use client::RankPathClient;

#[derive(Clone)]
struct RankPathServer {
    client: Arc<RankPathClient>,
}

// --- Tool input types ---

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetProjectInput {
    /// The project UUID
    project_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetCrawlHistoryInput {
    /// The project UUID
    project_id: String,
    /// Maximum number of results to return (1–100, default: 10)
    limit: Option<u32>,
    /// Number of results to skip for pagination (default: 0)
    offset: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetLatestCrawlInput {
    /// The project UUID
    project_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetIssuesInput {
    /// The project UUID
    project_id: String,
    /// Filter by severity: "critical", "warning", or "info"
    severity: Option<String>,
    /// Filter by status: "open", "acknowledged", or "ignored"
    status: Option<String>,
}

// --- Tool implementations ---

#[tool(tool_box)]
impl RankPathServer {
    fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(RankPathClient::new(api_key)),
        }
    }

    #[tool(description = "List all RankPath projects for the authenticated user")]
    async fn list_projects(&self) -> Result<CallToolResult, McpError> {
        match self.client.list_projects().await {
            Ok(projects) => {
                let text =
                    serde_json::to_string_pretty(&projects).unwrap_or_else(|e| e.to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(api_error(e)),
        }
    }

    #[tool(description = "Get details for a specific RankPath project by its UUID")]
    async fn get_project(
        &self,
        #[tool(aggr)] input: GetProjectInput,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_project(&input.project_id).await {
            Ok(project) => {
                let text = serde_json::to_string_pretty(&project).unwrap_or_else(|e| e.to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(api_error(e)),
        }
    }

    #[tool(description = "Get paginated crawl history for a RankPath project")]
    async fn get_crawl_history(
        &self,
        #[tool(aggr)] input: GetCrawlHistoryInput,
    ) -> Result<CallToolResult, McpError> {
        match self
            .client
            .get_crawl_history(&input.project_id, input.limit, input.offset)
            .await
        {
            Ok(data) => {
                let text = serde_json::to_string_pretty(&data).unwrap_or_else(|e| e.to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(api_error(e)),
        }
    }

    #[tool(
        description = "Get the latest crawl result with full SEO analysis for a RankPath project"
    )]
    async fn get_latest_crawl(
        &self,
        #[tool(aggr)] input: GetLatestCrawlInput,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_latest_crawl(&input.project_id).await {
            Ok(result) => {
                let text = serde_json::to_string_pretty(&result).unwrap_or_else(|e| e.to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(api_error(e)),
        }
    }

    #[tool(
        description = "Get SEO issues for a RankPath project, optionally filtered by severity (critical/warning/info) or status (open/acknowledged/ignored)"
    )]
    async fn get_issues(
        &self,
        #[tool(aggr)] input: GetIssuesInput,
    ) -> Result<CallToolResult, McpError> {
        match self
            .client
            .get_issues(
                &input.project_id,
                input.severity.as_deref(),
                input.status.as_deref(),
            )
            .await
        {
            Ok(data) => {
                let text = serde_json::to_string_pretty(&data).unwrap_or_else(|e| e.to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(api_error(e)),
        }
    }
}

fn api_error(e: anyhow::Error) -> CallToolResult {
    CallToolResult {
        content: vec![Content::text(e.to_string())],
        is_error: Some(true),
    }
}

// --- ServerHandler ---

impl ServerHandler for RankPathServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "rankpath-mcp-server".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            instructions: Some(
                "RankPath SEO analysis MCP server. \
                 Provides access to project data, crawl results, and SEO issues. \
                 Requires the RANKPATH_API_KEY environment variable."
                    .into(),
            ),
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let ctx = ToolCallContext::new(self, request, context);
        Self::tool_box().call(ctx).await
    }

    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult {
            tools: Self::tool_box().list(),
            next_cursor: None,
        })
    }
}

// --- Entry point ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key =
        env::var("RANKPATH_API_KEY").expect("RANKPATH_API_KEY environment variable is required");

    let server = RankPathServer::new(api_key);

    let service = server
        .serve(rmcp::transport::io::stdio())
        .await
        .inspect_err(|e| eprintln!("Server error: {e}"))?;

    service.waiting().await?;
    Ok(())
}
