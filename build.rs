use std::path::Path;
use std::process::Command;

fn main() {
    let output_path = Path::new("src/services.list");

    println!("cargo:rerun-if-changed=generate_services.py");
    println!("cargo:rerun-if-changed=raw_services.list");
    println!("cargo:rerun-if-env-changed=PYTHONPATH");

    let status = Command::new("python3").arg("generate_services.py").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ services.list 生成成功 -> {:?}", output_path);
        }
        Ok(exit_status) => {
            panic!("❌ Python 脚本执行失败，退出码: {}", exit_status);
        }
        Err(e) => {
            panic!("❌ 执行 Python 脚本时出错: {}", e);
        }
    }
}
