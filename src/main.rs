use anyhow::Result;
use std::env;

use std::process::Command;
pub fn add_start(cmdline: &str) {
    let _ = Command::new("sh").args(["-c", &cmdline]).output();
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("输入参数少，应该分别为: 文件名，字段名");
        return Ok(());
    }
    let file_name = args[1].clone();
    let field_name = args[2].clone();
    let exec = "sed -i '/#\\[sea_orm(column_name = \"";
    let exec = format!("{}{}", exec, field_name);
    let exec = format!("{}\"", exec);
    let exec = format!("{})\\]/a #\\[serde(rename = \"", exec);
    let exec = format!("{}{}", exec, field_name);
    let exec = format!("{}\"", exec);
    let exec = format!("{})\\]' ", exec);
    let exec = format!("{}{}", exec, file_name);
    println!("{}", exec);
    add_start(&exec);
    Ok(())
}
