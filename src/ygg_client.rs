use crate::flaresolverr::FlareSolverr;
use std::sync::Arc;

#[derive(Clone)]
pub enum YggClient {
    Direct(wreq::Client),
    Proxied {
        flaresolverr: Arc<FlareSolverr>,
        session_id: String,
    },
}

pub struct YggResponse {
    pub status: u16,
    pub body: String,
    pub url: String,
}

impl YggClient {
    fn session_ref(session_id: &str) -> Option<&str> {
        if session_id.is_empty() {
            None
        } else {
            Some(session_id)
        }
    }

    pub async fn get(&self, url: &str) -> Result<YggResponse, Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client.get(url).send().await?;
                let status = response.status().as_u16();
                let final_url = response.url().to_string();
                let body = response.text().await?;
                Ok(YggResponse {
                    status,
                    body,
                    url: final_url,
                })
            }
            YggClient::Proxied {
                flaresolverr,
                session_id,
            } => {
                let response = flaresolverr
                    .get(url, Self::session_ref(session_id), None)
                    .await?;
                let solution = response
                    .solution
                    .ok_or("No solution in FlareSolverr response")?;
                Ok(YggResponse {
                    status: solution.status,
                    body: solution.response,
                    url: solution.url,
                })
            }
        }
    }

    pub async fn post_form(
        &self,
        url: &str,
        form_data: &str,
    ) -> Result<YggResponse, Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client
                    .post(url)
                    .body(form_data.to_string())
                    .header(
                        "Content-Type",
                        "application/x-www-form-urlencoded; charset=UTF-8",
                    )
                    .send()
                    .await?;
                let status = response.status().as_u16();
                let final_url = response.url().to_string();
                let body = response.text().await?;
                Ok(YggResponse {
                    status,
                    body,
                    url: final_url,
                })
            }
            YggClient::Proxied {
                flaresolverr,
                session_id,
            } => {
                let response = flaresolverr
                    .post(url, form_data, Self::session_ref(session_id), None)
                    .await?;
                let solution = response
                    .solution
                    .ok_or("No solution in FlareSolverr response")?;
                Ok(YggResponse {
                    status: solution.status,
                    body: solution.response,
                    url: solution.url,
                })
            }
        }
    }

    pub async fn get_bytes(&self, url: &str) -> Result<(u16, Vec<u8>), Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client.get(url).send().await?;
                let status = response.status().as_u16();
                let bytes = response.bytes().await?.to_vec();
                Ok((status, bytes))
            }
            YggClient::Proxied {
                flaresolverr,
                session_id,
            } => {
                let response = flaresolverr
                    .get(url, Self::session_ref(session_id), None)
                    .await?;
                let solution = response
                    .solution
                    .ok_or("No solution in FlareSolverr response")?;
                let status = solution.status;
                let bytes = solution.response.into_bytes();
                Ok((status, bytes))
            }
        }
    }

    pub fn as_wreq_client(&self) -> Option<&wreq::Client> {
        match self {
            YggClient::Direct(client) => Some(client),
            YggClient::Proxied { .. } => None,
        }
    }
}
