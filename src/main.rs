extern crate colored;
mod model;
use core::fmt::{Debug, Formatter};
use std::ops::{Deref,DerefMut};
use colored::*;
use kube::{api::{ListParams, LogParams}, client::Client, Api, config::{KubeConfigOptions}};
use k8s_openapi::api::core::v1::Pod;
use kube::ResourceExt;
use futures::{TryStreamExt};
use clap::{Parser};
use rand::Rng;
use dict::{Dict};
use std::collections::HashMap;

#[derive(Default)]
pub struct CLI {
    command_name: String,
    version: String,
    description: String,
    // plugins: HashMap<String,String>,
    // default_plugins: HashMap<String, String>,
    default_project_version: String,
    extra_commands: String,
    extra_alpha_commands: String,
    completion_command: bool,
    // plugin_keys: Vec<String>,
    project_version: String,
    resolved_plugins: String,
    cmd: String,
    fs: String
}


// fn New() -> Result<*mut CLI, Box<dyn std::error::Error>>{
//    Ok(&mut CLI{
//     command_name: "Hahaha".to_owned(),
//     ..CLI::default()
//    })
// }


fn new_cli() -> Result<CLI, ()>{
    let c = CLI{
        command_name: "kubebuilder".to_owned(),
        description: "CLI tool".to_owned(),
        ..CLI::default()
    };
    Ok(c)
}

#[derive(Parser)]
#[clap(author = "Tanisha Banik", version="1.0.0", about="Software Developer")]
struct Kubetail{
    #[clap(short, long, value_parser, help="list all matching pods and containers names", default_value_t=false)]
    dry_run: bool,
    #[clap(short, long, value_parser, help="pod namespace", default_value="default")]
    namespace: String,
    #[clap(short, long, value_parser, help="pod label selector", default_value="")]
    label: String,
    #[clap(short, long, value_parser, help="returns previous terminated container logs. Defaults to false", default_value_t=false)]
    previous: bool,
    #[clap(short, long, value_parser, help="follows the log stream of the pod. Defaults to false", default_value_t=false)]
    follow: bool,
    #[clap(short, long, value_parser, help="number of lines from the end of the logs to show. Defaults to 0, shows streams right from the creation of the container", default_value_t=0)]
    tail: i64,
    #[clap(short='k', long, value_parser, help="pod name regex", default_value = "")]
    name: String,
    #[clap(short='c', long, value_parser, help="container name", default_value = "")]
    container: String,
    // #[clap(short='b', long, value_parser, help="number of bytes to read from the server before terminating the log output")]
    // limit_bytes: i64,
    #[clap(short= 'm', long, value_parser, help="pretty print output", default_value_t=false)]
    pretty: bool,
    #[clap(short, long, value_parser, help="relative time in seconds before the current time from which to show logs", default_value_t=20)]
    since: u64,
}


fn main() {
    println!("Hello, world!");
}
