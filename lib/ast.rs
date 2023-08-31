pub struct CompUnit {
    pub funt_def: FuncDef,
}

pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

pub struct FuncType(&str);
