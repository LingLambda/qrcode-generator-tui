use crate::app::SaveOption;
use crate::generator::generator_qrcode;
use chardetng::EncodingDetector;
use clap::{CommandFactory, Parser};
use encoding_rs::Encoding;
use std::{borrow::Cow, fs};

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
        generator_qrcode(text, SaveOption::SaveAndOpen);
        std::process::exit(0);
    } else if let Some(path) = args.path {
        let vec = fs::read(path).expect("读取文件错误");
        let name = guess_encoding(&vec);
        println!("检测到文件编码格式:{}", name);
        let cow: Cow<str>;
        let had_errors;
        if let Some(encoding) = Encoding::for_label(name.as_bytes()) {
            (cow, _, had_errors) = encoding.decode(&vec)
        } else {
            panic!("不支持的文件编码格式");
        }
        if had_errors {
            panic!("文件编码错误");
        }
        generator_qrcode(cow.to_string(), SaveOption::SaveAndOpen);
        std::process::exit(0);
    }
}

fn guess_encoding(vec: &[u8]) -> String {
    let mut detector = EncodingDetector::new();
    detector.feed(vec, false);
    let (encoding, guess) = detector.guess_assess(None, true);
    if !guess {
        panic!("无法推断文件编码格式");
    }
    encoding.name().to_string()
}
