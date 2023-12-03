
use tokio::*;
use clap::Parser;
use crate::api::*;
use super::build::Build;

use std::{
  path::PathBuf,
  env
};




#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum Operation {
  Build(Build),
  New {
    path: Box<str>,
    #[arg(short,long)]
    template: Option<Box<str>>,
    #[arg(long)]
    ts: bool,
    #[arg(long)]
    js: bool
  },
  Init {
    path: Option<PathBuf>,
    #[arg(short,long)]
    template: Option<Box<str>>,
    #[arg(long)]
    ts: bool,
    #[arg(long)]
    js: bool
  }
}

impl Operation {
  pub fn new()-> Self {
    Self::parse()
  }

  pub async fn spawn(self)-> io::Result<()> {
    match self {
      Operation::Build(build)=> build.build().await,
      Operation::Init { path,template,ts,.. }=> clone_repo(path,template,ts).await,
      _=> todo!()
    }
  }
}





async fn clone_repo(path: Option<PathBuf>,_template: Option<Box<str>>,ts: bool)-> io::Result<()> {
  let path=path.unwrap_or(env::current_dir()?);
  ensure_empty_dir(&path).await?;

  let _url=format!("https://github.com/kakashi-69-xd/proton-xd-templates/{}/{}",lang(ts),"next");



  Ok(())
}

fn lang(ts: bool)-> Box<str> {
  match ts {
    true=> "ts",
    false=> "js",
  }.into()
}

