#[macro_use]
extern crate clap;
extern crate modbus;
extern crate time;
extern crate serial;

use modbus::{Client, Coil};
use modbus::tcp;
use clap::App;

fn main() {
	
  let matches = App::new("client")
      .author("Klaus Landsdorf <klaus.landsdorf@bianco-royal.de>")
      .version(&crate_version!()[..])
      .about("Modbus Tcp client")
      .args_from_usage("<SERVER> 'The IP address or hostname of the server'
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

  let server_addr = matches.value_of("SERVER").unwrap();
  
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
      println!("{:?};{:?};{:?}", time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(), results[0], server_addr);
    } else {
      print!("{:?};", time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap());

      for value in results {
        print!("{:?};", value);
      }

      println!("{:?}", server_addr);
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
