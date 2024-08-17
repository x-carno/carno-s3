/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

mod aws;

#[derive(Debug, Parser)]
struct Opt {
    /// The profile name in file ~/.aws/credentials
    #[structopt(short, long, default_value = "default")]
    profile: Option<String>,

    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to only get buckets in the Region.
    #[structopt(short, long)]
    strict: bool,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// snippet-end:[s3.rust.list-buckets]

/// Lists your Amazon S3 buckets, or just the buckets in the Region.
/// # Arguments
///
/// * `[-s]` - Only list bucket in the Region.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    // tracing_subscriber::fmt::init();

    let Opt {
        profile,
        region,
        strict,
        verbose,
    } = Opt::parse();
    // println!("region: {:?}", region);
    // println!("strict:{}", strict);
    // println!("verbose:{}", verbose);

    // The name of the credentials profile you want to load
    let profile = profile.unwrap();
    println!("profile name: {:?}", profile);
    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile)
        .build();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));

    // CredentialsProviderChain::first_try(name, provider)

    println!();
    let region_str: String = String::from(region_provider.region().await.unwrap().as_ref());

    println!();

    if verbose {
        println!("S3 client version: {}", PKG_VERSION);
        println!(
            "Region:            {}",
            region_provider.region().await.unwrap().as_ref()
        );

        if strict {
            println!("Only lists buckets in the Region.");
        } else {
            println!("Lists all buckets.");
        }

        println!();
    }

    let shared_config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await;
    let s3_client = Client::new(&shared_config);

    aws::show_buckets(strict, &s3_client, &region_str).await
}
