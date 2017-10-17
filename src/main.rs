extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate clap;

mod types;
mod proto;
mod client;
mod server;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("igel_chat")
                          .version("0.1.0")
                          .author("Stefan Schindler <dns2utf8@estada.ch>")
                          .about("A rusty chat tool for the modern human")
                          .subcommand(SubCommand::with_name("client")
                                      .about("starting igel in client mode")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                                      .arg_from_usage("-r, --remote 'Use the well known global server instaed of [::1]'")
                          .subcommand(SubCommand::with_name("server")
                                      .about("starting igel in server mode")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                                      .arg_from_usage("-l, --listen 'listen to [::] instead of [::1]'")
                          .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("client") {
        println!("{:?}", matches);
        client::main(matches);
        return;
    }
    if let Some(matches) = matches.subcommand_matches("server") {
        println!("{:?}", matches);
        server::main(matches);
        return;
    }
    
    println!("you must use the server or the client mode!\n{}", matches.usage());
}
