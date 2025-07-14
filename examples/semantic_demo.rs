/// 语义注释系统演示
/// 
/// 这个演示展示了DER的语义注释系统如何工作，以及它如何解决
/// 你提出的"AI理解和人类阅读"的需求。

use der::core::*;
use der::compiler::*;

fn main() {
    println!("🎯 DER语义注释系统演示");
    println!("===================");
    println!();
    
    // 1. 展示传统二进制程序的问题
    demonstrate_binary_opacity();
    
    // 2. 展示DER的语义注释解决方案
    demonstrate_semantic_annotations();
    
    // 3. 展示AI如何使用语义注释理解代码
    demonstrate_ai_understanding();
    
    // 4. 展示人类如何通过语义注释理解代码
    demonstrate_human_understanding();
}

fn demonstrate_binary_opacity() {
    println!("❌ 传统二进制程序的问题");
    println!("------------------------");
    println!();
    
    // 创建一个简单的DER程序
    let mut program = Program::new();
    let val1 = program.constants.add_int(10);
    let val2 = program.constants.add_int(20);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[val1]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[val2]);
    let node3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    let node4 = Node::new(OpCode::Print, 4).with_args(&[3]);
    
    program.add_node(node1);
    program.add_node(node2);
    program.add_node(node3);
    let entry = program.add_node(node4);
    program.set_entry_point(entry);
    
    // 保存为二进制
    let file = std::fs::File::create("example_without_semantics.der").unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    println!("创建了一个简单的DER程序: example_without_semantics.der");
    
    // 显示二进制文件内容 - 对人类和AI都不友好
    let bytes = std::fs::read("example_without_semantics.der").unwrap();
    println!("二进制内容 (前32字节): {:?}", &bytes[..32.min(bytes.len())]);
    println!();
    println!("问题:");
    println!("• 🤖 AI无法理解程序意图");
    println!("• 👨‍💻 人类无法阅读二进制");
    println!("• 🔧 修改和调试困难");
    println!("• 📚 缺乏文档和解释");
    println!();
}

fn demonstrate_semantic_annotations() {
    println!("✅ DER语义注释解决方案");
    println!("----------------------");
    println!();
    
    // 使用AI翻译器生成带语义注释的程序
    let mut generator = AICodeGenerator::new();
    
    let prompt = "add two numbers 10 and 20 then print the result";
    println!("原始意图: \"{}\"", prompt);
    println!();
    
    match generator.generate_with_semantics(prompt, "example_with_semantics.der") {
        Ok((program, semantic_doc)) => {
            // 保存程序
            let file = std::fs::File::create("example_with_semantics.der").unwrap();
            let mut serializer = DERSerializer::new(file);
            serializer.write_program(&program).unwrap();
            
            // 保存语义注释
            let semantics_generator = SemanticAnnotationGenerator::new();
            semantics_generator.save_to_file(&semantic_doc, "example_with_semantics.ders").unwrap();
            
            println!("生成了两个文件:");
            println!("• 📦 example_with_semantics.der  - 高效的二进制程序");
            println!("• 📝 example_with_semantics.ders - 丰富的语义注释");
            println!();
            
            // 展示语义注释的内容
            println!("语义注释包含:");
            println!("• 🎯 程序目标: {}", semantic_doc.program_semantics.primary_goal);
            println!("• 🧮 算法类别: {}", semantic_doc.program_semantics.algorithm_category);
            println!("• ⏱️  时间复杂度: {}", semantic_doc.program_semantics.complexity_analysis.time_complexity);
            println!("• 💾 空间复杂度: {}", semantic_doc.program_semantics.complexity_analysis.space_complexity);
            println!("• 🧠 AI推理记录: {} 个决策点", semantic_doc.ai_reasoning_trace.graph_design_decisions.len());
            println!("• 📋 节点注释: {} 个节点", semantic_doc.node_annotations.len());
            println!();
        }
        Err(e) => println!("生成失败: {}", e),
    }
}

