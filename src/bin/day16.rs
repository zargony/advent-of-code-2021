use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use thiserror::Error;

/// Error variants
#[derive(Debug, Error)]
enum Error {
    #[error("Out of input data")]
    OutOfData,
    #[error("Invalid hex digit `{0}`")]
    InvalidHexDigit(char),
    #[error("Invalid packet type id {0}")]
    InvalidType(u64),
}

/// Parse bits from hexadecimal digits
#[allow(clippy::needless_lifetimes)]
fn hex2bits<'a>(s: &'a str) -> impl Iterator<Item = Result<bool, Error>> + 'a {
    s.chars().flat_map(|ch| {
        let n = ch.to_digit(16);
        [
            n.map(|n| n & 0b1000 > 0).ok_or(Error::InvalidHexDigit(ch)),
            n.map(|n| n & 0b0100 > 0).ok_or(Error::InvalidHexDigit(ch)),
            n.map(|n| n & 0b0010 > 0).ok_or(Error::InvalidHexDigit(ch)),
            n.map(|n| n & 0b0001 > 0).ok_or(Error::InvalidHexDigit(ch)),
        ]
    })
}

/// Parse integer number from bitstream
fn parse_number(
    bits: &mut impl Iterator<Item = Result<bool, Error>>,
    n: usize,
) -> Result<u64, Error> {
    let mut res = 0;
    for _ in 0..n {
        let bit = match bits.next().ok_or(Error::OutOfData)?? {
            false => 0,
            true => 1,
        };
        res = (res << 1) | bit
    }
    Ok(res)
}

/// Parse grouped integer number from bitstream
fn parse_grouped_number(
    bits: &mut impl Iterator<Item = Result<bool, Error>>,
) -> Result<u64, Error> {
    let mut res = 0;
    loop {
        let more = bits.next().ok_or(Error::OutOfData)??;
        res = (res << 4) | parse_number(bits, 4)?;
        if !more {
            return Ok(res);
        }
    }
}

/// Parse list of packets from bitstream
fn parse_packet_list(
    bits: &mut impl Iterator<Item = Result<bool, Error>>,
) -> Result<Vec<Packet>, Error> {
    if !bits.next().ok_or(Error::OutOfData)?? {
        let len = parse_number(bits, 15)?;
        let mut bits = bits.take(len as usize).collect_vec().into_iter();
        let mut packets = Vec::new();
        loop {
            match Packet::parse(&mut bits) {
                Ok(packet) => packets.push(packet),
                Err(Error::OutOfData) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(packets)
    } else {
        let count = parse_number(bits, 11)?;
        let packets = (0..count).map(|_| Packet::parse(bits)).try_collect()?;
        Ok(packets)
    }
}

/// Packet operators
#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Operator {
    /// Parse packet operator from bitstream
    fn parse(bits: &mut impl Iterator<Item = Result<bool, Error>>) -> Result<Self, Error> {
        Ok(match parse_number(bits, 3)? {
            0 => Self::Sum(parse_packet_list(bits)?),
            1 => Self::Product(parse_packet_list(bits)?),
            2 => Self::Minimum(parse_packet_list(bits)?),
            3 => Self::Maximum(parse_packet_list(bits)?),
            4 => Self::Literal(parse_grouped_number(bits)?),
            5 => Self::GreaterThan(parse_packet_list(bits)?),
            6 => Self::LessThan(parse_packet_list(bits)?),
            7 => Self::EqualTo(parse_packet_list(bits)?),
            id => return Err(Error::InvalidType(id)),
        })
    }
}

/// Packet
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u64,
    operator: Operator,
}

impl Packet {
    /// Parse packet from bitstream
    fn parse(bits: &mut impl Iterator<Item = Result<bool, Error>>) -> Result<Self, Error> {
        Ok(Self {
            version: parse_number(bits, 3)?,
            operator: Operator::parse(bits)?,
        })
    }

    /// Sum of version numbers
    fn version_sum(&self) -> u64 {
        self.version
            + self
                .subpackets()
                .iter()
                .map(|p| p.version_sum())
                .sum::<u64>()
    }

