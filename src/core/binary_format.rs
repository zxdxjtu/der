use std::io::{Read, Write, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DER_MAGIC: [u8; 4] = [0x44, 0x45, 0x52, 0x21]; // "DER!"
pub const VERSION: u16 = 0x0100; // Version 1.0

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FileHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub flags: u16,
    pub chunk_count: u32,
    pub reserved: [u8; 4],
}

impl FileHeader {
    pub fn new(chunk_count: u32) -> Self {
        FileHeader {
            magic: DER_MAGIC,
            version: VERSION,
            flags: 0,
            chunk_count,
            reserved: [0; 4],
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkHeader {
    pub chunk_type: [u8; 4],
    pub size: u32,
    pub flags: u32,
    pub checksum: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub opcode: u16,
    pub flags: u16,
    pub result_id: u32,
    pub timestamp: u64,
    pub arg_count: u8,
    pub args: [u32; 3],
}

impl Node {
    pub fn new(opcode: OpCode, result_id: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        Node {
            opcode: opcode as u16,
            flags: 0,
            result_id,
            timestamp,
            arg_count: 0,
            args: [0; 3],
        }
    }

    pub fn with_args(mut self, args: &[u32]) -> Self {
        self.arg_count = args.len().min(3) as u8;
        for (i, &arg) in args.iter().take(3).enumerate() {
            self.args[i] = arg;
        }
        self
    }

    pub fn set_flag(&mut self, flag: NodeFlag) {
        self.flags |= flag as u16;
    }

    pub fn has_flag(&self, flag: NodeFlag) -> bool {
        self.flags & (flag as u16) != 0
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // Control Flow
    Nop = 0x0000,
    Return = 0x0001,
    Call = 0x0002,
    Branch = 0x0003,
    
    // Arithmetic
    Add = 0x0100,
    Sub = 0x0101,
    Mul = 0x0102,
    Div = 0x0103,
    Mod = 0x0104,
    
    // Comparison
    Eq = 0x0200,
    Ne = 0x0201,
    Lt = 0x0202,
    Le = 0x0203,
    Gt = 0x0204,
    Ge = 0x0205,
    
    // Logical
    And = 0x0300,
    Or = 0x0301,
    Not = 0x0302,
    Xor = 0x0303,
    
    // Memory
    Load = 0x0400,
    Store = 0x0401,
    Alloc = 0x0402,
    Free = 0x0403,
    LoadArg = 0x0404,
    
    // Constants
    ConstInt = 0x0500,
    ConstFloat = 0x0501,
    ConstString = 0x0502,
    ConstBool = 0x0503,
    
    // Data Structures
    CreateArray = 0x0600,
    CreateMap = 0x0601,
    ArrayGet = 0x0602,
    ArraySet = 0x0603,
    MapGet = 0x0604,
    MapSet = 0x0605,
    
    // Functions
    DefineFunc = 0x0700,
    CreateClosure = 0x0701,
    
    // Type Operations
    Cast = 0x0800,
    TypeOf = 0x0801,
    
    // IO Operations
    Print = 0x0900,
    Read = 0x0901,
    
    // UI Operations (for future visualization)
    UICreateElement = 0x0A00,
    UISetAttribute = 0x0A01,
    UIAppendChild = 0x0A02,
    
    // Async Operations
    AsyncBegin = 0x0B00,
    AsyncAwait = 0x0B01,
    AsyncComplete = 0x0B02,
    
    // External Calls (FXI)
    ExternalCall = 0x0F00,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum NodeFlag {
    IsAsync = 0x0001,
    IsPure = 0x0002,
    IsUnsafe = 0x0004,
    HasSideEffects = 0x0008,
    IsTerminal = 0x0010,
    IsEntryPoint = 0x0020,
    RequiresProof = 0x0040,
}

#[derive(Clone)]
pub struct ConstantPool {
    pub integers: Vec<i64>,
    pub floats: Vec<f64>,
    pub strings: Vec<String>,
    pub booleans: Vec<bool>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            integers: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            booleans: Vec::new(),
        }
    }

    pub fn add_int(&mut self, value: i64) -> u32 {
        let index = self.integers.len() as u32;
        self.integers.push(value);
        index
    }

    pub fn add_float(&mut self, value: f64) -> u32 {
        let index = self.floats.len() as u32;
        self.floats.push(value);
        index
    }

    pub fn add_string(&mut self, value: String) -> u32 {
        let index = self.strings.len() as u32;
        self.strings.push(value);
        index
    }

    pub fn add_bool(&mut self, value: bool) -> u32 {
        let index = self.booleans.len() as u32;
        self.booleans.push(value);
        index
    }

    pub fn get_int(&self, index: u32) -> Option<i64> {
        self.integers.get(index as usize).copied()
    }

    pub fn get_float(&self, index: u32) -> Option<f64> {
        self.floats.get(index as usize).copied()
    }

    pub fn get_string(&self, index: u32) -> Option<&String> {
        self.strings.get(index as usize)
    }

    pub fn get_bool(&self, index: u32) -> Option<bool> {
        self.booleans.get(index as usize).copied()
    }
}

#[derive(Clone)]
pub struct Program {
    pub header: FileHeader,
    pub nodes: Vec<Node>,
    pub constants: ConstantPool,
    pub metadata: ProgramMetadata,
}

#[derive(Clone)]
pub struct ProgramMetadata {
    pub entry_point: u32,
    pub required_capabilities: Vec<Capability>,
    pub traits: Vec<Trait>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Capability {
    FileSystem,
    Network,
    Process,
    UI,
    ExternalCode,
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            header: FileHeader::new(0),
            nodes: Vec::new(),
            constants: ConstantPool::new(),
            metadata: ProgramMetadata {
                entry_point: 0,
                required_capabilities: Vec::new(),
                traits: Vec::new(),
            },
        }
    }

    pub fn add_node(&mut self, node: Node) -> u32 {
        let index = self.nodes.len() as u32;
        self.nodes.push(node);
        index
    }

    pub fn set_entry_point(&mut self, node_id: u32) {
        self.metadata.entry_point = node_id;
    }

    pub fn require_capability(&mut self, cap: Capability) {
        if !self.metadata.required_capabilities.contains(&cap) {
            self.metadata.required_capabilities.push(cap);
        }
    }
}