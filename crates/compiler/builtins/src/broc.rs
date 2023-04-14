use broc_module::symbol::ModuleId;

#[inline(always)]
pub fn module_source(module_id: ModuleId) -> &'static str {
    match module_id {
        ModuleId::RESULT => RESULT,
        ModuleId::NUM => NUM,
        ModuleId::STR => STR,
        ModuleId::LIST => LIST,
        ModuleId::DICT => DICT,
        ModuleId::SET => SET,
        ModuleId::BOX => BOX,
        ModuleId::BOOL => BOOL,
        ModuleId::ENCODE => ENCODE,
        ModuleId::DECODE => DECODE,
        ModuleId::HASH => HASH,
        ModuleId::JSON => JSON,
        _ => panic!(
            "ModuleId {:?} is not part of the standard library",
            module_id
        ),
    }
}

const RESULT: &str = include_str!("../broc/Result.broc");
const NUM: &str = include_str!("../broc/Num.broc");
const STR: &str = include_str!("../broc/Str.broc");
const LIST: &str = include_str!("../broc/List.broc");
const DICT: &str = include_str!("../broc/Dict.broc");
const SET: &str = include_str!("../broc/Set.broc");
const BOX: &str = include_str!("../broc/Box.broc");
const BOOL: &str = include_str!("../broc/Bool.broc");
const ENCODE: &str = include_str!("../broc/Encode.broc");
const DECODE: &str = include_str!("../broc/Decode.broc");
const HASH: &str = include_str!("../broc/Hash.broc");
const JSON: &str = include_str!("../broc/Json.broc");
