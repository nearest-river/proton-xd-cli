
use tokio::*;
use clap::Parser;
use crate::api::*;
use std::path::PathBuf;


use requestty::{
  Question,
  prompt_one
};





#[derive(Parser,Debug)]
pub struct New {
  path: Option<PathBuf>,
  #[arg(short,long)]
  template: Option<String>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>
}


impl New {
  pub async fn init(self)-> io::Result<()> {
    let path=&self.ensure_path();
    ensure_dir(path).await?;
    ensure_fresh_dir(path).await?;
    std::env::set_current_dir(path)?;


    let url=url(&ensure_template(self.template),self.ts.unwrap_or_default());
    clone_repo(&url,"./")?;// fix conflict bug

    //config file

    Ok(())
  }

  fn ensure_path(&self)-> PathBuf {
    match &self.path {
      Some(path)=> path.into(),
      None=> {
        let question=Question::input("Project name").default("my-app").build();
        
        prompt_one(question).unwrap().as_string().unwrap().into()
      }
    }
  }
}




