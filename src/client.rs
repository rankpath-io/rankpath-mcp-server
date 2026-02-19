use crate::models::*;

const BASE_URL: &str = "https://rankpath.io/api";

pub struct RankPathClient {
    http: reqwest::Client,
    api_key: String,
}

impl RankPathClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
        }
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        query: Vec<(&str, String)>,
    ) -> anyhow::Result<T> {
        let url = format!("{}{}", BASE_URL, path);
        let mut req = self.http.get(&url).bearer_auth(&self.api_key);
        for (key, val) in &query {
            req = req.query(&[(key, val)]);
        }
        let resp = req.send().await?;

        if resp.status().is_success() {
            Ok(resp.json::<T>().await?)
        } else {
            let status = resp.status();
            let err = resp
                .json::<ErrorResponse>()
                .await
                .unwrap_or_else(|_| ErrorResponse {
                    error: status.to_string(),
                    message: None,
                });
            anyhow::bail!("{}: {}", err.error, err.message.unwrap_or_default())
        }
    }

    pub async fn list_projects(&self) -> anyhow::Result<Vec<Project>> {
        let resp = self.get::<ProjectListResponse>("/projects", vec![]).await?;
        Ok(resp.data)
    }

    pub async fn get_project(&self, project_id: &str) -> anyhow::Result<Project> {
        let path = format!("/projects/{}", project_id);
        let resp = self.get::<ProjectResponse>(&path, vec![]).await?;
        Ok(resp.data)
    }

    pub async fn get_crawl_history(
        &self,
        project_id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> anyhow::Result<CrawlHistoryData> {
        let path = format!("/projects/{}/crawls", project_id);
        let mut query = vec![];
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = offset {
            query.push(("offset", o.to_string()));
        }
        let resp = self.get::<CrawlHistoryResponse>(&path, query).await?;
        Ok(resp.data)
    }

    pub async fn get_latest_crawl(&self, project_id: &str) -> anyhow::Result<CrawlResult> {
        let path = format!("/projects/{}/crawls/latest", project_id);
        let resp = self.get::<CrawlResultResponse>(&path, vec![]).await?;
        Ok(resp.data)
    }

    pub async fn get_issues(
        &self,
        project_id: &str,
        severity: Option<&str>,
        status: Option<&str>,
    ) -> anyhow::Result<IssuesData> {
        let path = format!("/projects/{}/issues", project_id);
        let mut query = vec![];
        if let Some(s) = severity {
            query.push(("severity", s.to_string()));
        }
        if let Some(s) = status {
            query.push(("status", s.to_string()));
        }
        let resp = self.get::<IssuesResponse>(&path, query).await?;
        Ok(resp.data)
    }
}
