mod fs_api;

use tokio::*;
use fs_api::*;
use io::Error;

use std::{
  path::Path,
  ffi::OsStr
};

use crate::{
  TEMPLATES,
  operation::config::Str
};

use requestty::{
  Question,
  prompt_one,
};

use crossterm::style::{
  Color,
  style,
  Stylize
};



pub fn confirm(msg: &str,default: bool)-> bool {
  let q=Question::confirm(msg)
  .default(default)
  .build();

  match prompt_one(q) {
    Ok(res)=> res.as_bool().unwrap(),
    _=> default
  }
}


pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  let path=path.as_ref();

  // Checks whether there are any files at `path`.
  if let None=fs::read_dir(path).await?.next_entry().await? {
    return Ok(());
  }

  let msg=format!(
    "{}: {} is not an empty directory. Do you want to continue?",
    style("warning").with(Color::Yellow),
    path.display()
  );
  let prompt=confirm(&msg,false);

  match prompt {
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display()).as_str()
    )),
    _=> Ok(())
  }
}

/// colors as string.
fn rgb((name,r,g,b): (&str,u8,u8,u8))-> Str {
  style(name).with(Color::Rgb { r,g,b })
  .to_string()
  .into_boxed_str()
}

pub fn ensure_template(template: Option<Str>)-> Str {
  if let Some(template)=template {
    return template;
  }

  let q=Question::select("Choose a Template")
  .choices(TEMPLATES.map(rgb))
  .build();

  let prompt=prompt_one(q).unwrap()
  .try_into_list_item().unwrap();

  // resetting a styled `String` is way too expensive.
  TEMPLATES[prompt.index].0.to_lowercase()
  .into_boxed_str()
}

pub fn ensure_lang<'a>(js: Option<bool>)-> &'a str {
  if let Some(js)=js {
    return lang(js);
  }

  let q=Question::select("Choose your language")
  .choices([
    rgb(("TypeScript",0x2d,0x79,0xc7)),
    rgb(("JavaScript",0xff,0xff,0x0))
  ])
  .build();
  let prompt=prompt_one(q)
  .unwrap()
  .try_into_list_item()
  .unwrap();

  lang(prompt.index!=0)
}

fn lang<'a>(js: bool)-> &'a str {
  match js {
    true=> "js",
    _=> "ts"
  }
}



pub async fn clone_repo<P: AsRef<Path>>(url: &str,into: P)-> io::Result<()> {
  let temp_dir=TempDir::new().await?;

  match git2::Repository::clone(url,&temp_dir.path()) {
    Ok(_)=> temp_dir.move_to(&into,".git*").await,
    Err(err)=> Err(Error::from_raw_os_error(err.raw_code())),
  }
}

pub fn to_boxed_os_str(str: Option<Str>)-> Option<Box<OsStr>> {
  Some(OsStr::new(str?.as_ref()).into())
}