use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCounts {
    pub critical: u32,
    pub warning: u32,
    pub info: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlSummary {
    pub id: String,
    pub status: String,
    pub crawled_at: String,
    pub score: Option<u32>,
    pub issue_counts: Option<IssueCounts>,
    pub http_status: Option<u32>,
    pub response_time_ms: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeoData {
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub h1_tags: Option<Vec<String>>,
    pub canonical_url: Option<String>,
    pub language: Option<String>,
    pub robots_meta: Option<String>,
    pub open_graph: Option<OpenGraph>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMetrics {
    pub word_count: Option<u32>,
    pub list_count: Option<u32>,
    pub image_count: u32,
    pub link_count: u32,
    pub internal_link_count: u32,
    pub external_link_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub has_alt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    pub text: String,
    pub is_internal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoAnalysis {
    pub citation_score: Option<u32>,
    pub citable_facts_count: Option<u32>,
    pub questions_answered: Option<Vec<String>>,
    pub strengths: Option<Vec<String>>,
    pub weaknesses: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,
    pub authority_topics: Option<Vec<String>>,
    pub analyzed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlResult {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub error_message: Option<String>,
    pub crawled_at: String,
    pub score: Option<u32>,
    pub issue_counts: Option<IssueCounts>,
    pub http_status: Option<u32>,
    pub response_time_ms: Option<u32>,
    pub seo_data: Option<SeoData>,
    pub content_metrics: Option<ContentMetrics>,
    pub images: Option<Vec<Image>>,
    pub links: Option<Vec<Link>>,
    pub geo_analysis: Option<GeoAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    #[serde(rename = "type")]
    pub issue_type: String,
    pub severity: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssuesSummary {
    pub critical: u32,
    pub warning: u32,
    pub info: u32,
    pub open: u32,
    pub acknowledged: u32,
    pub ignored: u32,
}

// API response wrappers

#[derive(Debug, Deserialize)]
pub struct ProjectListResponse {
    pub data: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectResponse {
    pub data: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlHistoryData {
    pub crawls: Vec<CrawlSummary>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct CrawlHistoryResponse {
    pub data: CrawlHistoryData,
}

#[derive(Debug, Deserialize)]
pub struct CrawlResultResponse {
    pub data: CrawlResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssuesData {
    pub issues: Vec<Issue>,
    pub total: u32,
    pub summary: IssuesSummary,
}

#[derive(Debug, Deserialize)]
pub struct IssuesResponse {
    pub data: IssuesData,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: Option<String>,
}
