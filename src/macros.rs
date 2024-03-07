
macro_rules! deno_option_type {
  ($name:ident {
    $(
      #[flag=$flag:literal]
      $vis:vis $field:ident: $typ:ty=$default:expr
    ),*
  })=> {
    #[derive(serde::Deserialize,serde::Serialize,Debug)]
    #[serde(rename_all="kebab-case")]
    pub struct $name {
      pub no_check: Option<bool>,
      pub import_map: Option<Box<std::path::Path>>,
      pub no_remote: Option<bool>,
      pub no_npm: Option<bool>,
      pub node_modules_dir: Option<Box<[Box<str>]>>,
      pub vendor: Option<bool>,
      pub config: Option<Box<std::path::Path>>,
      pub reload: Option<Box<[Box<str>]>>,
      pub lock: Option<Box<std::path::Path>>,
      pub lock_write: Option<bool>,
      pub no_lock: Option<bool>,
      pub cert: Option<Box<std::path::Path>>,
      pub quiet: Option<bool>,
      pub unsafely_ignore_certificate_errors: Option<Box<[Box<str>]>>,
      pub no_prompt: bool,
      pub catch_only: Option<bool>,
      pub location: Option<Box<std::path::Path>>,
      pub v8_flags: Option<Box<[Box<str>]>>,
      pub seed: Option<u128>,
      pub check: Option<bool>,
      pub env: Option<Box<std::path::Path>>,
      $(
        $vis $field: $typ,
      )*
    }


    impl Default for $name {
      fn default()-> Self {
        Self {
          no_prompt: true,
          // none
          no_check: None,
          catch_only: None,
          import_map: None,
          no_remote: None,
          no_npm: None,
          node_modules_dir: None,
          vendor: None,
          config: None,
          reload: None,
          lock: None,
          lock_write: None,
          no_lock: None,
          cert: None,
          quiet: None,
          unsafely_ignore_certificate_errors: None,
          location: None,
          v8_flags: None,
          seed: None,
          check: None,
          env: None,
          $(
            $field: $default,
          )*
        }
      }
    }

    impl crate::config::ToArgs for $name {
      fn to_flags(&self)-> std::collections::LinkedList<Option<Box<str>>> {
        use crate::config::Parse;
        std::collections::LinkedList::from_iter([
          self.no_check.parse("--no-check"),
          self.import_map.parse("--import-map"),
          self.no_remote.parse("--no-remote"),
          self.no_npm.parse("--no-npm"),
          self.node_modules_dir.parse("--node-modules-dir"),
          self.vendor.parse("--vendor"),
          self.config.parse("--config"),
          self.reload.parse("--reload"),
          self.lock.parse("--lock"),
          self.lock_write.parse("--lock-write"),
          self.no_lock.parse("--no-lock"),
          self.cert.parse("--cert"),
          self.quiet.parse("--quiet"),
          self.unsafely_ignore_certificate_errors.parse("--unsafely-ignore-certificate-errors"),
          self.no_prompt.parse("--no-prompt"),
          self.catch_only.parse("--catch-only"),
          self.location.parse("--location"),
          self.v8_flags.parse("--v8-flags"),
          self.seed.parse("--seed"),
          self.check.parse("--check"),
          self.env.parse("--env"),
          $(
            self.$field.parse($flag),
          )*
        ])
      }
    }
  };
}


#[macro_export]
macro_rules! command {
  (
    cmd: $cmd:literal,
    config: $config:expr
  )=> {
    let args=crate::config::ToArgs::to_flags(&$config).into_iter().filter_map(crate::api::to_boxed_os_str);
    let mut cmd=tokio::process::Command::new("deno");
    cmd.arg($cmd);
    cmd.args(args);
    // # Example "./proton-src/main.ts"
    cmd.arg(format!("{}.{}",$config.main.unwrap_or(crate::MAIN.into()),$config.language.unwrap_or_default().extension()));

    match cmd.spawn()?.wait().await {
      Err(err)=> Err(std::io::Error::new(err.kind(),"Some error occured while running deno.\nTry upgrading to the latest version of deno")),
      _=> Ok(()),
    }
  };
}


#[macro_export]
macro_rules! url {
  ($template:expr=> $lang:expr)=> {
    format!("https://github.com/proton-xd-templates/{}-template-{}",$template,$lang).into_boxed_str()
  };
}