fn demonstrate_ai_understanding() {
    println!("🤖 AI如何使用语义注释理解代码");
    println!("------------------------------");
    println!();
    
    // 创建AI代码理解助手
    let mut assistant = AICodeUnderstandingAssistant::new();
    
    match assistant.load_der_with_semantics("example_with_semantics.der") {
        Ok((program, semantics)) => {
            println!("AI加载程序和语义注释成功!");
            println!();
            
            // AI理解程序整体语义
            if let Some(program_semantics) = assistant.understand_program("example_with_semantics.der") {
                println!("🧠 AI的程序理解:");
                println!("  主要目标: {}", program_semantics.primary_goal);
                println!("  输入类型: {:?}", program_semantics.input_output_spec.input_types);
                println!("  输出类型: {:?}", program_semantics.input_output_spec.output_types);
                println!("  不变式: {:?}", program_semantics.invariants);
                println!();
            }
            
            // AI理解具体节点
            println!("🔍 AI对每个节点的理解:");
            for node in &program.nodes {
                if let Some(annotation) = assistant.understand_node("example_with_semantics.der", node.result_id) {
                    println!("  节点 {}: {}", node.result_id, annotation.semantic_role);
                    println!("    描述: {}", annotation.description);
                    println!("    数据变换: {}", annotation.data_transformation);
                    println!("    AI理由: {}", annotation.ai_rationale);
                    println!();
                }
            }
            
            println!("💡 优势:");
            println!("• AI可以快速理解程序意图");
            println!("• 保留了原始的推理过程");
            println!("• 支持智能代码修改和优化");
            println!("• 可以生成更好的解释给人类");
        }
        Err(e) => println!("AI理解失败: {}", e),
    }
    
    println!();
}

fn demonstrate_human_understanding() {
    println!("👨‍💻 人类如何通过语义注释理解代码");
    println!("--------------------------------");
    println!();
    
    // 读取语义注释文件
    match SemanticAnnotationGenerator::load_from_file("example_with_semantics.ders") {
        Ok(semantic_doc) => {
            println!("📖 人类友好的程序解释:");
            println!();
            
            // 高层次描述
            println!("🎯 这个程序的作用:");
            println!("   {}", semantic_doc.human_explanation.what_it_does);
            println!();
            
            println!("🤔 为什么采用这种方法:");
            println!("   {}", semantic_doc.human_explanation.why_this_approach);
            println!();
            
            // 执行步骤
            println!("⚙️  程序执行步骤:");
            for step in &semantic_doc.human_explanation.how_it_works {
                println!("   {}. {}", step.step_number, step.description);
                println!("      涉及节点: {:?}", step.involved_nodes);
                println!("      状态变化: {}", step.data_state_change);
            }
            println!();
            
            // AI的设计决策
            println!("🎯 AI的关键设计决策:");
            for decision in &semantic_doc.ai_reasoning_trace.graph_design_decisions {
                println!("   决策点: {}", decision.decision_point);
                println!("   考虑的方案: {:?}", decision.alternatives_considered);
                println!("   选择的方案: {}", decision.chosen_approach);
                println!("   理由: {}", decision.reasoning);
                println!("   信心度: {:.1}%", decision.confidence * 100.0);
                println!();
            }
            
            // 使用场景
            println!("🔧 适用场景:");
            for use_case in &semantic_doc.human_explanation.use_cases {
                println!("   • {}", use_case);
            }
            println!();
            
            // 改进建议
            println!("💡 改进建议:");
            for suggestion in &semantic_doc.human_explanation.improvement_suggestions {
                println!("   • {}", suggestion);
            }
            println!();
            
            println!("🌟 语义注释系统的价值:");
            println!("   • 保留AI的完整推理过程");
            println!("   • 提供人类可读的详细解释");
            println!("   • 支持代码审查和维护");
            println!("   • 便于学习和知识传递");
        }
        Err(e) => println!("读取语义注释失败: {}", e),
    }
    
    println!();
    println!("🎉 总结:");
    println!("--------");
    println!("DER的语义注释系统解决了AI-Native语言的关键挑战:");
    println!("• 🤖 AI可以理解和修改现有代码");
    println!("• 👨‍💻 人类可以理解AI生成的代码");
    println!("• 🔄 保持了二进制的高效性");
    println!("• 📚 提供了丰富的文档和解释");
    
    // 清理临时文件
    let _ = std::fs::remove_file("example_without_semantics.der");
    let _ = std::fs::remove_file("example_with_semantics.der");
    let _ = std::fs::remove_file("example_with_semantics.ders");
}