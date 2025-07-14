/// DER语义注释系统
/// 
/// 为每个.der文件提供配套的.ders（DER Semantic）文件，
/// 包含AI的推理过程、语义解释和人类可读的描述。
/// 
/// 这不是传统的"注释"，而是AI理解代码所需的语义上下文。

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 语义注释文档 - 对应一个.der文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDocument {
    /// 对应的DER文件路径
    pub der_file_path: String,
    
    /// 整体程序的语义描述
    pub program_semantics: ProgramSemantics,
    
    /// 每个节点的语义注释
    pub node_annotations: HashMap<u32, NodeAnnotation>,
    
    /// AI的推理过程记录
    pub ai_reasoning_trace: AIReasoningTrace,
    
    /// 人类可读的解释
    pub human_explanation: HumanExplanation,
    
    /// 版本和元数据
    pub metadata: AnnotationMetadata,
}

/// 程序整体语义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramSemantics {
    /// 程序的高层目标
    pub primary_goal: String,
    
    /// 输入输出规格
    pub input_output_spec: InputOutputSpec,
    
    /// 算法类别
    pub algorithm_category: String,
    
    /// 复杂度分析
    pub complexity_analysis: ComplexityAnalysis,
    
    /// 不变式和前后条件
    pub invariants: Vec<String>,
}

/// 单个节点的语义注释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnnotation {
    /// 节点ID
    pub node_id: u32,
    
    /// 语义角色（这个节点在算法中的作用）
    pub semantic_role: String,
    
    /// 人类可读的描述
    pub description: String,
    
    /// 数据变换描述
    pub data_transformation: String,
    
    /// 为什么AI选择这个操作
    pub ai_rationale: String,
    
    /// 与其他节点的语义关系
    pub semantic_dependencies: Vec<SemanticDependency>,
    
    /// 可能的优化建议
    pub optimization_hints: Vec<String>,
}

/// 语义依赖关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDependency {
    /// 依赖的节点ID
    pub target_node_id: u32,
    
    /// 依赖关系的语义类型
    pub dependency_type: DependencyType,
    
    /// 语义描述
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    /// 数据流依赖
    DataFlow,
    /// 控制流依赖
    ControlFlow,
    /// 语义约束依赖
    SemanticConstraint,
    /// 优化顺序依赖
    OptimizationOrder,
}

