
mod consts;
mod app;


pub mod config;
pub(in self) mod dev;
pub(in self) mod new;
pub(in self) mod init;
pub(in self) mod build;


pub use app::*;
pub use consts::*;


pub trait Operation {
  async fn run(self)-> std::io::Result<()>;
}



