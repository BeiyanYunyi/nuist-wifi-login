use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(name = "NUIST WiFi Login")]
#[clap(about = "南信带 WiFi 登录, Rust 版")]
struct Args {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// 中国移动
  #[clap(arg_required_else_help = true)]
  CMCC {
    /// 手机号
    #[clap(short, long)]
    username: String,
    /// 密码
    #[clap(short, long)]
    password: String,
  },

  /// 中国电信
  #[clap(arg_required_else_help = true)]
  ChinaNet {
    /// 手机号
    #[clap(short, long)]
    username: String,
    /// 密码
    #[clap(short, long)]
    password: String,
  },

  /// 中国联通
  #[clap(arg_required_else_help = true)]
  ChinaUnicom {
    /// 手机号
    #[clap(short, long)]
    username: String,
    /// 密码
    #[clap(short, long)]
    password: String,
  },

  /// 信带土著
  #[clap(arg_required_else_help = true)]
  School {
    /// 手机号
    #[clap(short, long)]
    username: String,
    /// 密码
    #[clap(short, long)]
    password: String,
  },
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Res1Data {
  pub code: u8,
  pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Req2Data<'a> {
  pub username: &'a str,
  pub password: &'a str,
  pub channel: &'a str,
  pub ifautologin: &'a str,
  pub pagesign: &'a str,
  pub usripadd: String,
}

pub struct UsrInfo {
  pub username: String,
  pub password: String,
  pub channel: String,
}

#[derive(Deserialize, Serialize)]
pub struct Res2InnerData {
  reauth: bool,
  username: String,
  balance: String,
  duration: String,
  outport: String,
  totaltimespan: String,
  usripadd: String,
}

#[derive(Deserialize, Serialize)]
pub struct Res2Data {
  pub code: u8,
  pub message: String,
  data: Option<Res2InnerData>,
}

pub fn get_usr_info() -> UsrInfo {
  let args = Args::parse();
  let usr_info: UsrInfo = match args.command {
    Commands::School { username, password } => UsrInfo {
      username,
      password,
      channel: String::from("1"),
    },
    Commands::CMCC { username, password } => UsrInfo {
      username,
      password,
      channel: String::from("2"),
    },
    Commands::ChinaNet { username, password } => UsrInfo {
      username,
      password,
      channel: String::from("3"),
    },
    Commands::ChinaUnicom { username, password } => UsrInfo {
      username,
      password,
      channel: String::from("4"),
    },
  };
  return usr_info;
}
