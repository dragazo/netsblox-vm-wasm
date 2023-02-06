mod utils;

use wasm_bindgen::prelude::*;

use netsblox_vm::bytecode::{ByteCode, InitInfo};
use netsblox_vm::ast::Parser;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Executable {
    bytecode: ByteCode,
    init_info: InitInfo,
}
#[wasm_bindgen]
impl Executable {
    pub fn compile(xml: &str) -> Result<Executable, String> {
        let proj = match Parser::default().parse(xml) {
            Ok(x) => x,
            Err(e) => return Err(format!("parse error: {e:?}")),
        };
        let role = match proj.roles.as_slice() {
            [x] => x,
            [] => return Err("no roles found in project".into()),
            _ => return Err("multiple roles found in project".into()),
        };
        let (bytecode, init_info, _, _) = match ByteCode::compile(role) {
            Ok(x) => x,
            Err(e) => return Err(format!("compile error: {e:?}")),
        };

        Ok(Self { bytecode, init_info })
    }
    pub fn encode_bytecode(&self) -> Vec<u8> {
        postcard::to_allocvec(&self.bytecode).unwrap()
    }
    pub fn encode_init_info(&self) -> Vec<u8> {
        postcard::to_allocvec(&self.init_info).unwrap()
    }
}