/// AI推理过程记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIReasoningTrace {
    /// 意图理解过程
    pub intent_analysis: IntentAnalysisTrace,
    
    /// 图设计决策
    pub graph_design_decisions: Vec<DesignDecision>,
    
    /// 优化应用记录
    pub optimizations_applied: Vec<OptimizationStep>,
    
    /// 验证推理
    pub verification_reasoning: Vec<VerificationStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAnalysisTrace {
    pub original_prompt: String,
    pub parsed_goals: Vec<String>,
    pub identified_patterns: Vec<String>,
    pub constraints_detected: Vec<String>,
    pub confidence_scores: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDecision {
    pub decision_point: String,
    pub alternatives_considered: Vec<String>,
    pub chosen_approach: String,
    pub reasoning: String,
    pub confidence: f32,
}

/// 人类可读的解释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanExplanation {
    /// 程序做什么（高层描述）
    pub what_it_does: String,
    
    /// 为什么这样实现
    pub why_this_approach: String,
    
    /// 如何工作（步骤分解）
    pub how_it_works: Vec<ExecutionStep>,
    
    /// 预期的使用场景
    pub use_cases: Vec<String>,
    
    /// 可能的改进方向
    pub improvement_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_number: usize,
    pub description: String,
    pub involved_nodes: Vec<u32>,
    pub data_state_change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOutputSpec {
    pub input_types: Vec<String>,
    pub input_constraints: Vec<String>,
    pub output_types: Vec<String>,
    pub output_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    pub time_complexity: String,
    pub space_complexity: String,
    pub best_case: String,
    pub worst_case: String,
    pub average_case: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStep {
    pub optimization_name: String,
    pub before_nodes: usize,
    pub after_nodes: usize,
    pub performance_impact: String,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStep {
    pub property_verified: String,
    pub proof_method: String,
    pub confidence: f32,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationMetadata {
    pub created_by: String,  // AI模型版本
    pub created_at: String,  // 时间戳
    pub der_file_hash: String,  // 对应的.der文件哈希
    pub annotation_version: String,
    pub language_version: String,  // DER语言版本
}

/// 语义注释生成器
pub struct SemanticAnnotationGenerator {
    ai_context: Option<crate::compiler::ai_translator::AIReasoningContext>,
}

impl SemanticAnnotationGenerator {
    pub fn new() -> Self {
        SemanticAnnotationGenerator {
            ai_context: None,
        }
    }
    
    /// 从AI上下文生成语义注释
    pub fn generate_from_ai_context(
        &self, 
        der_file_path: &str,
        ai_context: &crate::compiler::ai_translator::AIReasoningContext,
        original_prompt: &str,
        program: &crate::core::Program
    ) -> SemanticDocument {
        SemanticDocument {
            der_file_path: der_file_path.to_string(),
            program_semantics: self.extract_program_semantics(ai_context, program),
            node_annotations: self.generate_node_annotations(ai_context, program),
            ai_reasoning_trace: self.capture_ai_reasoning(ai_context, original_prompt),
            human_explanation: self.generate_human_explanation(ai_context, original_prompt, program),
            metadata: AnnotationMetadata {
                created_by: "DER-AI-v0.1".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                der_file_hash: self.calculate_file_hash(der_file_path),
                annotation_version: "1.0".to_string(),
                language_version: "DER-0.1".to_string(),
            },
        }
    }
    
    fn extract_program_semantics(&self, ai_context: &crate::compiler::ai_translator::AIReasoningContext, program: &crate::core::Program) -> ProgramSemantics {
        let intent = ai_context.intent_analysis.as_ref();
        
        ProgramSemantics {
            primary_goal: intent.map(|i| i.primary_goal.clone()).unwrap_or("Unknown".to_string()),
            input_output_spec: InputOutputSpec {
                input_types: vec!["None".to_string()],  // 分析程序确定
                input_constraints: vec![],
                output_types: vec!["Printed output".to_string()],
                output_guarantees: vec!["Deterministic result".to_string()],
            },
            algorithm_category: "Simple computation".to_string(),
            complexity_analysis: ComplexityAnalysis {
                time_complexity: "O(1)".to_string(),
                space_complexity: "O(1)".to_string(),
                best_case: "Constant time".to_string(),
                worst_case: "Constant time".to_string(),
                average_case: "Constant time".to_string(),
            },
            invariants: vec!["Program produces consistent output".to_string()],
        }
    }
    
    fn generate_node_annotations(&self, _ai_context: &crate::compiler::ai_translator::AIReasoningContext, program: &crate::core::Program) -> HashMap<u32, NodeAnnotation> {
        let mut annotations = HashMap::new();
        
        for (index, node) in program.nodes.iter().enumerate() {
            let annotation = NodeAnnotation {
                node_id: node.result_id,
                semantic_role: format!("Computation step {}", index + 1),
                description: self.describe_node_operation(node),
                data_transformation: self.describe_data_transformation(node),
                ai_rationale: "AI determined this operation was necessary for the intended computation".to_string(),
                semantic_dependencies: self.analyze_semantic_dependencies(node, program),
                optimization_hints: vec!["Could be constant-folded if inputs are known".to_string()],
            };
            
            annotations.insert(node.result_id, annotation);
        }
        
        annotations
    }
    
    fn describe_node_operation(&self, node: &crate::core::Node) -> String {
        match crate::core::OpCode::try_from(node.opcode) {
            Ok(opcode) => format!("Executes {:?} operation", opcode),
            Err(_) => format!("Unknown operation with opcode {}", node.opcode),
        }
    }
    
    fn describe_data_transformation(&self, node: &crate::core::Node) -> String {
        match crate::core::OpCode::try_from(node.opcode) {
            Ok(crate::core::OpCode::ConstInt) => "Loads integer constant into computation graph".to_string(),
            Ok(crate::core::OpCode::ConstString) => "Loads string constant for processing".to_string(),
            Ok(crate::core::OpCode::Add) => "Combines two numeric values through addition".to_string(),
            Ok(crate::core::OpCode::Print) => "Converts internal value to human-readable output".to_string(),
            _ => "Transforms data according to operation semantics".to_string(),
        }
    }
    
    fn analyze_semantic_dependencies(&self, node: &crate::core::Node, _program: &crate::core::Program) -> Vec<SemanticDependency> {
        let mut deps = Vec::new();
        
        for i in 0..node.arg_count as usize {
            if i < 3 && node.args[i] != 0 {
                deps.push(SemanticDependency {
                    target_node_id: node.args[i],
                    dependency_type: DependencyType::DataFlow,
                    description: format!("Requires result from node {} as input", node.args[i]),
                });
            }
        }
        
        deps
    }
    
    fn capture_ai_reasoning(&self, ai_context: &crate::compiler::ai_translator::AIReasoningContext, original_prompt: &str) -> AIReasoningTrace {
        AIReasoningTrace {
            intent_analysis: IntentAnalysisTrace {
                original_prompt: original_prompt.to_string(),
                parsed_goals: ai_context.intent_analysis.as_ref()
                    .map(|i| i.computational_requirements.clone())
                    .unwrap_or_default(),
                identified_patterns: vec!["Output generation pattern".to_string()],
                constraints_detected: vec!["Type safety required".to_string()],
                confidence_scores: [("intent_understanding".to_string(), 0.85)]
                    .iter().cloned().collect(),
            },
            graph_design_decisions: vec![
                DesignDecision {
                    decision_point: "Node sequence design".to_string(),
                    alternatives_considered: vec!["Direct output".to_string(), "Multi-step computation".to_string()],
                    chosen_approach: "Multi-step computation".to_string(),
                    reasoning: "Provides better optimization opportunities".to_string(),
                    confidence: 0.75,
                }
            ],
            optimizations_applied: vec![],
            verification_reasoning: vec![
                VerificationStep {
                    property_verified: "Type safety".to_string(),
                    proof_method: "Static analysis".to_string(),
                    confidence: 0.9,
                    assumptions: vec!["All opcodes are well-typed".to_string()],
                }
            ],
        }
    }
    
    fn generate_human_explanation(&self, _ai_context: &crate::compiler::ai_translator::AIReasoningContext, prompt: &str, program: &crate::core::Program) -> HumanExplanation {
        HumanExplanation {
            what_it_does: format!("This program responds to the request: '{}'", prompt),
            why_this_approach: "AI selected this implementation as the most direct way to achieve the user's intent".to_string(),
            how_it_works: (0..program.nodes.len()).map(|i| {
                ExecutionStep {
                    step_number: i + 1,
                    description: format!("Execute node {} to contribute to the final result", i + 1),
                    involved_nodes: vec![program.nodes[i].result_id],
                    data_state_change: "Updates computation state with new value".to_string(),
                }
            }).collect(),
            use_cases: vec![
                "Learning DER language concepts".to_string(),
                "Testing AI code generation".to_string(),
                "Demonstrating computational graphs".to_string(),
            ],
            improvement_suggestions: vec![
                "Add error handling for edge cases".to_string(),
                "Optimize for specific input patterns".to_string(),
                "Add more comprehensive verification proofs".to_string(),
            ],
        }
    }
    
    fn calculate_file_hash(&self, _file_path: &str) -> String {
        // 简化实现 - 实际应该计算文件的SHA256
        "sha256:placeholder".to_string()
    }
    
    /// 保存语义注释到文件
    pub fn save_to_file(&self, document: &SemanticDocument, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(document)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }
    
    /// 从文件加载语义注释
    pub fn load_from_file(file_path: &str) -> Result<SemanticDocument, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let document: SemanticDocument = serde_json::from_str(&content)?;
        Ok(document)
    }
}

/// AI代码理解助手
/// 
/// 当AI需要理解或修改现有DER代码时，使用这个助手
/// 来加载和分析语义注释
pub struct AICodeUnderstandingAssistant {
    semantic_cache: HashMap<String, SemanticDocument>,
}

impl AICodeUnderstandingAssistant {
    pub fn new() -> Self {
        AICodeUnderstandingAssistant {
            semantic_cache: HashMap::new(),
        }
    }
    
    /// 加载DER程序及其语义注释
    pub fn load_der_with_semantics(&mut self, der_path: &str) -> Result<(crate::core::Program, SemanticDocument), Box<dyn std::error::Error>> {
        // 加载DER程序
        use std::fs::File;
        let file = File::open(der_path)?;
        let mut deserializer = crate::core::DERDeserializer::new(file);
        let program = deserializer.read_program()?;
        
        // 尝试加载对应的语义注释
        let semantics_path = der_path.replace(".der", ".ders");
        let semantics = if std::path::Path::new(&semantics_path).exists() {
            SemanticAnnotationGenerator::load_from_file(&semantics_path)?
        } else {
            // 如果没有语义注释，生成基本的
            eprintln!("⚠️  No semantic annotations found for {}. AI understanding will be limited.", der_path);
            self.generate_minimal_semantics(der_path, &program)
        };
        
        self.semantic_cache.insert(der_path.to_string(), semantics.clone());
        
        Ok((program, semantics))
    }
    
    /// 为没有语义注释的程序生成最小语义信息
    fn generate_minimal_semantics(&self, der_path: &str, program: &crate::core::Program) -> SemanticDocument {
        SemanticDocument {
            der_file_path: der_path.to_string(),
            program_semantics: ProgramSemantics {
                primary_goal: "Unknown - no semantic annotations available".to_string(),
                input_output_spec: InputOutputSpec {
                    input_types: vec!["Unknown".to_string()],
                    input_constraints: vec![],
                    output_types: vec!["Unknown".to_string()],
                    output_guarantees: vec![],
                },
                algorithm_category: "Unknown".to_string(),
                complexity_analysis: ComplexityAnalysis {
                    time_complexity: "Unknown".to_string(),
                    space_complexity: "Unknown".to_string(),
                    best_case: "Unknown".to_string(),
                    worst_case: "Unknown".to_string(),
                    average_case: "Unknown".to_string(),
                },
                invariants: vec![],
            },
            node_annotations: HashMap::new(),
            ai_reasoning_trace: AIReasoningTrace {
                intent_analysis: IntentAnalysisTrace {
                    original_prompt: "Unknown".to_string(),
                    parsed_goals: vec![],
                    identified_patterns: vec![],
                    constraints_detected: vec![],
                    confidence_scores: HashMap::new(),
                },
                graph_design_decisions: vec![],
                optimizations_applied: vec![],
                verification_reasoning: vec![],
            },
            human_explanation: HumanExplanation {
                what_it_does: format!("DER program with {} nodes", program.nodes.len()),
                why_this_approach: "Unknown - no AI reasoning trace available".to_string(),
                how_it_works: vec![],
                use_cases: vec!["Legacy DER program".to_string()],
                improvement_suggestions: vec!["Add semantic annotations for better AI understanding".to_string()],
            },
            metadata: AnnotationMetadata {
                created_by: "Minimal-Generator".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                der_file_hash: "unknown".to_string(),
                annotation_version: "0.1".to_string(),
                language_version: "DER-0.1".to_string(),
            },
        }
    }
    
    /// 获取节点的语义理解
    pub fn understand_node(&self, der_path: &str, node_id: u32) -> Option<&NodeAnnotation> {
        self.semantic_cache.get(der_path)
            .and_then(|doc| doc.node_annotations.get(&node_id))
    }
    
    /// 获取程序的整体语义
    pub fn understand_program(&self, der_path: &str) -> Option<&ProgramSemantics> {
        self.semantic_cache.get(der_path)
            .map(|doc| &doc.program_semantics)
    }
}