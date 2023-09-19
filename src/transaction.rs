#[derive(Debug, Clone)]
pub enum Transaction {
    DeployContract { name: String, bytecode: Vec<u8> },
    ExecuteContract { name: String, args: Vec<String> },
}