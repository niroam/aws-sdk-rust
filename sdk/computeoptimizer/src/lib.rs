#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <p>AWS Compute Optimizer is a service that analyzes the configuration and utilization
//! metrics of your AWS compute resources, such as EC2 instances, Auto Scaling groups, AWS Lambda
//! functions, and Amazon EBS volumes. It reports whether your resources are optimal, and
//! generates optimization recommendations to reduce the cost and improve the performance of
//! your workloads. Compute Optimizer also provides recent utilization metric data, as well as projected
//! utilization metric data for the recommendations, which you can use to evaluate which
//! recommendation provides the best price-performance trade-off. The analysis of your usage
//! patterns can help you decide when to move or resize your running resources, and still
//! meet your performance and capacity requirements. For more information about Compute Optimizer,
//! including the required permissions to use the service, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/">AWS Compute Optimizer User
//! Guide</a>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("computeoptimizer", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;