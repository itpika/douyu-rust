use std::{sync::{Arc, Mutex}, net, io::{Read, self}, process::exit, collections::HashMap};

pub const MSG_LEN_SIZE: usize = 4;
pub const MSG_TYPE_SIZE: usize = 2;
pub const CRYPTO_CTX_SIZE: usize = 1;
pub const KEEP_CTX_SIZE: usize = 1;
pub struct MetaMsg<'a> {
    key: &'a str,
    value: &'a str,
}
impl<'a> MetaMsg<'a> {
    pub fn new(key: &'a str, value: &'a str) -> MetaMsg<'a> {
        MetaMsg {
            key,
            value
        }
    }
    pub fn stt(data: Vec<MetaMsg>) -> String {
        let ss: Vec<String> = data.iter().map(|v| {
            format!("{}@={}", v.key, v.value)
        }).collect();
        ss.join("/")
    }
    pub fn to_bytes(metas: Vec<MetaMsg>) -> Vec<u8> {
        let content = MetaMsg::stt(metas);
        let mut content = content.as_bytes().to_vec();
        content.append(&mut vec![0]);
        let content_len = content.len();
        let head_len = MSG_LEN_SIZE + MSG_TYPE_SIZE + CRYPTO_CTX_SIZE + KEEP_CTX_SIZE;
        let total_len = content_len + head_len;
        let mut head = vec![];
        head.append(&mut total_len.to_le_bytes()[0..4].repeat(2).to_vec());
        // println!("{}", head.len());
        let msg_type: u32  = 689;
        head.append(&mut msg_type.to_le_bytes()[0..2].to_vec());
        // println!("{}", head.len());
        head.append(&mut vec![0, 0]);
        // println!("{}", head.len());
        head.append(&mut content);
        // println!("{}", head.len());
        // head.as_slice()
        head
    }
    pub fn to_meta_msg(msg: &str) -> HashMap<String, String> {
        // println!("s {}", msg);
        let mut m:HashMap<String, String> = HashMap::new();
        for v in msg.split("/").collect::<Vec<&str>>() {
            if v.trim().len() == 0 {
                continue;
            }
            let line = v.split("@=").collect::<Vec<&str>>();
            if line.len() < 2 {
                log::warn!("line {:?}", line);
                log::warn!("msg {:?}", msg);
                break;
            }
            m.insert(line[0].to_string(), line[1].to_string());
        }
        m
    }
}

pub fn con_readmore(con: &Arc<Mutex<net::TcpStream>>, data: &mut [u8]) -> usize {
    let mut read_n :usize = 0;
    loop {
        if let Ok(ref mut mutex) = con.try_lock() {
            read_n += mutex.read(data).unwrap_or_else(|err| {
                if err.kind() == io::ErrorKind::TimedOut {
                    // println!("time out");
                    return 0;
                }
                println!("read error {}", err);
                exit(1)
            });
            if read_n == 0 {
                continue;
            }
            break;
        } else {
            continue;
        }
    }
    if read_n != data.len() {
        // log::info!("read_n {}, data_len {}", read_n, data.len());
        let (_, last) = data.split_at_mut(read_n);
        read_n += con_readmore(con, last);
        // log::info!("retry read_n {}", read_n);
    }
    return read_n;
}
pub fn con_read(con: &Arc<Mutex<net::TcpStream>>, data: &mut [u8]) -> usize {
    loop {
        let mut lock = con.try_lock();
        if let Ok(ref mut mutex) = lock {
            let n = mutex.read(data).unwrap_or_else(|err| {
                if err.kind() == io::ErrorKind::TimedOut {
                    // println!("time out");
                    return 0;
                }
                println!("read error {}", err);
                exit(1)
            });
            if n == 0 {
                continue;
            }
            return n;
        } else {
            continue;
        }
    }
}