use crate::err::{Error, ResultExt};
use miniserde::{json, Deserialize};
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub fn send<I>(args: I) -> Result<Option<String>, Error>
where
  I: IntoIterator,
  I::Item: AsRef<[u8]>,
{
  let mut path = "/tmp/yabai_".to_string();

  path.push_str(&env::var("USER").ok_or(Error::NoUserEnv)?);
  path.push_str(".socket");

  let mut buf = Vec::new();
  let mut stream = UnixStream::connect(&path).ok_or(Error::UnixConn)?;

  extend_from_args(args, &mut buf);
  stream.write_all(&buf).ok_or(Error::UnixWrite)?;
  buf.clear();

  let read = stream.read_to_end(&mut buf).ok_or(Error::UnixRead)?;
  if read == 0 {
    return Ok(None);
  }

  if buf[0] == 0x07 {
    let message = &buf[1..];
    let message = String::from_utf8_lossy(message);

    return Err(Error::Yabai(message.to_string()));
  }

  Ok(Some(String::from_utf8_lossy(&buf[..read]).to_string()))
}

pub fn query_windows() -> Result<Vec<Window>, Error> {
  let json = send(["query", "--windows"])?;
  let json = json.ok_or(Error::Json)?;

  Ok(json::from_str(&json)?)
}

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq)]
pub struct Window {
  pub id: u32,
  #[serde(rename = "has-focus")]
  pub has_focus: bool,
  #[serde(rename = "is-minimized")]
  pub is_minimized: bool,
  #[serde(rename = "is-floating")]
  pub is_floating: bool,
}

fn extend_from_args<I>(args: I, buf: &mut Vec<u8>)
where
  I: IntoIterator,
  I::Item: AsRef<[u8]>,
{
  let mut len = 0u32;

  // Reserve enough bytes for length prefix
  buf.extend_from_slice(&[0; 4]);

  for arg in args {
    buf.extend_from_slice(arg.as_ref());
    len += arg.as_ref().len() as u32;

    buf.push(0);
    len += 1;
  }

  // Write trailing nul byte
  len += 1;
  buf.push(0);

  // Replace length
  buf[0..4].copy_from_slice(&len.to_le_bytes());
}
