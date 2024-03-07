
mod config;
mod unstable;
mod permission;
mod dev_options;
mod compiler_options;

use unstable::*;
use permission::*;
use dev_options::*;
use compiler_options::*;

pub use config::*;
use std::path::Path;




pub type Str=Box<str>;
pub type Array<T>=Box<[T]>;


pub trait ToArgs {
  fn to_flags(&self)-> std::collections::LinkedList<Option<Str>>;
}


pub(in crate::operation::config) trait Parse {
  fn parse(&self,option: &str)-> Option<Str>;
}

impl<S: Parse> Parse for Option<S> {
  fn parse(&self,option: &str)-> Option<Str> {
    match self {
      Some(val)=> val.parse(option),
      _=> None
    }
  }
}



impl Parse for bool {
  fn parse(&self,option: &str)-> Option<Str> {
    match self {
      true=> Some(option.into()),
      _=> None
    }
  }
}

impl Parse for u128 {
  fn parse(&self,option: &str)-> Option<Str> {
    Some(format!("{option} {self}").into_boxed_str())
  }
}



impl Parse for Box<Path> {
  fn parse(&self,option: &str)-> Option<Str> {
    Some(format!("{option}={}",self.display()).into_boxed_str())
  }
}

impl Parse for Box<[Str]> {
  fn parse(&self,option: &str)-> Option<Str> {
    Some(format!("{option}=\"{}\"",self.join(",")).into_boxed_str())
  }
}

impl Parse for Str {
  fn parse(&self,option: &str)-> Option<Str> {
    Some(format!("{option} {}",&self).into_boxed_str())
  }
}



#[cfg(test)]
mod tests {
  use crate::operation::config::ToArgs;


  #[tokio::test]
  async fn write() {
    crate::config::Config::default().save("./proton-config.json").await.unwrap()
  }

  #[tokio::test]
  async fn read() {
    println!("{:#?}",crate::config::Config::find_config_file().await.unwrap().to_flags())
  }
}
