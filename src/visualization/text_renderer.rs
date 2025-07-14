use crate::core::{Program, Node, OpCode, ConstantPool};
use std::collections::HashMap;

pub struct TextRenderer {
    program: Program,
    rendered_nodes: HashMap<u32, String>,
}

impl TextRenderer {
    pub fn new(program: Program) -> Self {
        TextRenderer {
            program,
            rendered_nodes: HashMap::new(),
        }
    }

    pub fn render(&mut self) -> String {
        let entry_point = self.program.metadata.entry_point;
        self.render_node(entry_point, 0)
    }

    fn render_node(&mut self, node_id: u32, indent: usize) -> String {
        // Check if we've already rendered this node
        if let Some(cached) = self.rendered_nodes.get(&node_id) {
            return format!("{}<Reference to Node {}>", " ".repeat(indent), node_id);
        }

        let node = match self.program.nodes.get(node_id as usize) {
            Some(n) => n.clone(),
            None => return format!("{}<Invalid Node {}>", " ".repeat(indent), node_id),
        };

        let opcode = OpCode::try_from(node.opcode)
            .map(|op| format!("{:?}", op))
            .unwrap_or_else(|_| format!("Unknown({})", node.opcode));

        let mut result = format!("{}Node {} [{}]", " ".repeat(indent), node.result_id, opcode);

        // Add node description based on opcode
        let description = self.describe_node(&node);
        if !description.is_empty() {
            result.push_str(&format!(": {}", description));
        }

        // Cache this node's representation
        self.rendered_nodes.insert(node_id, result.clone());

        // Render arguments
        if node.arg_count > 0 {
            result.push_str("\n");
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    result.push_str(&format!("{}├─ ", " ".repeat(indent + 2)));
                    result.push_str(&self.render_node(arg_id, indent + 4));
                    if i < node.arg_count as usize - 1 {
                        result.push_str("\n");
                    }
                }
            }
        }

        result
    }

    fn describe_node(&self, node: &Node) -> String {
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                if let Some(val) = self.program.constants.get_int(node.args[0]) {
                    format!("{}", val)
                } else {
                    "Invalid constant".to_string()
                }
            }
            Ok(OpCode::ConstFloat) => {
                if let Some(val) = self.program.constants.get_float(node.args[0]) {
                    format!("{}", val)
                } else {
                    "Invalid constant".to_string()
                }
            }
            Ok(OpCode::ConstString) => {
                if let Some(val) = self.program.constants.get_string(node.args[0]) {
                    format!("\"{}\"", val)
                } else {
                    "Invalid constant".to_string()
                }
            }
            Ok(OpCode::ConstBool) => {
                if let Some(val) = self.program.constants.get_bool(node.args[0]) {
                    format!("{}", val)
                } else {
                    "Invalid constant".to_string()
                }
            }
            Ok(OpCode::Add) => "Addition".to_string(),
            Ok(OpCode::Sub) => "Subtraction".to_string(),
            Ok(OpCode::Mul) => "Multiplication".to_string(),
            Ok(OpCode::Div) => "Division".to_string(),
            Ok(OpCode::Eq) => "Equality check".to_string(),
            Ok(OpCode::Lt) => "Less than".to_string(),
            Ok(OpCode::Branch) => "Conditional branch".to_string(),
            Ok(OpCode::Call) => "Function call".to_string(),
            Ok(OpCode::DefineFunc) => "Function definition".to_string(),
            Ok(OpCode::CreateArray) => "Array creation".to_string(),
            Ok(OpCode::CreateMap) => "Map creation".to_string(),
            Ok(OpCode::Print) => "Print output".to_string(),
            _ => String::new(),
        }
    }

    pub fn render_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("=== DER Program Summary ===\n");
        summary.push_str(&format!("Total nodes: {}\n", self.program.nodes.len()));
        summary.push_str(&format!("Entry point: Node {}\n", self.program.metadata.entry_point));
        
        if !self.program.metadata.required_capabilities.is_empty() {
            summary.push_str("\nRequired capabilities:\n");
            for cap in &self.program.metadata.required_capabilities {
                summary.push_str(&format!("  - {:?}\n", cap));
            }
        }
        
        if !self.program.metadata.traits.is_empty() {
            summary.push_str("\nProgram traits:\n");
            for trait_def in &self.program.metadata.traits {
                summary.push_str(&format!("  - {}\n", trait_def.name));
                if !trait_def.preconditions.is_empty() {
                    summary.push_str("    Preconditions:\n");
                    for pre in &trait_def.preconditions {
                        summary.push_str(&format!("      * {}\n", pre));
                    }
                }
                if !trait_def.postconditions.is_empty() {
                    summary.push_str("    Postconditions:\n");
                    for post in &trait_def.postconditions {
                        summary.push_str(&format!("      * {}\n", post));
                    }
                }
            }
        }
        
        summary.push_str("\nOpcode usage:\n");
        let mut opcode_counts: HashMap<String, usize> = HashMap::new();
        for node in &self.program.nodes {
            let opcode_name = OpCode::try_from(node.opcode)
                .map(|op| format!("{:?}", op))
                .unwrap_or_else(|_| format!("Unknown({})", node.opcode));
            *opcode_counts.entry(opcode_name).or_insert(0) += 1;
        }
        
        let mut sorted_opcodes: Vec<_> = opcode_counts.into_iter().collect();
        sorted_opcodes.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        
        for (opcode, count) in sorted_opcodes {
            summary.push_str(&format!("  {} : {}\n", opcode, count));
        }
        
        summary
    }
}