#[derive(Debug)]
pub struct AssemblyFunctionHook<'a>(pub Vec<AssemblyFunctionHookData<'a>>);

impl<'a> AssemblyFunctionHook<'a> {
    pub fn new(values: Vec<AssemblyFunctionHookData<'a>>) -> Self {
        Self(values)
    }
}

#[derive(Debug)]
pub struct AssemblyFunctionHookData<'a> {
    pub(crate) suffix: String,
    pub(crate) execute_mode: &'a str,
    pub(crate) registers: Vec<&'a str>,
    pub(crate) callee_saved_registers: Vec<&'a str>,
    pub(crate) allocate_shadow_space: bool,
    pub(crate) asm_insert_before: Option<String>,
    pub(crate) asm_insert_after: Option<String>
}

impl<'a> AssemblyFunctionHookData<'a> {
    pub fn new(
    suffix: String,
    execute_mode: &'a str,
    registers: Vec<&'a str>,
    callee_saved_registers: Vec<&'a str>,
    allocate_shadow_space: bool,
    asm_insert_before: Option<String>,
    asm_insert_after: Option<String>
    ) -> Self {
        Self {
            suffix, execute_mode, registers,
            callee_saved_registers, allocate_shadow_space,
            asm_insert_before, asm_insert_after
        }
    }
}