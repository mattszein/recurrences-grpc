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
        let rules: Vec<String> = rrule_builder::rrule_from_string(&request.into_inner().rrule)
            .dates
            .into_iter()
            .map(|x| x.to_rfc3339_opts(SecondsFormat::Secs, true))
            .collect();
        let reply = recurrences_server::DatesReply {
            dates: rules, // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply))
    }

    async fn data_rrule_to_dates(
        &self,
        request: Request<DataRrule>,
    ) -> Result<Response<DatesReply>, Status> {
        let data = &request.into_inner();
        let rules: Vec<String> = rrule_builder::rrule_from_data(data)
            .dates
            .into_iter()
            .map(|x| x.to_rfc3339_opts(SecondsFormat::Secs, true))
            .collect();
        let reply = recurrences_server::DatesReply { dates: rules };
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
