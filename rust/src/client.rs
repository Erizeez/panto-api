use crate::errors::{Error, Result};
use crate::models::*;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use url::Url;

#[derive(Clone, Debug)]
pub struct PantoClient {
    base_url: Url,
    client: Client,
}

impl PantoClient {
    /// Create a new PantoClient instance with the given base URL.
    ///
    /// # Example
    /// ```rust
    /// use panto_api::PantoClient;
    ///
    /// let client = PantoClient::new("http://127.0.0.1:9090").unwrap();
    /// ```
    pub fn new(base_url: &str) -> Result<Self> {
        let mut trimmed = base_url.trim_end_matches('/').to_string();
        if !trimmed.ends_with('/') {
            trimmed.push('/');
        }
        let url = Url::parse(&trimmed)?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            base_url: url,
            client,
        })
    }

    /// Create with a custom reqwest client
    pub fn with_client(base_url: &str, client: Client) -> Result<Self> {
        let mut trimmed = base_url.trim_end_matches('/').to_string();
        if !trimmed.ends_with('/') {
            trimmed.push('/');
        }
        let url = Url::parse(&trimmed)?;
        Ok(Self {
            base_url: url,
            client,
        })
    }

    pub fn endpoint(&self, path: &str) -> Result<Url> {
        let clean_path = path.trim_start_matches('/');
        self.base_url.join(clean_path).map_err(Error::from)
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T> {
        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            let message = if let Ok(err_obj) = serde_json::from_str::<ErrorResponse>(&error_text) {
                err_obj.error
            } else {
                error_text
            };
            return Err(Error::Api { status, message });
        }

        let body = response.json::<T>().await?;
        Ok(body)
    }

    // ── System ──────────────────────────────────────────────────────────
    /// 获取核心版本信息
    pub async fn get_version(&self) -> Result<VersionResponse> {
        let url = self.endpoint("api/v1/version")?;
        self.request(self.client.get(url)).await
    }

    /// 获取系统运行状态与流量统计
    pub async fn get_status(&self) -> Result<StatusResponse> {
        let url = self.endpoint("api/v1/status")?;
        self.request(self.client.get(url)).await
    }

    /// 获取当前运行模式与全局出口
    pub async fn get_mode(&self) -> Result<ModeResponse> {
        let url = self.endpoint("api/v1/mode")?;
        self.request(self.client.get(url)).await
    }

    /// 切换运行模式与全局出口
    pub async fn set_mode(&self, req: &SetModeRequest) -> Result<ModeResponse> {
        let url = self.endpoint("api/v1/mode")?;
        self.request(self.client.put(url).json(req)).await
    }

    // ── Topology & Routing ──────────────────────────────────────────────
    /// 获取活跃网络拓扑图
    pub async fn get_topology(&self) -> Result<TopologyGraph> {
        let url = self.endpoint("api/v1/topology")?;
        self.request(self.client.get(url)).await
    }

    /// 获取所有出站策略组及成员
    pub async fn get_groups(&self) -> Result<GroupsResponse> {
        let url = self.endpoint("api/v1/groups")?;
        self.request(self.client.get(url)).await
    }

    /// 为选择组手动切换活跃出口节点
    pub async fn select_group_member(
        &self,
        group_id: &str,
        req: &SelectGroupRequest,
    ) -> Result<SelectGroupResponse> {
        let url = self.endpoint(&format!("api/v1/groups/{}/select", group_id))?;
        self.request(self.client.put(url).json(req)).await
    }

    // ── Flows ───────────────────────────────────────────────────────────
    /// 获取活跃网络流列表
    pub async fn get_flows(&self) -> Result<FlowsResponse> {
        let url = self.endpoint("api/v1/flows")?;
        self.request(self.client.get(url)).await
    }

    // ── Probe Benchmark ─────────────────────────────────────────────────
    /// 获取全球基准测速站点列表
    pub async fn get_probe_sites(&self) -> Result<ProbeSitesResponse> {
        let url = self.endpoint("api/v1/probe/sites")?;
        self.request(self.client.get(url)).await
    }

    /// 执行测速基准测试
    pub async fn test_probe_sites(
        &self,
        req: Option<&ProbeTestRequest>,
    ) -> Result<ProbeTestResponse> {
        let url = self.endpoint("api/v1/probe/test")?;
        let mut rb = self.client.post(url);
        if let Some(r) = req {
            rb = rb.json(r);
        }
        self.request(rb).await
    }

    // ── Observation ─────────────────────────────────────────────────────
    /// 获取用量观测授权申请列表
    pub async fn get_observation_consents(&self) -> Result<Vec<ObservationConsentItem>> {
        let url = self.endpoint("api/v1/observation/consents")?;
        self.request(self.client.get(url)).await
    }

    /// 提交对特定节点的观测授权决策
    pub async fn decide_observation_consent(
        &self,
        node_id: &str,
        req: &DecideConsentRequest,
    ) -> Result<DecideConsentResponse> {
        let url = self.endpoint(&format!("api/v1/observation/consents/{}/decide", node_id))?;
        self.request(self.client.post(url).json(req)).await
    }

    /// 撤销节点的观测授权
    pub async fn revoke_observation_consent(&self, node_id: &str) -> Result<DecideConsentResponse> {
        let url = self.endpoint(&format!("api/v1/observation/consents/{}", node_id))?;
        self.request(self.client.delete(url)).await
    }

    /// 获取观测推送引擎状态诊断
    pub async fn get_observation_status(&self) -> Result<ObservationStatusResponse> {
        let url = self.endpoint("api/v1/observation/status")?;
        self.request(self.client.get(url)).await
    }
}
