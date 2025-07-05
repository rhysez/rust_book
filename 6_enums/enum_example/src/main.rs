// Enums allow us to define different variants of data.
// An enum value can only be one of it's variants.
// In this case, an IP Address can only ever be either V4 or V6.
// So, an enum is appropriate to use for this.

// We can inject data directly into our enum variants like so: V4(String)
// This expresses that both V4 and V6 will have associated string values.
enum IpAddrKind {
    // Version 4 IP addresses are made up of four numeric segments.
    // So, we can create any V4 IP address we want by passing in four unsigned 8-bit integers.
    V4(u8, u8, u8, u8),
    V6(String)
}

// We can see here that enums can store many types of values.
// This is powerful, because we can store multiple shapes of data within one type (in this case, Message)
// To accomplish this with structs, we would need to use multiple structs.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// We can even define methods on enums, just like we can with structs.
impl Message {
    fn call(&self) {
        // method body
    }
}

fn main() {
    // ::V4() and ::V6() functions both construct a new instance of the IpAddrKind enum.
    // They are associated functions, as discussed in chapter 5.
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
