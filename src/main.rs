use koopa::ir::FunctionData;
use koopa::ir::Value;
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::string::ToString;


// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

pub struct InstRet{
    pub reg:String,
    pub valuekind:String, // 新建String表示类型名
}

// 根据内存形式 Koopa IR 生成汇编
trait GenerateAsm {
    fn generate(&self, result: &mut String) {}
    fn generate_inst(&self, result: &mut String, env: &FunctionData,regs:&Vec<&str>,reg_index:&mut usize,inst_reg:&mut HashMap<Value,String>,is_operand:bool) -> InstRet{
        InstRet{reg:"".to_string(),valuekind:"".to_string()}
    } 
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self, result: &mut String) {
        result.push_str("  .text\n");
        result.push_str("  .global main\n");
        // program遍历函数列表
        for &func in self.func_layout() {
            // 访问函数
            self.func(func).generate(result);
        }
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self, result: &mut String) {
        result.push_str(&self.name()[1..]);
        result.push_str(":\n");
        use std::collections::HashMap;

        let regs = vec!["t0","t1","t2","t3","t4","t5","t6", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"];
        let mut reg_index = 0;
        let mut inst_reg:HashMap<Value,String> = HashMap::new();

        // 遍历基本块列表
        for (&bb, node) in self.layout().bbs() {
            // 一些必要的处理
            // ...
            // 遍历指令列表
            for &inst in node.insts().keys() {
                // 一些必要的处理
                // ...
                // 处理指令
                let inst_ret = inst.generate_inst(result, self,&regs,&mut reg_index,&mut inst_reg,false);
                // 一条指令最多只有一个rd，保存对应的寄存器名

            }
        }
    }
}


impl GenerateAsm for koopa::ir::entities::Value {
    fn generate_inst(&self, result: &mut String, env: &FunctionData,regs:&Vec<&str>,reg_index:&mut usize,inst_reg:&mut HashMap<Value,String>,is_operand:bool) -> InstRet{
        use koopa::ir::ValueKind;
        use koopa::ir::BinaryOp::Eq;
        use koopa::ir::BinaryOp::Sub;
        let value_data = env.dfg().value(*self);
        // println!("value_data.kind():{:?}", value_data.kind());

        match value_data.kind() {
            ValueKind::Integer(int) => {
                
                // 处理 integer 指令,
                // 0 单独出现时依然需要li
                // 作为binary 出现时可以用x0代替
                // 因此需要来自binary的信息:is_operand
                // 将所有inst一起处理是否本身不够合理？
                
                let val = int.value();
                if !(val == 0 && is_operand){
                    let reg = regs[*reg_index];
                    *reg_index  += 1;
                    let str = " li ".to_string() + reg + ", " + val.to_string().as_str() + "\n";
                    result.push_str(&str);
                    return InstRet{reg:reg.to_string(),valuekind:"Integer".to_string()};
                } 
                return InstRet{reg:"x0".to_string(),valuekind:"Integer".to_string()};
            }
            ValueKind::Return(ret) => {
                // 处理 ret 指令
                match ret.value() {
                    Some(value) => {
                        value.generate_inst(result, env,regs,reg_index,inst_reg,false);
                    }
                    None => {}
                }
                result.push_str("  ret\n");
                return InstRet{reg:"".to_string(),valuekind:"Return".to_string()};
            }
            ValueKind::Binary(binaryop)=>{
                // lhs 是value
                // 返回reg名
                if is_operand {
                    return InstRet{reg:inst_reg[&self].to_string(),valuekind:"Binary".to_string()} ;
                }
                let lhs_ret = binaryop.lhs().generate_inst(result, env,regs,reg_index,inst_reg,true);
                let rhs_ret= binaryop.rhs().generate_inst(result, env,regs,reg_index,inst_reg,true);
                // 仅当类型时integer时，可以复用reg
                // 默认lhs作为rd，如果lhs是0，那么rhs作为rd
                // 如果都是0，另外选择一个寄存器作为rd
                let mut rd_reg = String::new();
                if &lhs_ret.valuekind =="Interger" &&lhs_ret.reg != "x0" {
                    rd_reg = lhs_ret.reg.clone();
                } else if &rhs_ret.valuekind =="Interger" &&rhs_ret.reg != "x0" {
                    rd_reg = rhs_ret.reg.clone();
                } else {
                    rd_reg = regs[*reg_index].to_string();
                    *reg_index  += 1;
                }

                inst_reg.insert(*self, rd_reg.clone());

                match binaryop.op() {
                    Eq =>{
                        let str = "xor ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        let str = "seqz ".to_string() + rd_reg.as_str() + ", " + rd_reg.as_str() + "\n";
                        result.push_str(&str);
                            
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    },
                    Sub=>{
                        let str = "sub ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    },
                    _ => unreachable!()
                }
                }
            _ => unreachable!(),
        }
            // 其他种类暂时遇不到
    }
}

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    // print!("{}", mode);
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // parse input file
    // println!("{}", ast);

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
