use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Opt {
    #[structopt(about = "Set a key-value pair")]
    Set {
        #[structopt(help = "Key")]
        key: String,
        #[structopt(help = "Value")]
        value: String,
        #[structopt(short, long)]
        ex: Option<i32>,
        #[structopt(short, long)]
        transaction_id: Option<i32>,
    },
    Del {
        #[structopt(help = "Key")]
        key: String,
        #[structopt(short, long)]
        transaction_id: Option<i32>,
    },
    #[structopt(about = "Get the value for a key")]
    Get {
        #[structopt(help = "Key")]
        key: String,
    },
    Ping {
        #[structopt(help = "Value")]
        value: Option<String>,
    },
    Subscribe {
        #[structopt(help = "Channel")]
        channel: String,
        #[structopt(short, long)]
        and: Option<Vec<String>>,
    },
    Publish {
        #[structopt(help = "Channel")]
        channel: String,
        #[structopt(help = "Value")]
        value: String,
    },
    Multi {},
    Exec {
        #[structopt(help = "Transaction id")]
        transaction_id: i32,
    },
    Watch {
        #[structopt(help = "Key")]
        key: String,
        #[structopt(help = "Transaction id")]
        transaction_id: i32,
    },
}

#[derive(StructOpt)]
#[structopt(about = "The request cli")]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Opt,

    #[structopt(short, long, default_value = "8000")]
    pub port: String,
}
#[derive(StructOpt)]
pub enum ServerType {
    Master {},
    Slave { port: String },
    Proxy {},
}

#[derive(StructOpt)]
pub struct ServerArgs {
    #[structopt(subcommand)]
    pub server_type: ServerType,

    #[structopt(short, long, default_value = "8000")]
    pub port: String,
}
