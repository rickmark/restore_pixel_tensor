extern crate argparse;

use argparse::{ArgumentParser, StoreOption};
use xmodem::Xmodem;
use std::fs;
use std::fs::*;
use serialport::{TTYPort, SerialPort};
use regex::Regex;

const XMODEM_BAUD_EBM: u32 = 9600;

const BL_REQUEST_REGEX: Regex = Regex::new(r"(?m)^\((\w+):0x([0-9A-F]+)\)$").unwrap();

struct BootloaderCommand {
    requested_payload: &str,
    base_address: i64
}

fn parse_bootloader_command(command: &str) -> BootloaderCommand {
    let groups = BL_REQUEST_REGEX.captures(command);

    let mut result = BootloaderCommand {
        requested_payload: groups[1],
        base_address: groups[2]
    };

    return result;
}

fn main() {
    let mut parser = ArgumentParser::new();

    let mut firmware: &str = "";
    let mut device: &str = "";
    parser.refer(&mut firmware).add_option(&["-f", "--firmware"], StoreOption<&str>, "The directory of the boot firmware");
    parser.refer(&mut device).add_option(&["-d", "--device"], StoreOption<&str>, "The UART device path");

    let firmwarePath = fs::read_dir(firmware);

    let (mut receivePort, mut sendPort) = serialport::new(device, XMODEM_BAUD_EBM).open().pair();

    let mut bootloaderCommand = parse_bootloader_command(receivePort.read());


    let mut xmodemConnection = Xmodem::new();
    xmodemConnection.send(&port, &firmwareFile);
}


// (bl2:0xXXXXXXXX)incrementer
// EBM request for payload of type bl2
// load address?
//
//