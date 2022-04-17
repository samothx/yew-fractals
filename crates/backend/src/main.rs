use actix_web::{App, HttpServer, Responder, web};
use actix_files as fs;

use structopt::StructOpt;
use std::path::PathBuf;
use mod_logger::{Logger, Level, LogDestination};
use std::io::BufWriter;
use std::fs::File;
use log::{error, info};

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = env ! ("CARGO_PKG_NAME"), author, about)]
struct Options {
    #[structopt(
    short,
    long,
    value_name = "ROOT",
    parse(from_os_str),
    default_value = ".",
    help = "Path to server root directory"
    )]
    root: PathBuf,
    #[structopt(
    short,
    long,
    value_name = "LOG_LEVEL",
    default_value = "info",
    help = "log level"
    )]
    log_level: Level,
    #[structopt(
    long,
    value_name = "LOG_FILE",
    parse(from_os_str),
    help = "Path to log file"
    )]
    log_file: Option<PathBuf>,
    #[structopt(
    short,
    long,
    value_name = "PORT",
    help = "The server port to listen on",
    default_value = "8080",
    )]
    port: u16,
    #[structopt(
    short,
    long,
    value_name = "IP",
    help = "The server IP address to listen on",
    default_value = "127.0.0.1",
    )]
    ip: String,
    #[structopt(
    short,
    long,
    value_name = "BASE",
    help = "The server's base path",
    default_value = "",
    )]
    base: String,

}


async fn api_echo() -> impl Responder {
    "Hello api"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = Options::from_args();
    Logger::set_default_level(options.log_level);
    if let Some(log_file) = options.log_file.as_ref() {
        match File::create(log_file) {
            Ok(file) => {
                Logger::set_log_dest(&LogDestination::Stream, Some(BufWriter::new(file)))
                    .expect(format!("Failed to log to {:?}", log_file).as_str());
            }
            Err(err) => { error!("Could not open logfile {:?}, error: {:?}", log_file, err); }
        };
    }

    info!("starting http server on {}:{}{}",
        options.ip, options.port,
        &if options.base.is_empty() {
            "".to_string()
        } else {
            format!(", with base path: {}", options.base)
        });


        HttpServer::new(move ||
            App::new()
                .service(web::scope(format!("{}/api", options.base).as_str())
                    .route("echo", web::get().to(api_echo)))
                .service(fs::Files::new(
                    format!("{}/", options.base).as_str(),
                    &options.root )) )
            .bind(("127.0.0.1", options.port))?
            .run()
            .await

    /*
    HttpServer::new(move || {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope(format!("{}/api", options.base).as_str())
                // ...so this handles requests for `GET base/app/index.html`
                .route("/index.html", web::get().to(index)),
        ).
    })
        .bind((options.ip, options.port))?
        .run()
        .await

     */
}
