//! Module for low level packet server, this module only aims to provide a generic
//! API for packet sending and receiving. Check out `protocol` module for advanced
//! use of this module.

use std::io::{Cursor, Read, Result as IoResult, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;

use crate::util::ReadCounter;

use crossbeam_channel::{bounded, Receiver, Sender, TryRecvError, unbounded};

pub mod serial;
use serial::*;


/// A raw packet data with its ID and destination address, it's only used for interfacing
/// with `PacketServer`, to avoid re-encoding packets manually you must use higher level
/// structures and functions in the `protocol` module.
#[derive(Debug)]
pub struct RawPacket {
    pub addr: SocketAddr,
    pub id: u16,
    pub data: Vec<u8>
}

impl RawPacket {

    /// Create a new raw packet with the given address and ID with empty data.
    pub fn blank(addr: SocketAddr, id: u16) -> Self {
        Self {
            addr,
            id,
            data: Vec::new()
        }
    }

    /// Create a new read-only cursor for this raw packet's data.
    pub fn get_cursor(&self) -> Cursor<&Vec<u8>> {
        Cursor::new(&self.data)
    }


    pub fn get_cursor_mut(&mut self) -> Cursor<&mut Vec<u8>> {
        Cursor::new(&mut self.data)
    }

}

#[derive(Debug)]
pub enum Event {
    Connected(SocketAddr),
    Packet(RawPacket),
    Disconnected(SocketAddr)
}

#[derive(Debug)]
enum InternalEvent {
    Connected(SocketAddr, TcpStream),
    Disconnected(SocketAddr)
}

#[derive(Debug)]
enum Request {
    Disconnect(SocketAddr),
    Packet(RawPacket)
}


/// The server interface from which you send and receive packets. This is the main communication
/// interface of the server.
#[derive(Debug)]
pub struct PacketServer {
    event_receiver: Receiver<Event>,
    request_sender: Sender<Request>,
}

impl PacketServer {

    pub fn bind(ip: &str, port: u16) -> IoResult<Self> {

        let (
            listener,
            event_receiver,
            internal_event_receiver
        ) = ServerListener::bind(ip, port)?;

        let (request_sender, request_receiver) = unbounded();

        let client_encoder = ClientEncoder {
            request_receiver,
            internal_event_receiver,
            clients: HashMap::new()
        };

        std::thread::spawn(move || listener.run());
        std::thread::spawn(move || client_encoder.run());

        Ok(Self {
            event_receiver,
            request_sender
        })

    }

    pub fn try_recv_event(&self) -> Option<Event> {
        self.event_receiver.try_recv().ok()
    }

    pub fn recv_event(&self) -> Event {
        self.event_receiver.recv().unwrap()
    }

    pub fn send(&self, packet: RawPacket) {
        // SAFETY: Unwrap should be safe because the ClientEncoder thread should not drop
        //  until this structure is dropped.
        self.request_sender.send(Request::Packet(packet)).unwrap();
    }

    pub fn kick(&self, addr: SocketAddr) {
        // SAFETY: Same as `send_raw`.
        self.request_sender.send(Request::Disconnect(addr)).unwrap();
    }

}


/// Internal structure that is moved to a single thread and accept incoming clients.
struct ServerListener {
    listener: TcpListener,
    event_sender: Sender<Event>,
    internal_event_sender: Sender<InternalEvent>
}

impl ServerListener {

    fn bind(ip: &str, port: u16) -> IoResult<(Self, Receiver<Event>, Receiver<InternalEvent>)> {

        let listener = TcpListener::bind((ip, port))?;
        let (event_sender, event_receiver) = bounded(256);
        let (internal_event_sender, internal_event_receiver) = unbounded();

        Ok((Self {
            listener,
            event_sender,
            internal_event_sender,
        }, event_receiver, internal_event_receiver))

    }

    fn run(self) {
        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    if !self.handle(stream, addr) {
                        break
                    }
                },
                Err(_) => break
            }
        }
    }

    fn handle(&self, stream: TcpStream, addr: SocketAddr) -> bool {

        if let Ok(write_stream) = stream.try_clone() {

            if let Err(_) = self.event_sender.send(Event::Connected(addr)) {
                // If the PacketServer structure was dropped, its event_receiver was dropped
                // and we should stop this thread so we return false.
                return false;
            }

            // SAFETY: ClientEncoder should live as long as this structure lives.
            self.internal_event_sender.send(InternalEvent::Connected(addr, write_stream)).unwrap();

            let worker = ClientDecoder {
                stream,
                addr,
                event_sender: self.event_sender.clone(),
                internal_event_sender: self.internal_event_sender.clone()
            };

            std::thread::spawn(move || worker.run());

        }

        true

    }

}