    /// Evaluate the packet
    fn eval(&self) -> u64 {
        match self.operator {
            Operator::Sum(ref packets) => packets.iter().map(|p| p.eval()).sum(),
            Operator::Product(ref packets) => packets.iter().map(|p| p.eval()).product(),
            Operator::Minimum(ref packets) => packets.iter().map(|p| p.eval()).min().unwrap(),
            Operator::Maximum(ref packets) => packets.iter().map(|p| p.eval()).max().unwrap(),
            Operator::Literal(value) => value,
            Operator::GreaterThan(ref packets) => (packets[0].eval() > packets[1].eval())
                .then(|| 1)
                .unwrap_or(0),
            Operator::LessThan(ref packets) => (packets[0].eval() < packets[1].eval())
                .then(|| 1)
                .unwrap_or(0),
            Operator::EqualTo(ref packets) => (packets[0].eval() == packets[1].eval())
                .then(|| 1)
                .unwrap_or(0),
        }
    }

    /// Subpackets
    fn subpackets(&self) -> &[Packet] {
        match self.operator {
            Operator::Sum(ref packets) => packets,
            Operator::Product(ref packets) => packets,
            Operator::Minimum(ref packets) => packets,
            Operator::Maximum(ref packets) => packets,
            Operator::Literal(_) => &[],
            Operator::GreaterThan(ref packets) => packets,
            Operator::LessThan(ref packets) => packets,
            Operator::EqualTo(ref packets) => packets,
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let line = Input::day(16)?.line()?;
    let mut bits = hex2bits(&line);
    let packet = Packet::parse(&mut bits).unwrap();

    println!("Version sum: {}", packet.version_sum());

    println!("Result: {}", packet.eval());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits2string(it: impl IntoIterator<Item = Result<bool, Error>>) -> String {
        it.into_iter()
            .map(|bit| if bit.unwrap() { '1' } else { '0' })
            .collect()
    }

    #[test]
    fn hex_bits() {
        let bits = hex2bits("D2FE28");
        assert_eq!(bits2string(bits), "110100101111111000101000");
    }

    #[test]
    fn part_1a() {
        let mut bits = hex2bits("D2FE28");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.operator, Operator::Literal(2021));
    }

    #[test]
    fn part_1b() {
        let mut bits = hex2bits("38006F45291200");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version, 1);
        match packet.operator {
            Operator::LessThan(ref packets) => {
                assert_eq!(packets.len(), 2);
                assert_eq!(packets[0].version, 6);
                assert_eq!(packets[0].operator, Operator::Literal(10));
                assert_eq!(packets[1].version, 2);
                assert_eq!(packets[1].operator, Operator::Literal(20));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn part_1c() {
        let mut bits = hex2bits("EE00D40C823060");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version, 7);
        match packet.operator {
            Operator::Maximum(ref packets) => {
                assert_eq!(packets.len(), 3);
                assert_eq!(packets[0].version, 2);
                assert_eq!(packets[0].operator, Operator::Literal(1));
                assert_eq!(packets[1].version, 4);
                assert_eq!(packets[1].operator, Operator::Literal(2));
                assert_eq!(packets[2].version, 1);
                assert_eq!(packets[2].operator, Operator::Literal(3));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn part_1d() {
        let mut bits = hex2bits("8A004A801A8002F478");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version_sum(), 16);

        let mut bits = hex2bits("620080001611562C8802118E34");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version_sum(), 12);

        let mut bits = hex2bits("C0015000016115A2E0802F182340");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version_sum(), 23);

        let mut bits = hex2bits("A0016C880162017C3686B18A3D4780");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn part_2() {
        let mut bits = hex2bits("C200B40A82");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 3);

        let mut bits = hex2bits("04005AC33890");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 54);

        let mut bits = hex2bits("880086C3E88112");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 7);

        let mut bits = hex2bits("CE00C43D881120");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 9);

        let mut bits = hex2bits("D8005AC2A8F0");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 1);

        let mut bits = hex2bits("F600BC2D8F");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 0);

        let mut bits = hex2bits("9C005AC2F8F0");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 0);

        let mut bits = hex2bits("9C0141080250320F1802104A08");
        let packet = Packet::parse(&mut bits).unwrap();
        assert_eq!(packet.eval(), 1);
    }
}
