use std::{sync::{Arc, Mutex, mpsc::{self}}, net::{TcpStream}, time::Duration, io::{Write}, process::exit, thread, os, env::consts::OS};

use crate::{Conf, protocal::{MetaMsg, self}, utils};


pub struct Room {
    room_id: u64,
    stream: Arc<Mutex<TcpStream>>,
    rc_msg: mpsc::Receiver<String>,
    tc_msg: mpsc::Sender<String>,
}

impl Room {
    pub fn new(room_id: u64, conf: Conf) -> Room {
        let stream = TcpStream::connect(conf.get_addr()).unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(1))).expect("set read timeout error");
        stream.set_write_timeout(None).expect("set write timeout error");
        let (tc, rc) = mpsc::channel();
        Room { room_id, stream: Arc::new(Mutex::new(stream)), rc_msg: rc, tc_msg: tc }
    }
    pub fn login(&self) -> Result<(), std::io::Error> {
        let login_data = MetaMsg::to_bytes(vec![
            MetaMsg::new("type", "loginreq"), 
            MetaMsg::new("roomid", &self.room_id.to_string()),
            MetaMsg::new("time", &utils::now_sec().to_string())
        ]);
        loop {
            let mut lock =  self.stream.try_lock();
            if let Ok(ref mut mutex) = lock {
                if let Err(e) = mutex.write(login_data.as_slice()) {
                    println!("{}", e);
                    return Err(e);
                }
                return Ok(());
            } else {
                continue;
            }
        }
    }
    pub fn ping(&self) {
        let stream = self.stream.clone();
        thread::spawn(move || {
            loop {
                loop {
                    let mut lock =  stream.try_lock();
                    if let Ok(ref mut mutex) = lock {
                        if let Err(e) = mutex.write(MetaMsg::to_bytes(vec![
                            MetaMsg::new("type", "mrkl"), 
                        ]).as_slice()) {
                            println!("ping error: {}", e);
                        }
                        break;
                    } else {
                        continue;
                    }
                }
                // log::info!("ping");
                thread::sleep(Duration::from_secs(20));
            }
        });
    }
    pub fn join_group(&self) {
        loop {
            let mut lock =  self.stream.try_lock();
            if let Ok(ref mut mutex) = lock {
                if let Err(e) = mutex.write(MetaMsg::to_bytes(vec![
                    MetaMsg::new("type", "joingroup"),
                    MetaMsg::new("rid", &self.room_id.to_string()),
                    MetaMsg::new("time", &utils::now_sec().to_string()),
                ]).as_slice()) {
                    println!("ping error: {}", e);
                }
                break;
            } else {
                continue;
            }
        }
    }
    pub fn revice_msg(&self) {
        let stream_clone = Arc::clone(&self.stream);
        let tc_msg = self.tc_msg.clone();
        thread::spawn(move || {
            loop {
                let head_data = &mut [0; protocal::MSG_LEN_SIZE*2 + protocal::MSG_TYPE_SIZE + protocal::CRYPTO_CTX_SIZE + protocal::KEEP_CTX_SIZE][..];
                // let head_data = Box::new(head_data);
                // let n = protocal::con_read(&stream_clone, *head_data);
                let n = protocal::con_readmore(&stream_clone, head_data);
                if n != head_data.len() {
                    log::error!("read len error {}", n);
                    exit(1);
                }
                // println!("head: {:?}", head_data);
                // log::warn!("head_data {:?}", head_data);
                let body_len = u32::from_le_bytes([head_data[0], head_data[1], head_data[2], head_data[3]]) as usize - (protocal::MSG_LEN_SIZE + protocal::MSG_TYPE_SIZE + protocal::CRYPTO_CTX_SIZE + protocal::KEEP_CTX_SIZE) as usize;
                // log::warn!("body len {}", body_len);
                let mut body = [0].repeat(body_len);
                // let body = Box::new(body.repeat(body_len).as_mut_slice());
                // let n = protocal::con_read(&stream_clone, *body);
                let n = protocal::con_readmore(&stream_clone, &mut body);
                if n != body.len() {
                    log::warn!("read length error: {}, {}", n, body.len());
                }
                let body = String::from_utf8_lossy(&body);
                let body = &body[0..body.len()-1];
                if let Err(err) =  tc_msg.send(body.to_string()) {
                    log::error!("send msg error: {}", err);
                }
            }
        });
        
        let mut num = 0;
        loop {
            let msg = self.rc_msg.recv().unwrap();
            num += 1;
            let data = MetaMsg::to_meta_msg(&msg);
            if let Some(v) = data.get("type") {
                match &v[..] {
                    "loginres" => {
                        log::info!("success login Room {}", self.room_id);
                        self.join_group();
                    }
                    "chatmsg" => {
                        log::info!("LV[{}] {}: {} {}", data["level"], data["nn"], data["txt"], num);
                    }
                    "uenter" => {
                        log::info!("LV[{}] {} 进入了直播间 {}", data["level"], data["nn"], num);
                    }
                    "dgb" => {
                        log::info!("LV[{}] {} 赠送了{} 个礼物 {}连击 {}", data["level"], data["nn"], data["gfcnt"], data["hits"], num);
                    }
                    _ => {
                        log::info!("type: {} {}", v, num);
                    }
                }
            } else {
                log::info!("unknown message: {}", msg);
            }
            // log::info!("-----------------------------")
        }
    }
    
}