/// A structure owned by a dedicated thread for each client that read incoming packets
/// and send them to a common event channel that is read by
struct ClientDecoder {
    stream: TcpStream,
    addr: SocketAddr,
    event_sender: Sender<Event>,
    internal_event_sender: Sender<InternalEvent>
}

impl ClientDecoder {

    fn run(mut self) {

        loop {

            match self.fetch() {
                Ok(packet) => {
                    if let Err(_) = self.event_sender.send(Event::Packet(packet)) {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }

        }

        let _ = self.event_sender.send(Event::Disconnected(self.addr));
        let _ = self.internal_event_sender.send(InternalEvent::Disconnected(self.addr));

    }

    fn fetch(&mut self) -> IoResult<RawPacket> {

        let mut stream = ReadCounter::new(&self.stream);

        let packet_len = stream.read_var_int()? as usize;

        let (
            packet_id_len,
            packet_id
        ) = stream.count_with(|stream| {
            stream.read_var_int().map(|i| i as u16)
        })?;

        // We must subtract length of the packet id var int because
        // it is counted in the packet length.
        let mut data = vec![0; packet_len - packet_id_len];
        stream.read_exact(&mut data[..])?;

        Ok(RawPacket {
            addr: self.addr,
            id: packet_id,
            data
        })

    }

}


/// A structure owned by a single thread (per server) that accept incoming packet or
/// disconnection request and send them to clients.
struct ClientEncoder {
    request_receiver: Receiver<Request>,
    internal_event_receiver: Receiver<InternalEvent>,
    clients: HashMap<SocketAddr, TcpStream>
}

impl ClientEncoder {

    fn run(mut self) {

        let mut buffer = Cursor::new(Vec::<u8>::new());

        'a: while let Ok(request) = self.request_receiver.recv() {

            loop {
                match self.internal_event_receiver.try_recv() {
                    Ok(InternalEvent::Connected(addr, stream)) => {
                        self.clients.insert(addr, stream);
                    }
                    Ok(InternalEvent::Disconnected(addr)) => {
                        self.clients.remove(&addr);
                    }
                    Err(TryRecvError::Empty) => {
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        break'a;
                    }
                }
            }

            match request {
                Request::Disconnect(addr) => {
                    if let Some(stream) = self.clients.get(&addr) {
                        // Let's ignore the error when we shut down.
                        let _ = stream.shutdown(Shutdown::Both);
                    }
                }
                Request::Packet(packet) => {
                    if let Some(stream) = self.clients.get_mut(&packet.addr) {

                        // SAFETY: We can unwrap because the internal buffer is backed by a
                        // vec that should not panic (we don't care, for now, of allocation
                        // issue), it can but for now let's ignore it.
                        buffer.set_position(0);
                        buffer.write_var_int(packet.id as i32).unwrap();

                        let id_len = buffer.position() as usize;
                        let data_len = packet.data.len();

                        // TODO: Do not ignore these results in the future.
                        let _ = stream.write_var_int((id_len + data_len) as i32);
                        let _ = stream.write_all(&buffer.get_ref()[..id_len]);
                        let _ = stream.write_all(&packet.data[..data_len]);

                    }
                }
            }

        }

    }

}
