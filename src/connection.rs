use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::config::ConfigManager;
use crate::packet;
use crate::packet::{MutablePacketByteBuf, PacketByteBuf};

pub struct PlayerConnection {
    conn: TcpStream,
    name: String,
    state: u8,
    protocol: u32,
    connected: bool,
    hostname: String,
    port: u16,
    wrong_packet_count: u8,
    config: Arc<ConfigManager>
}

impl PlayerConnection {
    pub async fn new(conn: TcpStream, config: Arc<ConfigManager>) -> Self {
        PlayerConnection {
            conn,
            name: String::from(""),
            state: 0,
            protocol: 0,
            connected: true,
            hostname: "".to_string(),
            port: 0,
            wrong_packet_count: 0,
            config
        }
    }

    pub async fn handle(&mut self) {
        while self.connected {
            let buff = &mut [0u8; 1024];

            match self.conn.read(buff).await {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    let mut p = packet::PacketByteBuf::new(buff);
                    if len as u32 != p.read_varint() + p.get_reader_idx() {
                        println!("WARN! Got packet with wrong lenght field!");
                        self.wrong_packet_count += 1;
                        //TODO make it configurable
                        if self.wrong_packet_count > 3 {
                            //Disconnect people with bad connections
                            self.disconnect();
                        }
                    }

                    self.handle_packet(p).await;
                }
                Err(_e) => {
                    break;
                }
            }
        }
    }

    fn handle_handshake(&mut self, mut p: PacketByteBuf) {
        if p.read_varint() != 0 {
            self.disconnect();
            return;
        }
        self.protocol = p.read_varint();
        self.hostname = std::str::from_utf8(p.read_string().as_slice()).unwrap().to_string();
        self.port = p.read_ushort();
        self.state = p.read_varint() as u8;
    }

    pub async fn handle_status(&mut self, mut p: PacketByteBuf) {
        match p.read_varint() {
            0 => {
                //Status Response
                let mut to_send = packet::MutablePacketByteBuf::new();
                to_send.write_varint(0x00);
                to_send.write_string(serde_json::to_string(self.config.get_config().get_status_response()).unwrap().as_str());

                self.send_packet(to_send).await;
            }
            1 => {
                //Pong!
                let mut send = packet::MutablePacketByteBuf::new();
                send.write_varint(0x01);
                //TODO: crappy long writer
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());
                send.write_byte(p.read_byte());

                self.send_packet(send).await;
            }
            _ => {}
        }
    }

    async fn handle_login(&mut self, mut p: PacketByteBuf) {
        if p.read_varint() != 0x00 {
            self.disconnect();
            return;
        }
        let name = p.read_string();
        let username = std::str::from_utf8(name.as_slice()).unwrap();
        self.name = String::from(username);

        let mut send = packet::MutablePacketByteBuf::new();
        send.write_varint(0x00);
        send.write_string(serde_json::to_string(self.config.get_config().get_kick_message()).unwrap().as_str());

        self.send_packet(send).await;
    }

    pub async fn send_packet(&mut self, mut p: MutablePacketByteBuf) {
        if !self.connected {
            return;
        }
        let b = p.get_bytes();
        self.conn.write(b.as_slice()).await.unwrap();
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub async fn handle_packet(&mut self, p: PacketByteBuf) {
        match self.state {
            0 => self.handle_handshake(p),
            1 => self.handle_status(p).await,
            2 => self.handle_login(p).await,
            _ => {}
        };
    }
}