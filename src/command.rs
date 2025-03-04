use crate::app::SaveOption;
use crate::generator::generator_qrcode;
use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(index = 1, exclusive = true, help = "直接传递文本")]
    text: Option<String>,
    #[arg(short, long, exclusive = true, help = "启动ui编辑器")]
    ui: bool,
    #[arg(short, long, exclusive = true, help = "读取指定路径文本文件")]
    path: Option<std::path::PathBuf>,
}

pub fn command() {
    let args = Args::parse();

    // println!("{:?}", args);
    if args.text.is_none() && !args.ui && args.path.is_none() {
        Args::command().print_help().expect("打印帮助信息失败");
        std::process::exit(0);
    }

    if let Some(text) = args.text {
        generator_qrcode(text,SaveOption::SaveAndOpen);
        std::process::exit(0);
    } else if args.ui {
    } else {
        panic!("未知错误");
    }
}
