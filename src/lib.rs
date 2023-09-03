pub mod ast {
    use std::fmt;

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
        pub num: i32,
    }
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ret {}\n", self.num)
        }
    }
}
