mod types;
use serde_json::{from_str, to_string_pretty};
use std::{
  io::{Error, ErrorKind},
  thread, time,
};
use types::{get_usr_info, Req2Data, Res1Data, Res2Data, UsrInfo};

async fn login(
  client: &reqwest::Client,
  usr_info: &UsrInfo,
) -> Result<(), Box<dyn std::error::Error>> {
  let res1 = client
    .get("http://10.255.255.46/api/v1/ip")
    .send()
    .await?
    .json::<Res1Data>()
    .await?;
  println!("{}", to_string_pretty(&res1).unwrap());
  if res1.code != 200 {
    panic!("请求错误");
  };
  let req2: Req2Data = Req2Data {
    username: usr_info.username.as_str(),
    password: usr_info.password.as_str(),
    channel: usr_info.channel.as_str(),
    ifautologin: "1",
    pagesign: "secondauth",
    usripadd: res1.data,
  };
  let res2 = client
    .post("http://10.255.255.46/api/v1/login")
    .json(&req2)
    .send()
    .await?
    .text() // 不直接 JSON 是因为中文编码问题，需要先转一遍 UTF8
    .await?;
  let res2_data = from_str::<Res2Data>(&res2).unwrap();
  println!("{}", to_string_pretty(&res2_data).unwrap());

  if res2_data.code == 200 {
    println!("登录成功");
    thread::sleep(time::Duration::from_millis(500));
    return Ok(());
  } else {
    return Err(Error::new(ErrorKind::Other, res2_data.message).into());
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let usr_info = get_usr_info();
  let client = reqwest::Client::new();
  loop {
    let res = login(&client, &usr_info).await;
    match res {
      Ok(_) => break,
      Err(msg) => {
        println!("{} 登录失败，5秒后重试", msg);
        thread::sleep(time::Duration::from_secs(5));
      }
    }
  }
  Ok(())
}
