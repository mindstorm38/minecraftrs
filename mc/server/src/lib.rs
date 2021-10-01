use std::io::{Result as IoResult, Read, BufReader, BufWriter, Cursor, ErrorKind};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::collections::VecDeque;

use crossbeam_channel::{bounded, Sender, Receiver};

use crate::ext::PacketReadExt;

pub mod ext;


/// A structure handling a server client, this structure is the interface used for final
/// communications with the client.
pub struct ServerClient {
    stream: TcpStream,
    addr: SocketAddr
}

pub struct RawPacket {
    pub addr: SocketAddr,
    pub id: usize,
    pub data: Cursor<Vec<u8>>
}


/// The server interface from which you send and receive packets.
pub struct Server {
    packets: VecDeque<RawPacket>,
    packet_receiver: Receiver<RawPacket>
}

impl Server {

    pub fn bind(ip: &str, port: u16) -> IoResult<Self> {

        let (
            listener,
            packet_receiver
        ) = ServerListener::bind(ip, port)?;

        std::thread::spawn(move || {
            listener.run();
        });

        Ok(Self {
            packets: VecDeque::new(),
            packet_receiver
        })

    }

    pub fn wait_packet(&mut self) -> RawPacket {
        self.packet_receiver.recv().unwrap()
    }

    pub fn poll_packet(&mut self) -> Option<RawPacket> {
        self.packet_receiver.try_recv().ok()
    }

}



struct ServerListener {
    listener: TcpListener,
    packet_sender: Sender<RawPacket>
}

impl ServerListener {

    fn bind(ip: &str, port: u16) -> IoResult<(Self, Receiver<RawPacket>)> {

        let listener = TcpListener::bind((ip, port))?;
        let (packet_sender, packet_receiver) = bounded(128);

        Ok((Self {
            listener,
            packet_sender
        }, packet_receiver))

    }

    fn run(mut self) {

        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    self.handle(stream, addr);
                },
                Err(_) => break
            }
        }

    }

    fn handle(&self, stream: TcpStream, addr: SocketAddr) {

        let packet_sender = self.packet_sender.clone();

        std::thread::spawn(move || {

            let worker = ClientWorker {
                stream,
                addr,
                packet_sender
            };

            worker.run()

        });

    }

}


struct ClientWorker {
    stream: TcpStream,
    addr: SocketAddr,
    packet_sender: Sender<RawPacket>
}

impl ClientWorker {

    fn run(mut self) {

        println!("[{}] Fetching...", self.addr);

        loop {

            match self.fetch() {
                Ok(packet) => {
                    if let Err(_) = self.packet_sender.send(packet) {
                        println!("[{}] Crossbeam channel closed.", self.addr);
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }

        }

        println!("[{}] Closed.", self.addr);

    }

    fn fetch(&mut self) -> IoResult<RawPacket> {

        let packet_len = self.stream.read_var_int()? as usize;

        let mut data = vec![0; packet_len];
        self.stream.read_exact(&mut data[..])?;

        let mut cursor = Cursor::new(data);
        let packet_id = cursor.read_var_int()? as usize;

        Ok(RawPacket {
            addr: self.addr,
            id: packet_id,
            data: cursor
        })

    }

}