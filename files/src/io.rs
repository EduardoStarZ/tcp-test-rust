use crate::{MarkerByte, MessageType};

pub fn write(message : String, marker : MarkerByte) -> Vec<u8> {
    let mut bytes : Vec<u8> = message.bytes().collect::<Vec<u8>>();

    bytes.insert(0, marker.0);
    return bytes;
}

pub fn read(mut buffer : String, message : &mut Vec<u8>) -> MarkerByte {
    message.reverse();
    let message_marker : MarkerByte = MarkerByte(message.pop().unwrap());
    message.reverse();


    buffer = String::from_utf8(message.to_vec()).unwrap();

    return message_marker;
}
