use tonic::{transport::Server, Request, Response, Status};

use chrono::SecondsFormat;
use recurrences_server::rrule_processing_server::{RruleProcessing, RruleProcessingServer};
use recurrences_server::{DataRrule, DatesReply, RRuleRequest};
mod rrule_builder;

pub mod recurrences_server {
    tonic::include_proto!("recurrencerule"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyRruleProcessing {}

#[tonic::async_trait]
impl RruleProcessing for MyRruleProcessing {
    async fn rrule_to_dates(
        &self,
        request: Request<RRuleRequest>, // Accept request of type RRuleRequest
    ) -> Result<Response<DatesReply>, Status> {
        // Return an instance of type DatesReply
        println!("Got a request: {:?}", request);
        let mut errors: Vec<String> = vec![];
        let result: rrule_builder::ProcessResult =
            rrule_builder::rrule_from_string(&request.into_inner().rrule).unwrap_or_else(|e| {
                errors.push(e.to_string());
                rrule_builder::ProcessResult {
                    rrule_result: None,
                    rrule: "".to_string(),
                    valid: false,
                    errors: [].to_vec(),
                }
            });
        let reply;
        if errors.is_empty() {
            let rules: Vec<String> = result
                .rrule_result
                .unwrap()
                .dates
                .into_iter()
                .map(|x| x.to_rfc3339_opts(SecondsFormat::Secs, true))
                .collect();
            reply = recurrences_server::DatesReply {
                dates: rules,
                rrule: result.rrule,
                valid: result.valid,
                errors: result.errors,
            };
        } else {
            reply = recurrences_server::DatesReply {
                dates: vec![],
                rrule: "".to_string(),
                valid: false,
                errors,
            };
        }

        Ok(Response::new(reply))
    }

    async fn data_rrule_to_dates(
        &self,
        request: Request<DataRrule>,
    ) -> Result<Response<DatesReply>, Status> {
        let mut errors: Vec<String> = vec![];

        let data = &request.into_inner();
        let result: rrule_builder::ProcessResult = rrule_builder::rrule_from_data(data)
            .unwrap_or_else(|e| {
                errors.push(e.to_string());
                rrule_builder::ProcessResult {
                    rrule_result: None,
                    rrule: "".to_string(),
                    valid: false,
                    errors: [].to_vec(),
                }
            });
        let reply;
        if errors.is_empty() {
            let rules: Vec<String> = result
                .rrule_result
                .unwrap()
                .dates
                .into_iter()
                .map(|x| x.to_rfc3339_opts(SecondsFormat::Secs, true))
                .collect();
            reply = recurrences_server::DatesReply {
                dates: rules,
                rrule: result.rrule,
                valid: result.valid,
                errors: result.errors,
            };
        } else {
            reply = recurrences_server::DatesReply {
                dates: vec![],
                rrule: "".to_string(),
                valid: false,
                errors,
            };
        }
        Ok(Response::new(reply)) // Send back our dates
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:4400".parse()?;
    let processer = MyRruleProcessing::default();

    Server::builder()
        .add_service(RruleProcessingServer::new(processer))
        .serve(addr)
        .await?;

    Ok(())
}
