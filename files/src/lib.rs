pub mod io;

pub struct MarkerByte(u8);
pub enum MessageType {
    Raw,
    File,
    Bytes,
    Image,
    Video,
    Reference
}

fn get_marker(message : Vec<u8>) -> MarkerByte {
    return MarkerByte(*message.get(0).unwrap());
}

fn bound_marker(marker : MarkerByte) -> MessageType {
    return match marker.0 {
        1 => MessageType::Raw,
        2 => MessageType::File,
        3 => MessageType::Video,
        4 => MessageType::Image,
        5 => MessageType::Bytes,
        6 => MessageType::Reference,
        _ => panic!("Wrongfull marker")
    }
}
