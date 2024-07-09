use crate::pb::{self, *};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status};

pub use pb::hwid::{hwid_service_server::HwidService, HwIdRequest, HwIdResponse};
type RResult<T> = tonic::Result<Response<T>>;

#[derive(Debug, Default)]
pub struct HwIdServiceServer {
    interface: String,
    identifier: Mutex<Option<String>>,
}

trait TonicStatus<T> {
    fn to_tonic(self) -> Result<T, Status>;
}

impl<T, E: std::string::ToString> TonicStatus<T> for Result<T, E> {
    fn to_tonic(self) -> Result<T, Status> {
        self.map_err(|e| Status::internal(e.to_string()))
    }
}

impl HwIdServiceServer {
    pub fn new(interface: String) -> Self {
        Self {
            interface,
            ..Default::default()
        }
    }
}

#[tonic::async_trait]
impl HwidService for HwIdServiceServer {
    async fn get_hw_id(&self, request: Request<HwIdRequest>) -> RResult<HwIdResponse> {
        let mut guard = self.identifier.lock().await;
        let identifier = match guard.as_mut() {
            Some(id) => id.clone(),
            None => {
                let raw = tokio::fs::read(format!("/sys/class/net/{}/address", self.interface))
                    .await
                    .to_tonic()?;
                let id = String::from_utf8(raw).to_tonic()?.trim().to_string();
                *guard = Some(id.clone());
                id
            }
        };
        Ok(Response::new(HwIdResponse { identifier }))
    }
}
