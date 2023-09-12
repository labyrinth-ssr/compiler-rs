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
    #[derive(Debug)]
    pub enum BinaryOp {
        Mul,
        Div,
        Mod,
        Add,
        Sub,
        Eq,
        Ne,
        Lt,
        Gt,
        Le,
        Ge,
        And,
        Or,
    }

        
    #[derive(Debug)]
    pub enum Exp {
        Number(i32),
        UnaryExp(UnaryOp, Box<Exp>),
        BinaryExp(Box<Exp>, BinaryOp, Box<Exp>),
    }
    
    trait Pc {
        fn add_pc(&self) -> usize;
        fn previous_pc(&self) -> usize;
        fn load_pc(&self) -> usize;
    }

    impl Pc for Exp {
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

    // 什么时候调用write
    // 使用rc（多线程不安全）给expr编号
    impl fmt::Display for Exp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Exp::Number(n) => {
                    // println!("format Number({})", n);
                    write!(f, "{}", n) //
                },
                Exp::UnaryExp(op, exp) => {
                    // println!("format {:?},",self);
                    let mut prev_exp = String::new();
                    let mut prev_stmt = String::new();

                    match &**exp { //当前op的exp
                        Exp::Number(num) =>{ 
                            // println!("match number");   
                            prev_exp = num.to_string()}, // 当前op的exp，如果是
                        Exp::UnaryExp(unaryop, expression) => {
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt = format!("{}",exp);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp = format!("%{}", exp.previous_pc().to_string())
                        },
                        Exp::BinaryExp(_,_ ,_ )=>{}
                    }

                    match op {
                        UnaryOp::Pos => {
                            // print!("<Pos {} Pos>", prev_stmt);
                            write!(f, "{}", prev_stmt)},
                        UnaryOp::Neg => {
                            // print!("<Neg {}%{} = sub 0, {} Neg>\n",prev_stmt, self.load_pc(), prev_exp);
                            write!(f, "{}  %{} = sub 0, {}\n  ",prev_stmt, self.add_pc(), prev_exp)},
                        UnaryOp::Not => {
                            // print!("<Not {}%{} = eq {}, 0 Not>\n",prev_stmt, self.load_pc(), prev_exp);
                            write!(f, "{}  %{} = eq {}, 0\n  ",prev_stmt, self.add_pc(), prev_exp)},
                    }
                },
                Exp::BinaryExp(exp1,op ,exp2)=> {
                    // println!("format {:?},",self);
                    let mut prev_exp1 = String::new();
                    let mut prev_exp2 = String::new();
                    let mut prev_stmt1 = String::new();
                    let mut prev_stmt2 = String::new();

                    match &**exp1 { //当前op的exp1
                        Exp::Number(num) =>{
                            // println!("match number");   
                            prev_exp1 = num.to_string()
                        },
                        Exp::UnaryExp(unaryop, expression) => {
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt1 = format!("{}",exp1);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp1 = format!("%{}", exp1.previous_pc().to_string())
                        },
                        Exp::BinaryExp(expa,op0 ,expb ) =>{
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt2 = format!("{}",exp1);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp2 = format!("%{}", exp1.previous_pc().to_string())
                        }
                    }

                    match &**exp2 { //当前op的exp2
                        Exp::Number(num) =>{ 
                            // println!("match number");   
                            prev_exp2 = num.to_string()
                        }, // 当前op的exp2，如果是
                        Exp::UnaryExp(unaryop, expression) => {
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt2 = format!("{}",exp2);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp2 = format!("%{}", exp2.previous_pc().to_string())
                        },
                        Exp::BinaryExp(expa,op0 ,expb )=> {
                            // println!("format {:?} 's sub exp {:?}",self,exp);
                            prev_stmt2 = format!("{}",exp2);
                            // println!("prev_stmt:({})",prev_stmt);
                            // println!("prev_exp:[{}]",exp.previous_pc().to_string());
                            prev_exp2 = format!("%{}", exp2.previous_pc().to_string())
                        }
                    }

                    match op {
                        BinaryOp::Mul => {
                            // print!("<Mul {}%{} = mul {}, {} Mul>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}{}%{} = mul {}, {}\n  ",prev_stmt1,prev_stmt2, self.add_pc(), prev_exp1, prev_exp2)
                        },
                        BinaryOp::Div => {
                            // print!("<Div {}%{} = sdiv {}, {} Div>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}  %{} = sdiv {}, {}\n  ",prev_stmt1, self.add_pc(), prev_exp1, prev_exp2)
                        },
                        BinaryOp::Mod => {
                            // print!("<Mod {}%{} = srem {}, {} Mod
                            write!(f, "{}  {}  %{} = mod {}, {}\n  ",prev_stmt1,prev_stmt2,self.add_pc(), prev_exp1, prev_exp2)
                        },

                        BinaryOp::Add => {
                            // print!("<Add {}%{} = add {}, {} Add>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}{}%{} = add {}, {}\n  ",prev_stmt1,prev_stmt2,self.add_pc(), prev_exp1, prev_exp2)
                        },

                        BinaryOp::Sub =>{
                            // print!("<Sub {}%{} = sub {}, {} Sub>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}{}%{} = sub {}, {}\n  ",prev_stmt1,prev_stmt2, self.add_pc(), prev_exp1, prev_exp2)
                        },
                        BinaryOp::Eq =>{
                            // print!("<Eq {}%{} = eq {}, {} Eq>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}{}%{} = eq {}, {}\n  ",prev_stmt1,prev_stmt2, self.add_pc(), prev_exp1, prev_exp2)
                        },
                        BinaryOp::Le=>{
                            // print!("<Le {}%{} = sle {}, {} Le>\n",prev_stmt1, self.load_pc(), prev_exp1, prev_exp2);
                            write!(f, "{}{}%{} = le {}, {}\n  ",prev_stmt1,prev_stmt2, self.add_pc(), prev_exp1, prev_exp2)
                        },
                        _ =>{Ok(())}
                        }

                    }  


            }
        }
    }

    
    // impl fmt::Display for MulExp {
        
    // }
    #[derive(Debug)]
    pub struct Block {
        pub stmt: Stmt,
    }

    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "%entry:\n  {}", self.stmt)
        }
    }
    #[derive(Debug)]
    pub struct Stmt {
        pub exp: Exp,
    }
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // print!("{}ret %{}\n", self.exp, self.exp.load_pc());
            let prev_stmt = format!("{}",self.exp);
            write!(f, "{}ret %{}\n", prev_stmt, self.exp.previous_pc())
        }
    }
}
