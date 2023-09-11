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

#[derive(PartialEq, Eq)]
pub enum ParentType {
    Binary,
    Return,
    None
}

// 根据内存形式 Koopa IR 生成汇编
trait GenerateAsm {
    fn generate(&self, result: &mut String) {}
    fn generate_inst(&self, result: &mut String, env: &FunctionData,regs:&Vec<&str>,reg_index:&mut usize,inst_reg:&mut HashMap<Value,String>,parent_type:ParentType) -> InstRet{
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
            // 遍历指令列表
            for &inst in node.insts().keys() {
                // 处理指令
                let inst_ret = inst.generate_inst(result, self,&regs,&mut reg_index,&mut inst_reg,ParentType::None);
            }
        }
    }
}

impl GenerateAsm for koopa::ir::entities::Value {
    fn generate_inst(&self, result: &mut String, env: &FunctionData,regs:&Vec<&str>,reg_index:&mut usize,inst_reg:&mut HashMap<Value,String>,parent_type:ParentType) -> InstRet{
        use koopa::ir::ValueKind;
        use koopa::ir::BinaryOp::*;
        let value_data = env.dfg().value(*self);

        match value_data.kind() {
            ValueKind::Integer(int) => {

                // 1. 父类型是二元运算，
                //     1.1 val非0，rd为临时寄存器，添加指令li reg, val
                //     1.2 val为0，rd为x0，不添加指令
                // 2. 父类型时return，rd为a0/a1，添加指令li rd，val
                let val = int.value();
                let mut rd = "";
                match parent_type {
                    ParentType::Binary => {
                        if val != 0 {
                            rd = regs[*reg_index];
                            *reg_index  += 1;
                            let str = "  li    ".to_string() + rd+ ", " + val.to_string().as_str() + "\n";
                    result.push_str(&str);
                        } else {
                            rd = "x0";
                        }
                    },
                    ParentType::Return => {
                        rd = "a0";
                        let str = "  li    ".to_string() + rd+ ", " + val.to_string().as_str() + "\n";
                        result.push_str(&str);
                    },
                    _ => {}
                };

                    return InstRet{reg:rd.to_string(),valuekind:"Integer".to_string()};
                } ,
            ValueKind::Return(ret) => {
                match ret.value() {
                    Some(value) => {
                        let inst_ret =value.generate_inst(result, env,regs,reg_index,inst_reg,ParentType::Return);
                        if inst_ret.valuekind == "Binary" {
                            let str = "  mv    a0, ".to_string() + inst_ret.reg.as_str() + "\n";
                            result.push_str(&str);
                        }
                    }
                    None => {}
                }
                result.push_str("  ret\n");
                return InstRet{reg:"".to_string(),valuekind:"Return".to_string()};
            }
            ValueKind::Binary(binaryop)=>{
                // 父类型时表达式时不添加指令
                if parent_type != ParentType::None {
                    return InstRet{reg:inst_reg[&self].to_string(),valuekind:"Binary".to_string()} ;
                }
                let lhs_ret = binaryop.lhs().generate_inst(result, env,regs,reg_index,inst_reg,ParentType::Binary);
                let rhs_ret= binaryop.rhs().generate_inst(result, env,regs,reg_index,inst_reg,ParentType::Binary);

                // 当类型为integer且非零时，可以复用reg
                let mut rd_reg = String::new();
                if &lhs_ret.valuekind =="Integer" &&lhs_ret.reg != "x0" {
                    rd_reg = lhs_ret.reg.clone();
                } else if &rhs_ret.valuekind =="Integer" &&rhs_ret.reg != "x0" {
                    rd_reg = rhs_ret.reg.clone();
                } else {
                    rd_reg = regs[*reg_index].to_string();
                    *reg_index  += 1;
                }

                inst_reg.insert(*self, rd_reg.clone());

                match binaryop.op() {
                    Eq =>{
                        let str = "  xor   ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        let str = "  seqz  ".to_string() + rd_reg.as_str() + ", " + rd_reg.as_str() + "\n";
                        result.push_str(&str);
                            
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    },
                    Sub=>{
                        let str = "  sub   ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    },
                    Mul=>{
                        let str = "  mul   ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    },
                    Add=>{
                        let str = "  add   ".to_string() + rd_reg.as_str() + ", " + lhs_ret.reg.as_str() + ", " + rhs_ret.reg.as_str() + "\n";
                        result.push_str(&str);
                        InstRet{reg:rd_reg,valuekind:"Binary".to_string()}
                    }
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
            println!("{}",program_str);
            write!(&mut writer, "{}", program_str)
        }
        _ => unreachable!(),
    }
}
