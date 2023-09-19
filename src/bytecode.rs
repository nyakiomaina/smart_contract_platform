use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

pub struct ByteCode {
    code : Vec<u8>
}

impl ByteCode {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code }
    }

    // pub fn execute(&self) {
    //     let compiler_config = Cranelift::default();
    //     let engine = JIT::new(&compiler_config).engine();
    //     let store = Store::new(&engine);
    
    //     let module = Module::new(&store, &self.code).expect("Failed to create module");
        
    //     // Assuming the API now requires a mutable store as the first argument, 
    //     // and the imports as the third argument.
    //     let instance = Instance::new(&mut store, &module, &wasmer::imports! {})
    //         .expect("Failed to instantiate the module");
    
    //     let run = instance.exports.get_function("run")
    //         .expect("Failed to fetch the run function from the exports");
    //     run.call(&[]).expect("Failed to execute the run function");
    // }
    
}