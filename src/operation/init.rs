
use tokio::*;
use clap::Parser;
use super::{
  config::Config,
  Operation
};


use crate::{
  api::*,
  operation::CONFIG_FILE_NAME
};



#[derive(Parser,Debug)]
pub struct Init {
  #[arg(short,long)]
  template: Option<Box<str>>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>
}

impl Operation for Init {
  async fn run(self)-> io::Result<()> {
    ensure_fresh_dir("./").await?;

    let url=url!(&ensure_template(self.template)=> ensure_lang(self.js));
    clone_repo(&url,"./").await?;


    let cwd=std::env::current_dir()?;
    Config::new(cwd.file_name().unwrap().to_str().unwrap()).save(CONFIG_FILE_NAME).await
  }
}




