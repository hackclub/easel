//! Deploys assembled programs to the CPU
//!
//! Will be completed once the protocol is solidified.
//!
//! // TODO: write arduino reciever

use std::env;

use clap::Args;
use clio::Input;
use serialport::{SerialPortInfo, SerialPortType};
use thiserror::Error;

const BOARD: &str = "FATEFUL_BOARD";
const PORT: &str = "FATEFUL_PORT";

#[derive(Debug, Args)]
pub struct DeployArgs {
    input: Input,
    #[clap(short = 'P', long)]
    port: Option<String>,
    #[clap(short = 'B', long)]
    board: Option<String>,
    #[clap(short, long)]
    baud: Option<u32>,
}

#[derive(Debug, Error)]
pub enum DeployError {
    #[error("port was unable to be automatically detected")]
    PortNotFound,
    #[error("invalid board provided: {0}")]
    InvalidBoard(String),
    #[error("error opening serial port: {0}")]
    Port(#[from] serialport::Error),
    #[error("error reading input: {0}")]
    Input(std::io::Error),
    #[error("error writing to serial port: {0}")]
    Write(std::io::Error),
}

pub fn deploy(mut args: DeployArgs) -> Result<(), DeployError> {
    let board = args.board.or(env::var(BOARD).ok());
    let port_name = match args.port.or(env::var(PORT).ok()) {
        Some(port) => port,
        None => match board {
            Some(b) => find_board(b)?,
            None => auto_detect().ok_or_else(|| DeployError::PortNotFound)?,
        },
    };

    let baud = args
        .baud
        .or(env::var("FATEFUL_BAUD")
            .ok()
            .and_then(|var| var.parse().ok()))
        .unwrap_or(115200);

    let mut port = serialport::new(port_name, baud).open()?;

    let mut data = [0; 1 << 16];
    args.input
        .lock()
        .read(&mut data)
        .map_err(|err| DeployError::Input(err))?;
    port.write_all(&data)
        .map_err(|err| DeployError::Write(err))?;

    Ok(())
}

fn find_board(board: String) -> Result<String, DeployError> {
    match board.as_str() {
        "uno" | "nano" => {
            Uno::find_port(&serialport::available_ports()?).ok_or(DeployError::PortNotFound)
        }
        "micro" => {
            Micro::find_port(&serialport::available_ports()?).ok_or(DeployError::PortNotFound)
        }
        _ => Err(DeployError::InvalidBoard(board)),
    }
}

fn auto_detect() -> Option<String> {
    let devices = match serialport::available_ports() {
        Ok(devices) => devices,
        Err(_) => return None,
    };

    Uno::find_port(&devices).or_else(|| Micro::find_port(&devices))
}

fn find_vid_pid(devices: &[SerialPortInfo], vid_pid: &[(u16, u16)]) -> Option<String> {
    for device in devices {
        if let SerialPortType::UsbPort(ref usb) = device.port_type {
            if vid_pid.contains(&(usb.vid, usb.pid)) {
                return Some(device.port_name.to_owned());
            }
        }
    }

    None
}

trait Board {
    const DISPLAY_NAME: &'static str;

    fn find_port(devices: &[SerialPortInfo]) -> Option<String>;
}

struct Uno;

impl Board for Uno {
    const DISPLAY_NAME: &'static str = "Arduino Uno";

    fn find_port(devices: &[SerialPortInfo]) -> Option<String> {
        find_vid_pid(
            devices,
            &[
                (0x2341, 0x0043),
                (0x2341, 0x0001),
                (0x2A03, 0x0043),
                (0x2341, 0x0243),
            ],
        )
    }
}

struct Micro;

impl Board for Micro {
    const DISPLAY_NAME: &'static str = "Arduino Micro";

    fn find_port(devices: &[SerialPortInfo]) -> Option<String> {
        find_vid_pid(
            devices,
            &[
                (0x2341, 0x0037),
                (0x2341, 0x8037),
                (0x2A03, 0x0037),
                (0x2A03, 0x8037),
                (0x2341, 0x0237),
                (0x2341, 0x8237),
            ],
        )
    }
}
