use mc_server::Server;
use mc_server::ext::PacketReadExt;


fn main() {

    let mut server = Server::bind("0.0.0.0", 25565).unwrap();

    loop {

        let mut packet = server.wait_packet();

        println!("[{}] #{} {:02X?}", packet.addr, packet.id, &packet.data.get_ref()[..]);

        match packet.id {
            0 => {
                println!("=> Protocol version: {}", packet.data.read_var_int().unwrap());
            },
            _ => {}
        }

    }

}