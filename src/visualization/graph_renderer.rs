use crate::core::{Program, Node, OpCode};
use std::collections::{HashMap, HashSet};

pub struct GraphRenderer {
    program: Program,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: u32,
    pub label: String,
    pub opcode: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from: u32,
    pub to: u32,
    pub label: String,
}

pub struct GraphLayout {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub width: f32,
    pub height: f32,
}

impl GraphRenderer {
    pub fn new(program: Program) -> Self {
        GraphRenderer { program }
    }

    pub fn render_to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph DER {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box, style=rounded, fontname=\"Arial\"];\n");
        dot.push_str("  edge [fontname=\"Arial\", fontsize=10];\n\n");

        // Render nodes
        for (idx, node) in self.program.nodes.iter().enumerate() {
            let opcode_name = OpCode::try_from(node.opcode)
                .map(|op| format!("{:?}", op))
                .unwrap_or_else(|_| format!("Unknown({})", node.opcode));

            let label = self.get_node_label(node, &opcode_name);
            let color = self.get_node_color(&opcode_name);

            dot.push_str(&format!(
                "  n{} [label=\"{}\", fillcolor=\"{}\", style=\"filled,rounded\"];\n",
                node.result_id, label, color
            ));
        }

        dot.push_str("\n");

        // Render edges
        for (idx, node) in self.program.nodes.iter().enumerate() {
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    // Find the node that produces this result
                    if let Some(arg_node) = self.find_node_by_result_id(arg_id) {
                        dot.push_str(&format!(
                            "  n{} -> n{} [label=\"arg{}\"];\n",
                            arg_node.result_id, node.result_id, i
                        ));
                    }
                }
            }
        }

        // Mark entry point
        let entry_point = self.program.metadata.entry_point;
        if let Some(entry_node) = self.program.nodes.get(entry_point as usize) {
            dot.push_str(&format!(
                "  n{} [peripheries=2, penwidth=2];\n",
                entry_node.result_id
            ));
        }

        dot.push_str("}\n");
        dot
    }

    pub fn render_to_mermaid(&self) -> String {
        let mut mermaid = String::new();
        mermaid.push_str("graph TD\n");

        // Render nodes
        for (idx, node) in self.program.nodes.iter().enumerate() {
            let opcode_name = OpCode::try_from(node.opcode)
                .map(|op| format!("{:?}", op))
                .unwrap_or_else(|_| format!("Unknown({})", node.opcode));

            let label = self.get_node_label(node, &opcode_name);
            
            mermaid.push_str(&format!("    n{}[\"{}\"]\n", node.result_id, label));
        }

        // Apply styling
        mermaid.push_str("\n");
        for (idx, node) in self.program.nodes.iter().enumerate() {
            let opcode_name = OpCode::try_from(node.opcode)
                .map(|op| format!("{:?}", op))
                .unwrap_or_else(|_| format!("Unknown({})", node.opcode));

            let style = self.get_mermaid_style(&opcode_name);
            mermaid.push_str(&format!("    style n{} {}\n", node.result_id, style));
        }

        // Render edges
        mermaid.push_str("\n");
        for (idx, node) in self.program.nodes.iter().enumerate() {
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    if let Some(arg_node) = self.find_node_by_result_id(arg_id) {
                        mermaid.push_str(&format!(
                            "    n{} -->|arg{}| n{}\n",
                            arg_node.result_id, i, node.result_id
                        ));
                    }
                }
            }
        }

        // Mark entry point
        let entry_point = self.program.metadata.entry_point;
        if let Some(entry_node) = self.program.nodes.get(entry_point as usize) {
            mermaid.push_str(&format!(
                "    style n{} stroke:#ff0000,stroke-width:4px\n",
                entry_node.result_id
            ));
        }

        mermaid
    }

    pub fn calculate_layout(&self) -> GraphLayout {
        let mut layout = GraphLayout {
            nodes: Vec::new(),
            edges: Vec::new(),
            width: 800.0,
            height: 600.0,
        };

        // Simple hierarchical layout
        let levels = self.calculate_node_levels();
        let max_level = levels.values().max().copied().unwrap_or(0);
        
        // Group nodes by level
        let mut nodes_by_level: HashMap<usize, Vec<&Node>> = HashMap::new();
        for (node_id, level) in &levels {
            if let Some(node) = self.program.nodes.get(*node_id as usize) {
                nodes_by_level.entry(*level).or_insert(Vec::new()).push(node);
            }
        }

        // Position nodes
        let level_height = 100.0;
        for (level, nodes) in nodes_by_level {
            let node_width = 120.0;
            let node_spacing = 20.0;
            let total_width = nodes.len() as f32 * (node_width + node_spacing) - node_spacing;
            let start_x = (layout.width - total_width) / 2.0;
            let y = level as f32 * level_height + 50.0;

            for (i, node) in nodes.iter().enumerate() {
                let x = start_x + i as f32 * (node_width + node_spacing);
                
                let opcode_name = OpCode::try_from(node.opcode)
                    .map(|op| format!("{:?}", op))
                    .unwrap_or_else(|_| format!("Unknown({})", node.opcode));

                let label = self.get_node_label(node, &opcode_name);

                layout.nodes.push(GraphNode {
                    id: node.result_id,
                    label,
                    opcode: opcode_name,
                    x,
                    y,
                    width: node_width,
                    height: 60.0,
                });
            }
        }

        // Create edges
        for node in &self.program.nodes {
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    if let Some(arg_node) = self.find_node_by_result_id(arg_id) {
                        layout.edges.push(GraphEdge {
                            from: arg_node.result_id,
                            to: node.result_id,
                            label: format!("arg{}", i),
                        });
                    }
                }
            }
        }

        layout.height = (max_level + 2) as f32 * level_height;
        layout
    }

    fn find_node_by_result_id(&self, result_id: u32) -> Option<&Node> {
        self.program.nodes.iter().find(|n| n.result_id == result_id)
    }

    fn get_node_label(&self, node: &Node, opcode_name: &str) -> String {
        let mut label = format!("Node {}\\n{}", node.result_id, opcode_name);

        // Add constant values to the label
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                if let Some(val) = self.program.constants.get_int(node.args[0]) {
                    label.push_str(&format!("\\nValue: {}", val));
                }
            }
            Ok(OpCode::ConstFloat) => {
                if let Some(val) = self.program.constants.get_float(node.args[0]) {
                    label.push_str(&format!("\\nValue: {}", val));
                }
            }
            Ok(OpCode::ConstString) => {
                if let Some(val) = self.program.constants.get_string(node.args[0]) {
                    label.push_str(&format!("\\nValue: \\\"{}\\\"", val));
                }
            }
            Ok(OpCode::ConstBool) => {
                if let Some(val) = self.program.constants.get_bool(node.args[0]) {
                    label.push_str(&format!("\\nValue: {}", val));
                }
            }
            _ => {}
        }

        label
    }

    fn get_node_color(&self, opcode_name: &str) -> &'static str {
        match opcode_name {
            "ConstInt" | "ConstFloat" | "ConstString" | "ConstBool" => "#e8f5e9",
            "Add" | "Sub" | "Mul" | "Div" | "Mod" => "#fff3e0",
            "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" => "#e3f2fd",
            "And" | "Or" | "Not" | "Xor" => "#f3e5f5",
            "Branch" => "#fff9c4",
            "Call" | "Return" => "#fce4ec",
            "DefineFunc" | "CreateClosure" => "#e1f5fe",
            "CreateArray" | "CreateMap" | "ArrayGet" | "ArraySet" | "MapGet" | "MapSet" => "#f1f8e9",
            "Print" | "Read" => "#efebe9",
            _ => "#f5f5f5",
        }
    }

    fn get_mermaid_style(&self, opcode_name: &str) -> &'static str {
        match opcode_name {
            "ConstInt" | "ConstFloat" | "ConstString" | "ConstBool" => "fill:#e8f5e9,stroke:#4caf50",
            "Add" | "Sub" | "Mul" | "Div" | "Mod" => "fill:#fff3e0,stroke:#ff9800",
            "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" => "fill:#e3f2fd,stroke:#2196f3",
            "And" | "Or" | "Not" | "Xor" => "fill:#f3e5f5,stroke:#9c27b0",
            "Branch" => "fill:#fff9c4,stroke:#ffeb3b",
            "Call" | "Return" => "fill:#fce4ec,stroke:#e91e63",
            "DefineFunc" | "CreateClosure" => "fill:#e1f5fe,stroke:#00bcd4",
            "CreateArray" | "CreateMap" | "ArrayGet" | "ArraySet" | "MapGet" | "MapSet" => "fill:#f1f8e9,stroke:#8bc34a",
            "Print" | "Read" => "fill:#efebe9,stroke:#795548",
            _ => "fill:#f5f5f5,stroke:#9e9e9e",
        }
    }

    fn calculate_node_levels(&self) -> HashMap<u32, usize> {
        let mut levels: HashMap<u32, usize> = HashMap::new();
        let mut visited: HashSet<u32> = HashSet::new();

        // Start from entry point
        let entry_point = self.program.metadata.entry_point;
        if let Some(entry_node) = self.program.nodes.get(entry_point as usize) {
            self.calculate_node_level(entry_node.result_id, 0, &mut levels, &mut visited);
        }

        // Process any unvisited nodes
        for node in &self.program.nodes {
            if !visited.contains(&node.result_id) {
                self.calculate_node_level(node.result_id, 0, &mut levels, &mut visited);
            }
        }

        levels
    }

    fn calculate_node_level(
        &self,
        node_id: u32,
        current_level: usize,
        levels: &mut HashMap<u32, usize>,
        visited: &mut HashSet<u32>,
    ) {
        if visited.contains(&node_id) {
            return;
        }

        visited.insert(node_id);
        levels.insert(node_id, current_level);

        if let Some(node) = self.find_node_by_result_id(node_id) {
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    if let Some(arg_node) = self.find_node_by_result_id(arg_id) {
                        let new_level = current_level + 1;
                        if let Some(&existing_level) = levels.get(&arg_id) {
                            if new_level > existing_level {
                                levels.insert(arg_id, new_level);
                            }
                        } else {
                            self.calculate_node_level(arg_id, new_level, levels, visited);
                        }
                    }
                }
            }
        }
    }
}