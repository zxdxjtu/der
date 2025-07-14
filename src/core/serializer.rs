use std::io::{Write, Result};
use crate::core::binary_format::*;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct DERSerializer<W: Write> {
    writer: W,
}

impl<W: Write> DERSerializer<W> {
    pub fn new(writer: W) -> Self {
        DERSerializer { writer }
    }

    pub fn write_program(&mut self, program: &Program) -> Result<()> {
        // Write file header
        self.write_header(&program.header)?;

        // Write metadata chunk
        self.write_metadata_chunk(&program.metadata)?;

        // Write implementation chunk
        self.write_impl_chunk(&program.nodes)?;

        // Write constant pool chunk
        self.write_const_chunk(&program.constants)?;

        Ok(())
    }

    fn write_header(&mut self, header: &FileHeader) -> Result<()> {
        self.writer.write_all(&header.magic)?;
        self.writer.write_u16::<LittleEndian>(header.version)?;
        self.writer.write_u16::<LittleEndian>(header.flags)?;
        self.writer.write_u32::<LittleEndian>(header.chunk_count)?;
        self.writer.write_all(&header.reserved)?;
        Ok(())
    }

    fn write_metadata_chunk(&mut self, metadata: &ProgramMetadata) -> Result<()> {
        let chunk_type = *b"META";
        let mut chunk_data = Vec::new();

        // Write entry point
        chunk_data.write_u32::<LittleEndian>(metadata.entry_point)?;

        // Write capabilities
        chunk_data.write_u32::<LittleEndian>(metadata.required_capabilities.len() as u32)?;
        for cap in &metadata.required_capabilities {
            let cap_id = match cap {
                Capability::FileSystem => 1u32,
                Capability::Network => 2,
                Capability::Process => 3,
                Capability::UI => 4,
                Capability::ExternalCode => 5,
            };
            chunk_data.write_u32::<LittleEndian>(cap_id)?;
        }

        // Write traits
        chunk_data.write_u32::<LittleEndian>(metadata.traits.len() as u32)?;
        for trait_def in &metadata.traits {
            // Write trait name
            let name_bytes = trait_def.name.as_bytes();
            chunk_data.write_u32::<LittleEndian>(name_bytes.len() as u32)?;
            chunk_data.write_all(name_bytes)?;

            // Write preconditions
            chunk_data.write_u32::<LittleEndian>(trait_def.preconditions.len() as u32)?;
            for precond in &trait_def.preconditions {
                let bytes = precond.as_bytes();
                chunk_data.write_u32::<LittleEndian>(bytes.len() as u32)?;
                chunk_data.write_all(bytes)?;
            }

            // Write postconditions
            chunk_data.write_u32::<LittleEndian>(trait_def.postconditions.len() as u32)?;
            for postcond in &trait_def.postconditions {
                let bytes = postcond.as_bytes();
                chunk_data.write_u32::<LittleEndian>(bytes.len() as u32)?;
                chunk_data.write_all(bytes)?;
            }
        }

        self.write_chunk_header(chunk_type, chunk_data.len() as u32)?;
        self.writer.write_all(&chunk_data)?;
        Ok(())
    }

    fn write_impl_chunk(&mut self, nodes: &[Node]) -> Result<()> {
        let chunk_type = *b"IMPL";
        let chunk_size = (nodes.len() * std::mem::size_of::<Node>()) as u32;

        self.write_chunk_header(chunk_type, chunk_size)?;

        for node in nodes {
            self.write_node(node)?;
        }

        Ok(())
    }

    fn write_node(&mut self, node: &Node) -> Result<()> {
        self.writer.write_u16::<LittleEndian>(node.opcode)?;
        self.writer.write_u16::<LittleEndian>(node.flags)?;
        self.writer.write_u32::<LittleEndian>(node.result_id)?;
        self.writer.write_u64::<LittleEndian>(node.timestamp)?;
        self.writer.write_u8(node.arg_count)?;
        for arg in &node.args {
            self.writer.write_u32::<LittleEndian>(*arg)?;
        }
        Ok(())
    }

    fn write_const_chunk(&mut self, constants: &ConstantPool) -> Result<()> {
        let chunk_type = *b"CNST";
        let mut chunk_data = Vec::new();

        // Write integers
        chunk_data.write_u32::<LittleEndian>(constants.integers.len() as u32)?;
        for &val in &constants.integers {
            chunk_data.write_i64::<LittleEndian>(val)?;
        }

        // Write floats
        chunk_data.write_u32::<LittleEndian>(constants.floats.len() as u32)?;
        for &val in &constants.floats {
            chunk_data.write_f64::<LittleEndian>(val)?;
        }

        // Write strings
        chunk_data.write_u32::<LittleEndian>(constants.strings.len() as u32)?;
        for val in &constants.strings {
            let bytes = val.as_bytes();
            chunk_data.write_u32::<LittleEndian>(bytes.len() as u32)?;
            chunk_data.write_all(bytes)?;
        }

        // Write booleans
        chunk_data.write_u32::<LittleEndian>(constants.booleans.len() as u32)?;
        for &val in &constants.booleans {
            chunk_data.write_u8(if val { 1 } else { 0 })?;
        }

        self.write_chunk_header(chunk_type, chunk_data.len() as u32)?;
        self.writer.write_all(&chunk_data)?;
        Ok(())
    }

    fn write_chunk_header(&mut self, chunk_type: [u8; 4], size: u32) -> Result<()> {
        let header = ChunkHeader {
            chunk_type,
            size,
            flags: 0,
            checksum: 0, // TODO: Implement checksum calculation
        };

        self.writer.write_all(&header.chunk_type)?;
        self.writer.write_u32::<LittleEndian>(header.size)?;
        self.writer.write_u32::<LittleEndian>(header.flags)?;
        self.writer.write_u32::<LittleEndian>(header.checksum)?;
        Ok(())
    }
}