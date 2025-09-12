/// 程序运行的参数
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// config file path
    #[arg(short, long)]
    pub config_path: String,
}
