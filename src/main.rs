use koopa::ir::FunctionData;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::string::ToString;
// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
// lalrpop_mod!(sysy);
lalrpop_mod!(sysy);
// 根据内存形式 Koopa IR 生成汇编
trait GenerateAsm {
    fn generate(&self, result: &mut String) {}
    fn generate_stmt(&self, result: &mut String, env: &FunctionData) {}
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self, result: &mut String) {
        result.push_str("  .text\n");
        result.push_str("  .global main\n");
        for &func in self.func_layout() {
            self.func(func).generate(result);
        }
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self, result: &mut String) {
        result.push_str(&self.name()[1..]);
        result.push_str(":\n");

        // 遍历基本块列表
        for (&bb, node) in self.layout().bbs() {
            // 一些必要的处理
            // ...
            // 遍历指令列表
            for &inst in node.insts().keys() {
                // 一些必要的处理
                // ...
                // 处理指令
                inst.generate_stmt(result, self);
            }
        }
    }
}

impl GenerateAsm for koopa::ir::entities::Value {
    fn generate_stmt(&self, result: &mut String, env: &FunctionData) {
        // 访问指令
        use koopa::ir::ValueKind;
        let value_data = env.dfg().value(*self);

        match value_data.kind() {
            ValueKind::Integer(int) => {
                // // 处理 integer 指令
                // // ...
                result.push_str("  li a0, ");
                result.push_str(int.value().to_string().as_str());
                result.push_str("\n");
            }
            ValueKind::Return(ret) => {
                // 处理 ret 指令
                match ret.value() {
                    Some(value) => {
                        value.generate_stmt(result, env);
                    }
                    None => {}
                }
                result.push_str("  ret\n");
            }
            // 其他种类暂时遇不到
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    print!("{}", mode);
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // parse input file
    println!("{}", ast);

    let driver = koopa::front::Driver::from(ast.to_string());
    let program = driver.generate_program().unwrap();
    // 数据和layout是分离表示的
    let mut program_str = String::new();
    program.generate(&mut program_str);
    let write_file = File::create(output).unwrap();
    let mut writer = BufWriter::new(&write_file);

    match mode.as_str() {
        "-koopa" => {
            // 文本形式IR，文件output
            // write!(&mut writer, "{}", ast)
            Ok(())
        }
        "-riscv" => {
            // RISC-V汇编，文件output
            write!(&mut writer, "{}", program_str)
        }
        _ => unreachable!(),
    }
}
