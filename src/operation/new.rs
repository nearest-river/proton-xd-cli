
use tokio::*;
use clap::Parser;
use crate::api::*;
use std::path::PathBuf;

use requestty::{
  Question,
  prompt_one
};

use super::{
  config::Config,
  CONFIG_FILE_NAME, Operation
};




#[derive(Parser,Debug)]
pub struct New {
  path: Option<PathBuf>,
  #[arg(short,long)]
  template: Option<Box<str>>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>,
  #[arg(long="cd")]
  change_dir: Option<bool>
}


impl Operation for New {
  async fn run(self)-> io::Result<()> {
    let path=&self.ensure_path();
    fs::create_dir_all(path).await?;
    ensure_fresh_dir(path).await?;


    let url=url!(&ensure_template(self.template)=> ensure_lang(self.js));
    clone_repo(&url,path).await?;


    Config::new(
      path.file_name()
      .unwrap()
      .to_str()
      .unwrap()
    ).save(CONFIG_FILE_NAME).await
  }
}

impl New {
  fn ensure_path(&self)-> PathBuf {
    match &self.path {
      Some(path)=> path.into(),
      None=> {
        let question=Question::input("Project name")
        .default("my-app")
        .build();

        prompt_one(question)
        .unwrap()
        .as_string()
        .unwrap()
        .into()
      }
    }
  }
}




