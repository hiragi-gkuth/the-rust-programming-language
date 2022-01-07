#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl IpAddr {
    fn mask(&self, mask: u8) -> IpAddr {
        match self {
            IpAddr::V4 => {
                
            }
            IpAddr::V6 => {

            }
        }
    }
}

impl Message {
    fn call(&self) {
    }
}

fn use_of_enum() {
    let router = Some(IpAddr::V4(192, 168, 0, 1));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let router = router.unwrap();
    let masked = router.mask(8);
    let m = Message::Quit;
    m.call();

    dbg!(&home);
    dbg!(&loopback);
}
