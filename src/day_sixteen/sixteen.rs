use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::panic::catch_unwind;
use std::thread::current;

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

fn read_packets(binary_string: String, packets: &mut Vec<String>, start: usize, end: usize) {
    if start == end {
        return;
    }
    let current_packet = &binary_string[start..];
    let id_of_packet = get_id_of_packet(&current_packet);
    let version_of_packet = get_version_of_packet(&current_packet);
    println!("Found packet version: {}", version_of_packet);
    const LITERAL_PACKET_SIZE: usize = 26;

    if id_of_packet == 4 {
        // literal
        packets.push(current_packet[start..LITERAL_PACKET_SIZE].parse().unwrap());
        read_packets(
            binary_string.clone(),
            packets,
            LITERAL_PACKET_SIZE as usize,
            end,
        );
    } else {
        let mut sub_packets = get_sub_packets(&current_packet);
        for sub_packet in &sub_packets {
            packets.push((*sub_packet.clone()).parse().unwrap());
        }
    }
}

pub fn star_one() -> usize {
    let lines = get_lines("src/day_sixteen/sample.txt");

    let binary = binary_from_hex(&lines[0]).chars().collect::<Vec<char>>();
    let binary_string = &binary.iter().collect::<String>();

    let mut different_packets: Vec<String> = vec![];
    let mut current_start_of_packet = 0;
    read_packets(
        binary_string.clone(),
        &mut different_packets,
        0,
        binary_string.len(),
    );

    println!("{:?}", different_packets);
    return 0;
}

/// Decodes a packet to binary representation
fn decode_packet(hex: &str) -> String {
    let packet = binary_from_hex(hex);
    return packet;
}

fn binary_from_hex(hex: &str) -> String {
    return hex.chars().map(to_binary).collect();
}

/// Gets the ID of the packet
fn get_id_of_packet(packet: &str) -> i32 {
    let packet_id = packet[3..6].to_string();
    return i32::from_str_radix(&packet_id, 2).unwrap();
}

fn get_version_of_packet(packet: &str) -> i32 {
    let packet_id = packet[..3].to_string();
    return i32::from_str_radix(&packet_id, 2).unwrap();
}

fn get_sub_packets(packet: &str) -> Vec<String> {
    let mode = packet.chars().nth(6).unwrap();
    let mut subpackets: Vec<String> = vec![];
    // Mode 0 means length is 15 bits wide representing
    // number of bits in subpackets
    // Mode 1 means 11 bits of L determine number of subpackets, each subpacket
    // consisting of 11 bits
    let mut length = 0;
    let packet_indexed = packet.chars().collect::<Vec<char>>();
    if mode == '0' {
        length = i32::from_str_radix(&packet[7..23], 2).unwrap();

        for subpacket in packet_indexed[23..].chunks(length as usize) {
            subpackets.push(subpacket.iter().collect::<String>());
        }
    } else {
        length = i32::from_str_radix(&packet[7..18], 2).unwrap();

        let length_to_search: usize = ((length * 11) + 18) as usize;
        for subpacket in packet_indexed[18..length_to_search].chunks(11 as usize) {
            subpackets.push(subpacket.iter().collect::<String>());
        }
    }

    return subpackets;
}

/// Gets the number encoded in the packet
fn get_number_of_literal_packet(packet: &str) -> i32 {
    let literal = &packet[6..].to_string();
    let mut number = String::new();
    for quad in literal.chars().collect::<Vec<char>>().chunks(5) {
        let bin_string: String = quad[1..5].iter().collect();
        if quad[0] == '0' {
            number.push_str(&bin_string.clone());
            break;
        }
        number.push_str(&bin_string.clone());
    }
    return i32::from_str_radix(&number, 2).unwrap();
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let result = binary_from_hex("FFFF");
        assert_eq!(result, "1111111111111111");
    }

    #[test]
    fn test_decode_packet() {
        let result = decode_packet("D2FE28");
        assert_eq!(result, "110100101111111000101000");
    }

    #[test]
    fn test_get_id() {
        let result = decode_packet("D2FE28");
        let packet_id = get_id_of_packet(&result);
        assert_eq!(4, packet_id);
    }

    #[test]
    fn test_get_literal() {
        let result = decode_packet("D2FE28");
        let packet_id = get_number_of_literal_packet(&result);
        assert_eq!(2021, packet_id);
    }
    #[test]
    fn test_get_sub_packets() {
        let result = decode_packet("EE00D40C823060");
        let subpackets = get_sub_packets(&result);
        assert_eq!(
            result,
            "11101110000000001101010000001100100000100011000001100000"
        );
        assert_eq!(
            format!("{}{}{}", subpackets[0], subpackets[1], subpackets[2]),
            "010100000011001000001000110000011"
        );
    }
}
