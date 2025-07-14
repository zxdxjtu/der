use std::io::{Read, Result, Error, ErrorKind};
use crate::core::binary_format::*;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct DERDeserializer<R: Read> {
    reader: R,
}

impl<R: Read> DERDeserializer<R> {
    pub fn new(reader: R) -> Self {
        DERDeserializer { reader }
    }

    pub fn read_program(&mut self) -> Result<Program> {
        let header = self.read_header()?;
        
        if header.magic != DER_MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid DER magic number"));
        }

        let mut program = Program::new();
        program.header = header;

        // Read chunks
        for _ in 0..header.chunk_count {
            self.read_chunk(&mut program)?;
        }

        Ok(program)
    }

    fn read_header(&mut self) -> Result<FileHeader> {
        let mut magic = [0u8; 4];
        self.reader.read_exact(&mut magic)?;

        let version = self.reader.read_u16::<LittleEndian>()?;
        let flags = self.reader.read_u16::<LittleEndian>()?;
        let chunk_count = self.reader.read_u32::<LittleEndian>()?;
        
        let mut reserved = [0u8; 4];
        self.reader.read_exact(&mut reserved)?;

        Ok(FileHeader {
            magic,
            version,
            flags,
            chunk_count,
            reserved,
        })
    }

    fn read_chunk(&mut self, program: &mut Program) -> Result<()> {
        let chunk_header = self.read_chunk_header()?;

        match &chunk_header.chunk_type {
            b"META" => self.read_metadata_chunk(program, chunk_header.size)?,
            b"IMPL" => self.read_impl_chunk(program, chunk_header.size)?,
            b"CNST" => self.read_const_chunk(program, chunk_header.size)?,
            b"PROF" => {
                // Skip proof chunks for now
                let mut buffer = vec![0u8; chunk_header.size as usize];
                self.reader.read_exact(&mut buffer)?;
            }
            _ => {
                // Skip unknown chunks
                let mut buffer = vec![0u8; chunk_header.size as usize];
                self.reader.read_exact(&mut buffer)?;
            }
        }

        Ok(())
    }

    fn read_chunk_header(&mut self) -> Result<ChunkHeader> {
        let mut chunk_type = [0u8; 4];
        self.reader.read_exact(&mut chunk_type)?;

        let size = self.reader.read_u32::<LittleEndian>()?;
        let flags = self.reader.read_u32::<LittleEndian>()?;
        let checksum = self.reader.read_u32::<LittleEndian>()?;

        Ok(ChunkHeader {
            chunk_type,
            size,
            flags,
            checksum,
        })
    }

    fn read_metadata_chunk(&mut self, program: &mut Program, size: u32) -> Result<()> {
        let mut buffer = vec![0u8; size as usize];
        self.reader.read_exact(&mut buffer)?;
        let mut cursor = std::io::Cursor::new(buffer);

        // Read entry point
        program.metadata.entry_point = cursor.read_u32::<LittleEndian>()?;

        // Read capabilities
        let cap_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..cap_count {
            let cap_id = cursor.read_u32::<LittleEndian>()?;
            let cap = match cap_id {
                1 => Capability::FileSystem,
                2 => Capability::Network,
                3 => Capability::Process,
                4 => Capability::UI,
                5 => Capability::ExternalCode,
                _ => continue,
            };
            program.metadata.required_capabilities.push(cap);
        }

        // Read traits
        let trait_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..trait_count {
            // Read trait name
            let name_len = cursor.read_u32::<LittleEndian>()? as usize;
            let mut name_bytes = vec![0u8; name_len];
            cursor.read_exact(&mut name_bytes)?;
            let name = String::from_utf8(name_bytes)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 in trait name"))?;

            let mut trait_def = Trait {
                name,
                preconditions: Vec::new(),
                postconditions: Vec::new(),
            };

            // Read preconditions
            let precond_count = cursor.read_u32::<LittleEndian>()?;
            for _ in 0..precond_count {
                let len = cursor.read_u32::<LittleEndian>()? as usize;
                let mut bytes = vec![0u8; len];
                cursor.read_exact(&mut bytes)?;
                let precond = String::from_utf8(bytes)
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 in precondition"))?;
                trait_def.preconditions.push(precond);
            }

            // Read postconditions
            let postcond_count = cursor.read_u32::<LittleEndian>()?;
            for _ in 0..postcond_count {
                let len = cursor.read_u32::<LittleEndian>()? as usize;
                let mut bytes = vec![0u8; len];
                cursor.read_exact(&mut bytes)?;
                let postcond = String::from_utf8(bytes)
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 in postcondition"))?;
                trait_def.postconditions.push(postcond);
            }

            program.metadata.traits.push(trait_def);
        }

        Ok(())
    }

    fn read_impl_chunk(&mut self, program: &mut Program, size: u32) -> Result<()> {
        let node_count = size as usize / std::mem::size_of::<Node>();
        
        for _ in 0..node_count {
            let node = self.read_node()?;
            program.nodes.push(node);
        }

        Ok(())
    }

    fn read_node(&mut self) -> Result<Node> {
        let opcode = self.reader.read_u16::<LittleEndian>()?;
        let flags = self.reader.read_u16::<LittleEndian>()?;
        let result_id = self.reader.read_u32::<LittleEndian>()?;
        let timestamp = self.reader.read_u64::<LittleEndian>()?;
        let arg_count = self.reader.read_u8()?;
        
        let mut args = [0u32; 3];
        for i in 0..3 {
            args[i] = self.reader.read_u32::<LittleEndian>()?;
        }

        Ok(Node {
            opcode,
            flags,
            result_id,
            timestamp,
            arg_count,
            args,
        })
    }

    fn read_const_chunk(&mut self, program: &mut Program, size: u32) -> Result<()> {
        let mut buffer = vec![0u8; size as usize];
        self.reader.read_exact(&mut buffer)?;
        let mut cursor = std::io::Cursor::new(buffer);

        // Read integers
        let int_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..int_count {
            let val = cursor.read_i64::<LittleEndian>()?;
            program.constants.integers.push(val);
        }

        // Read floats
        let float_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..float_count {
            let val = cursor.read_f64::<LittleEndian>()?;
            program.constants.floats.push(val);
        }

        // Read strings
        let string_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..string_count {
            let len = cursor.read_u32::<LittleEndian>()? as usize;
            let mut bytes = vec![0u8; len];
            cursor.read_exact(&mut bytes)?;
            let string = String::from_utf8(bytes)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 in string constant"))?;
            program.constants.strings.push(string);
        }

        // Read booleans
        let bool_count = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..bool_count {
            let val = cursor.read_u8()? != 0;
            program.constants.booleans.push(val);
        }

        Ok(())
    }
}