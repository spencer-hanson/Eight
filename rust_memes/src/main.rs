mod console;
mod eight;
mod networking;
mod testing;
mod webui;

use eight::common::logging::EightLogger;
use eight::common::parsing::ast::EightAST;
use log;
use log::{info, LevelFilter};
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CommandLineArgs {
    #[structopt(short = "s", long = "slave")]
    is_slave: bool,

    #[structopt(short = "m", long = "master")]
    is_master: bool,

    #[structopt(short = "f", long = "file")]
    filename: String,

    #[structopt(name = "namespace")]
    namespace: String,
}

static LOGGER: EightLogger = EightLogger;

fn init_logger() {
    match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace)) {
        Err(x) => {
            panic!("Unable to set up logger! Error: '{}'", x)
        }
        _ => {}
    }
}

fn main() {
    init_logger();

    let opts = CommandLineArgs::from_args();
    info!("Starting under namespace: '{}'", opts.namespace);

    // testing::test_multiborrow();
    // testing::test_dropborrow();
    // exit(0);

    if opts.is_master {
        let _web_thread = webui::index::start_webserver();
        console::commands::wait_for_commands();
        _web_thread.join().unwrap();
    } else if opts.is_slave {
        networking::search::search_for_master(opts.namespace);
    } else if !opts.filename.is_empty() {
        let error_msg = format!("Error reading file '{}'!", opts.filename);

        let file_data = std::fs::read_to_string(opts.filename).expect(error_msg.as_str());

        let exprs = EightAST::new(eight::start_parse(file_data)).optimize_ast();

        eight::runners::local::run(exprs);
    }
}
