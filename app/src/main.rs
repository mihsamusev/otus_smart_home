pub mod device;
use crate::device::Socket;

fn main() {
    let mut sock = Socket::default();

    sock.set_on();
    println!("{}", sock);

    for _ in 1..=3 {
        sock.update_power();
        println!("{}", sock);
    }

    sock.set_off();
    println!("{}", sock);
}
