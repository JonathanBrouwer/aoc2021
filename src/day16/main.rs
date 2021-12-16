use crate::day16::main::Packet::*;

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let (packet, _) = parse_packet(&input);
    packet.version_sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let (packet, _) = parse_packet(&input);
    packet.calculate()
}

enum Packet {
    LiteralPacket { version: usize, literal: usize },
    Subpackets { version: usize, typ: usize, packets: Vec<Packet> },
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match self {
            Packet::LiteralPacket { version, .. } =>
                {*version}
            Packet::Subpackets { version, packets, .. } =>
                {*version + packets.iter().map(|p| p.version_sum()).sum::<usize>() }
        }
    }

    pub fn calculate(&self) -> usize {
        match self {
            Packet::LiteralPacket { literal, .. } => *literal,
            Packet::Subpackets { typ: 0, packets, .. } => packets.iter().map(|p| p.calculate()).sum(),
            Packet::Subpackets { typ: 1, packets, .. } => packets.iter().map(|p| p.calculate()).product(),
            Packet::Subpackets { typ: 2, packets, .. } => packets.iter().map(|p| p.calculate()).min().unwrap(),
            Packet::Subpackets { typ: 3, packets, .. } => packets.iter().map(|p| p.calculate()).max().unwrap(),
            Packet::Subpackets { typ: 5, packets, .. } => (packets[0].calculate() > packets[1].calculate()) as usize,
            Packet::Subpackets { typ: 6, packets, .. } => (packets[0].calculate() < packets[1].calculate()) as usize,
            Packet::Subpackets { typ: 7, packets, .. } => (packets[0].calculate() == packets[1].calculate()) as usize,
            _ => unreachable!()
        }
    }
}

fn parse_packet(packet: &[bool]) -> (Packet, usize) {
    let version = bits_to_num(&packet[0..3]);
    let typ = bits_to_num(&packet[3..6]);

    match typ {
        4 => {
            let mut start = 6usize;
            let mut literal = 0usize;
            loop {
                literal = (literal << 4) | bits_to_num(&packet[start + 1..start + 5]);
                if !packet[start] { break; }
                start += 5;
            }
            start += 5;
            (LiteralPacket { version, literal }, start)
        }
        _ => {
            let mut packets = vec![];
            let mut rest = 7;
            if !packet[6] {
                //Length type 0
                let length = bits_to_num(&packet[7..7+15]);
                rest += 15;
                let end = rest + length;
                while rest < end {
                    let (subpacket, subrest) = parse_packet(&packet[rest..]);
                    packets.push(subpacket);
                    rest += subrest;
                }

            } else {
                //Length type 1
                let length = bits_to_num(&packet[7..7+11]);
                rest += 11;
                for _ in 0..length {
                    let (subpacket, subrest) = parse_packet(&packet[rest..]);
                    packets.push(subpacket);
                    rest += subrest;
                }
            }
            (Subpackets {version, typ, packets }, rest)
        }
    }
}

fn parse_input(inp: &str) -> Vec<bool> {
    to_hex(inp)
}

fn bits_to_num(inp: &[bool]) -> usize {
    inp.into_iter().fold(0usize, |c, n| (c << 1) | *n as usize)
}

fn to_hex(s: &str) -> Vec<bool> {
    s.chars().map(|c| to_hex_char(c)).flatten().collect()
}

fn to_hex_char(c: char) -> [bool; 4] {
    match c {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1("D2FE28"), 6);
        assert_eq!(part1("38006F45291200"), 9);
        assert_eq!(part1("EE00D40C823060"), 14);
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(943, result);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(167737115857, result);
    }
}



