use crate::app::SaveOption;
use image::Luma;
use qrcode::types::QrError;
use qrcode::QrCode;
use std::process::Command;

pub fn generator_qrcode(text: String, option: SaveOption) {
    let mut open_image = true;
    match option {
        SaveOption::SaveNotOpen => {
            open_image = false;
        }
        SaveOption::DontSave => {
            return;
        }
        _ => {}
    }
    let code_result = QrCode::new(&text);
    match code_result {
        Ok(qrcode) => {
            generator_image(qrcode, None, open_image);
        }
        Err(QrError::DataTooLong) => {
            let mut index = 0;
            for str in slice_to_vec(text) {
                index += 1;
                let code = QrCode::new(&str);
                generator_image(code.unwrap(), Some(index), open_image);
            }
        }
        _ => {}
    }
}

fn slice_to_vec(string: String) -> Vec<String> {
    string
        .chars()
        .collect::<Vec<char>>()
        .chunks(1000)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn generator_image(code: QrCode, index: Option<u8>, open_img: bool) {
    let image = code.render::<Luma<u8>>().min_dimensions(300, 300).build();
    let mut file_name = String::from("qrcode");
    if let Some(i) = index {
        file_name = format!("{}({})", file_name, i);
    }
    file_name.push_str(".png");
    // 将二维码保存为图像文件
    image.save(&file_name).expect("保存文件失败");
    let path_name = format!("./{}", file_name);
    println!("二维码已生成并保存在{}", path_name);
    if open_img {
        windows_open_file(path_name.as_str())
    }
}

fn windows_open_file(file_path: &str) {
    let status = Command::new("cmd")
        .arg("/C") // 通过 cmd 执行
        .arg("start") // 调用 start 命令
        .arg(file_path) // 传入文件路径
        .status() // 获取执行状态
        .expect("打开文件失败:");

    if status.success() {
        println!("文件成功打开: {}", file_path);
    } else {
        eprintln!("打开文件失败: {}", file_path);
    }
}
