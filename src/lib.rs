pub mod ast {
    use std::fmt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static PC: AtomicUsize = AtomicUsize::new(0);
    #[derive(Debug)]
    pub struct CompUnit {
        pub func_def: FuncDef,
    }

    impl fmt::Display for CompUnit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.func_def)
        }
    }
    #[derive(Debug)]
    pub struct FuncDef {
        pub func_type: FuncType,
        pub ident: String,
        pub block: Block,
    }

    impl fmt::Display for FuncDef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.func_type {
                FuncType::Int => {
                    write!(f, "fun @{}(): i32 {{\n{}}}", self.ident, self.block)
                }
                FuncType::String => {
                    write!(f, "fun @{}(): String {{\n{}}}", self.ident, self.block)
                }
            }
        }
    }
    #[derive(Debug)]
    pub enum FuncType {
        Int,
        String,
    }
    #[derive(Debug)]
    pub enum UnaryOp {
        Pos,
        Neg,
        Not,
    }

    trait Pc {
        fn add_pc(&self) -> usize;
        fn previous_pc(&self) -> usize;
        fn load_pc(&self) -> usize;
    }

    #[derive(Debug)]
    pub enum UnaryExp {
        Number(i32),
        Op(UnaryOp, Box<UnaryExp>),
    }
    impl Pc for UnaryExp {
        fn add_pc(&self) -> usize {
            PC.fetch_add(1, Ordering::Relaxed)
        }

        fn previous_pc(&self) -> usize {
            PC.load(Ordering::Relaxed) - 1
        }

        fn load_pc(&self) -> usize {
            PC.load(Ordering::Relaxed)
        }
    }

    // 使用rc（多线程不安全）给expr编号
    impl fmt::Display for UnaryExp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UnaryExp::Number(n) => {
                    // println!("format Number({})", n);
                    write!(f, "{}", n) //
                },
                UnaryExp::Op(op, exp) => {
                    
                    // println!("format {:?},",self);
                    
                    let mut prev_exp = String::new();
                    let mut prev_stmt = String::new();
                    match &**exp { //当前op的exp
                        UnaryExp::Number(num) =>{ 
                            // println!("match number");   
                            prev_exp = num.to_string()}, // 当前op的exp，如果是
                        UnaryExp::Op(unaryop, expression) => {
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt = format!("{}",exp);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp = format!("%{}", exp.previous_pc().to_string())
                        }
                    }

                    match op {
                        UnaryOp::Pos => {
                            // print!("<Pos {} Pos>", prev_stmt);
                            write!(f, "{}", prev_stmt)},
                        UnaryOp::Neg => {
                            // print!("<Neg {}%{} = sub 0, {} Neg>\n",prev_stmt, self.load_pc(), prev_exp);
                            write!(f, "{}  %{} = sub 0, {}\n",prev_stmt, self.add_pc(), prev_exp)},
                        UnaryOp::Not => {
                            // print!("<Not {}%{} = eq {}, 0 Not>\n",prev_stmt, self.load_pc(), prev_exp);
                            write!(f, "{}  %{} = eq {}, 0\n",prev_stmt, self.add_pc(), prev_exp)},
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Block {
        pub stmt: Stmt,
    }

    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "%entry:\n{}", self.stmt)
        }
    }
    #[derive(Debug)]
    pub struct Stmt {
        pub exp: UnaryExp,
    }
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // print!("{}ret %{}\n", self.exp, self.exp.load_pc());
            let prev_stmt = format!("{}",self.exp);
            write!(f, "{}  ret %{}\n", prev_stmt, self.exp.previous_pc())
        }
    }
}
