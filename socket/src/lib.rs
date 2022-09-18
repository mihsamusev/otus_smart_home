#[repr(C)]
pub struct Socket {
    is_on: bool,
    power: f32,
}

#[no_mangle]
pub extern "C" fn set_on(socket: Socket) -> Socket {
    Socket {
        is_on: true,
        ..socket
    }
}

#[no_mangle]
pub extern "C" fn set_off(_socket: Socket) -> Socket {
    Socket {
        is_on: false,
        power: 0.0,
    }
}

#[no_mangle]
pub extern "C" fn update_power(socket: Socket) -> Socket {
    let mut power = 0.0;
    if socket.is_on {
        power = rand::random::<f32>();
    }
    Socket { power, ..socket }
}
