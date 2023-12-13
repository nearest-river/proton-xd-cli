
mod config;
mod unstable;
mod permission;
mod compiler_options;

use unstable::*;
use permission::*;
use compiler_options::*;

pub(crate) use config::*;


use super::Writer;
use std::path::Path;




pub(crate) type Str=Box<str>;
pub(crate) type Array<T>=Box<[T]>;


pub(crate) trait ToArgs {
  fn to_flags(&self)-> std::collections::LinkedList<Option<Str>>;
}


pub(in crate::ser::config) trait Parse {
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
  use crate::ser::config::ToArgs;


  #[tokio::test]
  async fn xd() {
    crate::config::Config::default().save("./proton-config.json").await.unwrap()
  }

  #[tokio::test]
  async fn read() {
    println!("{:#?}",crate::config::Config::find_config_file().await.unwrap().to_flags())
  }
}
