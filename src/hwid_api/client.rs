use crate::endpoint::EndpointConfig;
use crate::pb::{self, *};
use crate::types::*;
use anyhow::Result;
use tonic::transport::Channel;
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};

type Client = pb::hwid::hwid_service_client::HwidServiceClient<Channel>;

#[derive(Debug)]
pub struct HwIdClient {
    endpoint: EndpointConfig,
}

impl HwIdClient {
    pub fn new(endpoint: EndpointConfig) -> Self {
        Self { endpoint }
    }

    async fn connect(&self) -> anyhow::Result<Client> {
        let channel = self.endpoint.connect().await?;
        Ok(Client::new(channel))
    }

    pub async fn get_id(&self) -> anyhow::Result<String> {
        let request = pb::hwid::HwIdRequest {};
        let resp = self.connect().await?.get_hw_id(request).await?;
        let identifier = resp.into_inner().identifier;
        Ok(identifier)
    }
}
