#[macro_use]
extern crate clap;
extern crate modbus;
extern crate time;
extern crate serial;

use modbus::{Client, Coil};
use modbus::tcp;
use clap::App;

use std::io;
use std::time::Duration;

use serial::prelude::*;

fn main() {
	
  let matches = App::new("client")
      .author("Klaus Landsdorf <klaus.landsdorf@bianco-royal.de>")
      .version(&crate_version!()[..])
      .about("Modbus Tcp client")
      .args_from_usage("<SERVER> 'The IP address or hostname of the server'
                      \
                        <COM> 'The COM port to send one value by serial'
                      \
                        --read-coils=[ADDR] [QUANTITY] 'Read QUANTITY coils from ADDR'
                      \
                        --read-discrete-inputs=[ADDR] [QUANTITY] 'Read QUANTITY inputs from \
                        ADDR'
                      --write-single-coil=[ADDR] [On,Off] \
                        'Write the coil value (On or Off) to ADDR'
                      \
                        --write-multiple-coils=[ADDR] [On,Off..] 'Write multiple coil values \
                        (On or Off) to ADDR (use \"..\" without spaces to group them e.g. \
                        \"On, Off, On, Off\")'
                      \
                        --read-input-registers=[ADDR], [QUANTITY] 'Read QUANTITY input \
                        registersfrom ADDR'
                      \
                        --read-holding-registers=[ADDR], [QUANTITY] 'Read QUANTITY holding \
                        registers from ADDR'
                      \
                        --write-single-register=[ADDR] [VALUE] 'Write VALUE to register ADDR'
                      \
                        --write-multiple-registers=[ADDR] [V1,V2...] 'Write multiple register \
                        values to ADDR (use \"..\" to group them e.g. \"23, 24, 25\")'")
      .get_matches();

  let mut client = tcp::Transport::new(matches.value_of("SERVER").unwrap()).unwrap();

  if let Some(args) = matches.values_of("read-coils") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let qtty: u16 = args[1].parse().expect(matches.usage());
     
      println!("{:?}", client.read_coils(addr, qtty).expect("IO Error"));
  }
  else if let Some(args) = matches.values_of("read-discrete-inputs") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let qtty: u16 = args[1].parse().expect(matches.usage());
      
      println!("{:?}", client.read_discrete_inputs(addr, qtty).expect("IO Error"));
  }
  else if let Some(args) = matches.values_of("write-single-coil") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let value: Coil = args[1].parse().expect(matches.usage());
      
      client.write_single_coil(addr, value).expect("IO Error");
  }
  else if let Some(args) = matches.values_of("write-multiple-coils") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let values: Vec<Coil> = args[1]
          .split(',')
          .map(|s| s.trim().parse().expect(matches.usage()))
          .collect();
      
      client.write_multiple_coils(addr, &values).expect("IO Error");
  }
  else if let Some(args) = matches.values_of("read-holding-registers") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let qtty: u16 = args[1].parse().expect(matches.usage());
      
      let results = client.read_holding_registers(addr, qtty).expect("IO Error");

    if qtty == 1 {
      println!("{:?}, {:?}", time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(), results[0]);

      /*
      let mut port = serial::open(matches.value_of("COM").unwrap()).unwrap();
      let result = results[0];
      interact(&mut port, &mut [result as u8, (result >> 8) as u8]).unwrap();
      */

    } else {
      println!("{:?}, {:?}", time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(), results);
    }        
  }
  else if let Some(args) = matches.values_of("write-single-register") {
      
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let value: u16 = args[1].parse().expect(matches.usage());
      
      client.write_single_register(addr, value).expect("IO Error");
  }
  else if let Some(args) = matches.values_of("write-multiple-registers") {
  
      let args: Vec<&str> = args.collect();
      let addr: u16 = args[0].parse().expect(matches.usage());
      let values: Vec<u16> = args[1]
          .split(',')
          .map(|s| s.trim().parse().expect(matches.usage()))
          .collect();
      
      client.write_multiple_registers(addr, &values).expect("IO Error");
  }
}

fn interact<T: SerialPort>(port: &mut T, values: &mut [u8]) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(5000)));

    let mut buf: Vec<u8> = (0..255).collect();

    try!(port.write(&values));
    try!(port.read(&mut buf[..]));

    Ok(())
}
