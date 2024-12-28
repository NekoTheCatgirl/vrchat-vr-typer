use clap::Parser;

#[derive(Parser)]
#[command(name = "vrchat-vr-typer")]
#[command(author = "Neko The Catgirl")]
#[command(about = "A vrchat OSC utility to make typing in vr easier for touch typers!")]
pub struct Arguments {
    #[arg(short, long, default_value = "127.0.0.1")]
    #[doc = r"The ip to connect to, leave it default if the same device"]
    pub ip: String,
    #[arg(short, long, default_value_t = 9000)]
    #[doc = r"The port to connect to, leave default, unless you use a tunneling service"]
    pub port: u16,
}