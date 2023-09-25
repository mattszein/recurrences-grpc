use chrono::DateTime;
use chrono::SecondsFormat;
use iso8601_duration::Duration;
use recurrences_server::rrule_processing_server::{RruleProcessing, RruleProcessingServer};
use recurrences_server::{DataRequest, DataRrule, Dates, DatesReply, RRuleRequest};
use rrule::Tz;
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status};
mod rrule_builder;

pub mod recurrences_server {
    tonic::include_proto!("recurrencerule"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyRruleProcessing {}

pub fn add_duration(date: DateTime<Tz>, duration: &String) -> Result<DateTime<Tz>, Box<dyn Error>> {
    let iso_dur: Duration = duration
        .parse()
        .map_err(|e| format!("[duration] {:?}", e))?;
    //let dur = Duration::from_std(iso_dur.to_std().map_err(|e| format!("{:?}", e))?)?;
    Ok(date
        + iso_dur
            .to_chrono()
            .ok_or("[duration] Error converting to chrono duration")?)
}

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
                }
            });
        let reply;
        if errors.is_empty() {
            let rules: Vec<Dates> = result
                .rrule_result
                .unwrap()
                .dates
                .into_iter()
                .map(|x| Dates {
                    start: x.to_rfc3339_opts(SecondsFormat::Secs, true),
                    end: "".to_string(),
                })
                .collect();

            reply = recurrences_server::DatesReply {
                dates: rules,
                rrule: result.rrule,
                valid: true,
                errors,
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
        request: Request<DataRequest>,
    ) -> Result<Response<DatesReply>, Status> {
        let mut errors: Vec<String> = vec![];
        let data_request = request.into_inner();
        let result = rrule_builder::process_rrules(&data_request).unwrap_or_else(|e| {
            errors.push(e.to_string());
            rrule_builder::ProcessResult {
                rrule_result: None,
                rrule: "".to_string(),
            }
        });
        let reply;
        if errors.is_empty() {
            let dates: Vec<Dates> = result
                .rrule_result
                .unwrap()
                .dates
                .into_iter()
                .map(|x: DateTime<Tz>| Dates {
                    start: x.to_rfc3339_opts(SecondsFormat::Secs, true),
                    end: add_duration(x, &data_request.duration)
                        .expect("Reason")
                        .to_rfc3339_opts(SecondsFormat::Secs, true),
                })
                .collect();
            reply = recurrences_server::DatesReply {
                dates,
                rrule: result.rrule,
                valid: true,
                errors,
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
