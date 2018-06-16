#[derive(Debug, Clone)]
pub enum Instruction {
    CreateObject {
        type_spec: (String, String),
        num_args: usize,
    },
    Assign {
        // Assign a = pop(), b = pop(), a[name] = `b`
        name: String,
    },
    GetTopScope,
    GetModuleGlobals {
        name: String,
    },
    CallMethod {
        name: String,
        num_args: usize,
        use_result: bool,
    },
    GetMember {
        name: String,
    },
    LookupName {
        name: String,
    },
    CallFunctionObject {
        num_args: usize,
        use_result: bool,
    },
    CreateString {
        value: String,
    },
    CreateInt {
        value: i64,
    },
    CreateBool {
        value: bool,
    },
    Diag,
    DebugPrintObject,
}
