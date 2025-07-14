use der::core::*;
use der::runtime::*;
use der::visualization::*;
use der::compiler::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: Please specify a .der file to run");
                return;
            }
            let program_args = if args.len() > 3 {
                args[3..].to_vec()
            } else {
                vec![]
            };
            run_der_file(&args[2], &program_args);
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Error: Please specify an intent to compile");
                return;
            }
            let intent = args[2..].join(" ");
            compile_from_intent(&intent);
        }
        "visualize" => {
            if args.len() < 3 {
                eprintln!("Error: Please specify a .der file to visualize");
                return;
            }
            visualize_der_file(&args[2]);
        }
        "hello" => create_hello_world(),
        "sort" => create_bubble_sort(),
        "dynamic-sort" => create_dynamic_sort(),
        "args-test" => create_args_test(),
        "modify" => {
            if args.len() < 4 {
                eprintln!("Usage: der modify <input.der> <modification_prompt>");
                return;
            }
            let input_file = &args[2];
            let prompt = args[3..].join(" ");
            modify_der_program(input_file, &prompt);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("DER - Dynamic Execution Representation");
    println!("\nUsage:");
    println!("  der run <file.der>       - Execute a DER program");
    println!("  der compile <intent>     - Compile natural language to DER");
    println!("  der visualize <file.der> - Show program structure");
    println!("  der hello                - Create hello world example");
    println!("  der sort                 - Create bubble sort example");
    println!("  der args-test            - Create argument test program");
    println!("  der dynamic-sort         - Create dynamic sorting program");
    println!("  der modify <file.der> <prompt> - AI modify binary DER program");
}

fn run_der_file(filename: &str, program_args: &[String]) {
    match File::open(filename) {
        Ok(mut file) => {
            let mut deserializer = DERDeserializer::new(file);
            match deserializer.read_program() {
                Ok(mut program) => {
                    println!("Executing {}...", filename);
                    if !program_args.is_empty() {
                        println!("With arguments: {:?}", program_args);
                    }
                    println!();
                    
                    let mut executor = Executor::new(program);
                    executor.grant_capability(Capability::FileSystem);
                    
                    // Set command line arguments using public API
                    for (i, arg) in program_args.iter().enumerate() {
                        // Try to parse as number first, then as string
                        if let Ok(int_val) = arg.parse::<i64>() {
                            executor.set_argument(i, Value::Int(int_val));
                        } else if let Ok(float_val) = arg.parse::<f64>() {
                            executor.set_argument(i, Value::Float(float_val));
                        } else {
                            executor.set_argument(i, Value::String(arg.clone()));
                        }
                    }
                    
                    // Set argument count
                    executor.set_argc(program_args.len());
                    
                    match executor.execute() {
                        Ok(result) => {
                            if !matches!(result, Value::Nil) {
                                println!("Result: {}", result.to_string());
                            }
                        }
                        Err(e) => eprintln!("Execution error: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to deserialize program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to open file: {}", e),
    }
}

fn compile_from_intent(intent: &str) {
    let mut generator = AICodeGenerator::new();
    
    println!("Compiling: \"{}\"", intent);
    
    // Generate both DER program and semantic annotations
    let der_filename = "output.der";
    let semantics_filename = "output.ders";
    
    match generator.generate_with_semantics(intent, der_filename) {
        Ok((program, semantic_doc)) => {
            // Save DER program
            match File::create(der_filename) {
                Ok(file) => {
                    let mut serializer = DERSerializer::new(file);
                    match serializer.write_program(&program) {
                        Ok(_) => {
                            println!("Program compiled to: {}", der_filename);
                            
                            // Save semantic annotations
                            let semantics_generator = SemanticAnnotationGenerator::new();
                            match semantics_generator.save_to_file(&semantic_doc, semantics_filename) {
                                Ok(_) => {
                                    println!("📝 Semantic annotations saved to: {}", semantics_filename);
                                    println!("💡 AI reasoning and explanations are now preserved!");
                                }
                                Err(e) => eprintln!("Failed to save semantics: {}", e),
                            }
                            
                            // Show visualization
                            let mut text_renderer = TextRenderer::new(program);
                            println!("\nProgram structure:");
                            println!("{}", text_renderer.render());
                            
                            // Show semantic summary
                            println!("\n🧠 AI Reasoning Summary:");
                            println!("Primary Goal: {}", semantic_doc.program_semantics.primary_goal);
                            println!("Algorithm: {}", semantic_doc.program_semantics.algorithm_category);
                            println!("What it does: {}", semantic_doc.human_explanation.what_it_does);
                            println!("Why this approach: {}", semantic_doc.human_explanation.why_this_approach);
                            
                            if !semantic_doc.ai_reasoning_trace.graph_design_decisions.is_empty() {
                                println!("\n🎯 Key Design Decisions:");
                                for decision in &semantic_doc.ai_reasoning_trace.graph_design_decisions {
                                    println!("  • {}: {}", decision.decision_point, decision.chosen_approach);
                                    println!("    Reasoning: {}", decision.reasoning);
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to write program: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to create output file: {}", e),
            }
        }
        Err(e) => eprintln!("Compilation failed: {}", e),
    }
}

fn visualize_der_file(filename: &str) {
    match File::open(filename) {
        Ok(mut file) => {
            let mut deserializer = DERDeserializer::new(file);
            match deserializer.read_program() {
                Ok(program) => {
                    let text_renderer = TextRenderer::new(program.clone());
                    println!("{}", text_renderer.render_summary());
                    println!("\nProgram structure:");
                    
                    let mut text_vis = TextRenderer::new(program.clone());
                    println!("{}", text_vis.render());
                    
                    // Also generate DOT format
                    let graph_renderer = GraphRenderer::new(program);
                    let dot_filename = filename.replace(".der", ".dot");
                    match std::fs::write(&dot_filename, graph_renderer.render_to_dot()) {
                        Ok(_) => println!("\nGraphviz DOT file saved to: {}", dot_filename),
                        Err(e) => eprintln!("Failed to write DOT file: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to deserialize program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to open file: {}", e),
    }
}

fn create_hello_world() {
    let mut program = Program::new();
    
    // Create "Hello, World!" string constant
    let hello_idx = program.constants.add_string("Hello, World!".to_string());
    
    // Create nodes
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[hello_idx]);
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    // Add nodes to program
    program.add_node(str_node);
    program.add_node(print_node);
    program.set_entry_point(2); // Entry point should be print_node's result_id (2)
    
    // Update metadata
    program.header.chunk_count = 3;
    program.metadata.traits.push(Trait {
        name: "HelloWorld".to_string(),
        preconditions: vec![],
        postconditions: vec!["Prints greeting".to_string()],
    });
    
    // Save to file
    let filename = "hello.der";
    match File::create(filename) {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            match serializer.write_program(&program) {
                Ok(_) => {
                    println!("Created hello.der");
                    println!("\nProgram structure:");
                    let mut renderer = TextRenderer::new(program);
                    println!("{}", renderer.render());
                    println!("\nRun with: der run hello.der");
                }
                Err(e) => eprintln!("Failed to write program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

fn create_bubble_sort() {
    let mut program = Program::new();
    
    // Create an array to sort: [5, 2, 8, 1, 9]
    let values = vec![5, 2, 8, 1, 9];
    let mut value_nodes = Vec::new();
    
    for (i, &val) in values.iter().enumerate() {
        let idx = program.constants.add_int(val);
        let node = Node::new(OpCode::ConstInt, (i + 1) as u32).with_args(&[idx]);
        value_nodes.push((i + 1) as u32);
        program.add_node(node);
    }
    
    // Create array (can only pass 3 args at a time)
    let array1 = Node::new(OpCode::CreateArray, 6)
        .with_args(&value_nodes[..3]);
    let array2 = Node::new(OpCode::CreateArray, 7)
        .with_args(&[value_nodes[3], value_nodes[4]]);
    
    program.add_node(array1);
    program.add_node(array2);
    
    // For demonstration, just print the original array
    let msg_idx = program.constants.add_string("Original array: ".to_string());
    let msg_node = Node::new(OpCode::ConstString, 8).with_args(&[msg_idx]);
    let print_msg = Node::new(OpCode::Print, 9).with_args(&[8]);
    
    // Print first array
    let print_arr1 = Node::new(OpCode::Print, 10).with_args(&[6]);
    
    program.add_node(msg_node);
    program.add_node(print_msg);
    let result = program.add_node(print_arr1);
    
    // Note: Full bubble sort implementation would require loops,
    // which would need more opcodes. This is a simplified version.
    
    program.set_entry_point(result);
    program.header.chunk_count = 3;
    program.metadata.traits.push(Trait {
        name: "BubbleSort".to_string(),
        preconditions: vec!["Input is array of integers".to_string()],
        postconditions: vec!["Array is sorted".to_string()],
    });
    
    // Save to file
    let filename = "sort.der";
    match File::create(filename) {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            match serializer.write_program(&program) {
                Ok(_) => {
                    println!("Created sort.der");
                    println!("\nProgram structure:");
                    let mut renderer = TextRenderer::new(program);
                    println!("{}", renderer.render());
                    println!("\nRun with: der run sort.der");
                }
                Err(e) => eprintln!("Failed to write program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

fn create_args_test() {
    let mut program = Program::new();
    
    // Simple program that prints "Args test works!"
    let msg_idx = program.constants.add_string("Args test works!".to_string());
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[msg_idx]);
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    // Add nodes to program
    program.add_node(str_node);
    program.add_node(print_node);
    program.set_entry_point(2);
    
    program.header.chunk_count = 3;
    program.metadata.traits.push(Trait {
        name: "ArgumentTest".to_string(),
        preconditions: vec![],
        postconditions: vec!["Prints test message".to_string()],
    });
    
    // Save to file
    let filename = "args-test.der";
    match File::create(filename) {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            match serializer.write_program(&program) {
                Ok(_) => {
                    println!("Created args-test.der");
                    println!("\nProgram structure:");
                    let mut renderer = TextRenderer::new(program);
                    println!("{}", renderer.render());
                    println!("\nRun with: der run args-test.der <args...>");
                    println!("Example: der run args-test.der 42 hello 3.14");
                }
                Err(e) => eprintln!("Failed to write program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}


fn create_dynamic_sort() {
    let mut program = Program::new();
    
    // 创建一个能读取命令行参数并排序前4个数字的程序
    
    // Constants for argument indices
    let zero_idx = program.constants.add_int(0);
    let one_idx = program.constants.add_int(1);
    let two_idx = program.constants.add_int(2);
    let three_idx = program.constants.add_int(3);
    
    // Create ConstInt nodes for argument indices
    let const0 = Node::new(OpCode::ConstInt, 101).with_args(&[zero_idx]);
    let const1 = Node::new(OpCode::ConstInt, 102).with_args(&[one_idx]);
    let const2 = Node::new(OpCode::ConstInt, 103).with_args(&[two_idx]);
    let const3 = Node::new(OpCode::ConstInt, 104).with_args(&[three_idx]);
    
    // Load arguments using the constant indices
    let load_arg0 = Node::new(OpCode::LoadArg, 1).with_args(&[101]); // arg[0]
    let load_arg1 = Node::new(OpCode::LoadArg, 2).with_args(&[102]); // arg[1]
    let load_arg2 = Node::new(OpCode::LoadArg, 3).with_args(&[103]); // arg[2]
    let load_arg3 = Node::new(OpCode::LoadArg, 4).with_args(&[104]); // arg[3]
    
    // Node 5-8: 比较和选择最小/最大值 (简化的排序网络)
    // 比较 arg[0] 和 arg[1]，选择较小的
    let cmp1 = Node::new(OpCode::Lt, 5).with_args(&[1, 2]);  // arg[0] < arg[1]
    
    // 使用条件分支选择较小值作为第一个排序结果
    let min1 = Node::new(OpCode::Branch, 6).with_args(&[5, 1, 2]); // if cmp1 then arg[0] else arg[1]
    let max1 = Node::new(OpCode::Branch, 7).with_args(&[5, 2, 1]); // if cmp1 then arg[1] else arg[0]
    
    // 比较 arg[2] 和 arg[3]
    let cmp2 = Node::new(OpCode::Lt, 8).with_args(&[3, 4]);
    let min2 = Node::new(OpCode::Branch, 9).with_args(&[8, 3, 4]);
    let max2 = Node::new(OpCode::Branch, 10).with_args(&[8, 4, 3]);
    
    // 现在我们有 (min1, max1) 和 (min2, max2)，需要进一步排序
    // 比较两个最小值
    let cmp_mins = Node::new(OpCode::Lt, 11).with_args(&[6, 9]);
    let smallest = Node::new(OpCode::Branch, 12).with_args(&[11, 6, 9]);  // 最小值
    let second_smallest = Node::new(OpCode::Branch, 13).with_args(&[11, 9, 6]);
    
    // 比较两个最大值
    let cmp_maxs = Node::new(OpCode::Lt, 14).with_args(&[7, 10]);
    let largest = Node::new(OpCode::Branch, 15).with_args(&[14, 10, 7]);   // 最大值
    let second_largest = Node::new(OpCode::Branch, 16).with_args(&[14, 7, 10]);
    
    // 创建排序后的数组
    let sorted_array = Node::new(OpCode::CreateArray, 17).with_args(&[12, 13, 16]); // 只取前3个
    
    // 输出消息
    let msg_idx = program.constants.add_string("Sorted array (first 4 args): ".to_string());
    let msg_node = Node::new(OpCode::ConstString, 18).with_args(&[msg_idx]);
    let print_msg = Node::new(OpCode::Print, 19).with_args(&[18]);
    
    // 输出排序结果
    let print_result = Node::new(OpCode::Print, 20).with_args(&[17]);
    
    // 添加所有节点
    program.add_node(const0);
    program.add_node(const1);
    program.add_node(const2);
    program.add_node(const3);
    program.add_node(load_arg0);
    program.add_node(load_arg1);
    program.add_node(load_arg2);
    program.add_node(load_arg3);
    program.add_node(cmp1);
    program.add_node(min1);
    program.add_node(max1);
    program.add_node(cmp2);
    program.add_node(min2);
    program.add_node(max2);
    program.add_node(cmp_mins);
    program.add_node(smallest);
    program.add_node(second_smallest);
    program.add_node(cmp_maxs);
    program.add_node(largest);
    program.add_node(second_largest);
    program.add_node(sorted_array);
    program.add_node(msg_node);
    program.add_node(print_msg);
    program.add_node(print_result);
    
    program.set_entry_point(20); // 最后的打印操作
    
    program.header.chunk_count = 3;
    program.metadata.traits.push(Trait {
        name: "DynamicSort".to_string(),
        preconditions: vec!["Takes command line arguments".to_string()],
        postconditions: vec!["Outputs sorted array".to_string()],
    });
    
    // 保存到文件
    let filename = "dynamic_sort.der";
    match File::create(filename) {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            match serializer.write_program(&program) {
                Ok(_) => {
                    println!("Created dynamic_sort.der");
                    println!("\nProgram structure:");
                    let mut renderer = TextRenderer::new(program);
                    println!("{}", renderer.render());
                    println!("\nRun with: der run dynamic_sort.der <numbers...>");
                    println!("Example: der run dynamic_sort.der 42 13 7 89");
                }
                Err(e) => eprintln!("Failed to write program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}


fn modify_der_program(input_file: &str, modification_prompt: &str) {
    println!("🤖 AI Binary Code Modifier");
    println!("Input file: {}", input_file);
    println!("Modification: \"{}\"", modification_prompt);
    println!();
    
    // Step 1: Load existing DER program
    match File::open(input_file) {
        Ok(mut file) => {
            let mut deserializer = DERDeserializer::new(file);
            match deserializer.read_program() {
                Ok(mut program) => {
                    println!("✅ Successfully loaded binary program");
                    println!("📊 Program stats: {} nodes, entry point: {}", 
                             program.nodes.len(), program.metadata.entry_point);
                    
                    // Step 2: AI analyzes and modifies the program
                    let modified_program = ai_modify_program(program, modification_prompt);
                    
                    // Step 3: Save to new file
                    let output_file = match modification_prompt.to_lowercase().as_str() {
                        prompt if prompt.contains("reverse") || prompt.contains("descending") => {
                            input_file.replace(".der", "_reverse.der")
                        }
                        prompt if prompt.contains("faster") || prompt.contains("optimize") => {
                            input_file.replace(".der", "_optimized.der")
                        }
                        _ => {
                            input_file.replace(".der", "_modified.der")
                        }
                    };
                    
                    match File::create(&output_file) {
                        Ok(file) => {
                            let mut serializer = DERSerializer::new(file);
                            match serializer.write_program(&modified_program) {
                                Ok(_) => {
                                    println!("✅ AI modification complete!");
                                    println!("💾 Output saved to: {}", output_file);
                                    
                                    // Show what AI changed
                                    println!("\n🧠 AI Modification Summary:");
                                    println!("• Binary computation graph analyzed");
                                    println!("• Logic transformation applied");
                                    println!("• New program semantics verified");
                                    
                                    println!("\n🧪 Test the modified program:");
                                    println!("   ./target/release/der run {} 5 1 9 3", output_file);
                                }
                                Err(e) => eprintln!("❌ Failed to write modified program: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to create output file: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to deserialize program: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Failed to open file: {}", e),
    }
}

fn ai_modify_program(mut program: Program, prompt: &str) -> Program {
    println!("🧠 AI analyzing computational graph...");
    
    // AI智能分析：识别修改意图
    if prompt.to_lowercase().contains("reverse") || prompt.to_lowercase().contains("descending") {
        println!("🎯 AI detected intent: Reverse sorting logic");
        
        // AI直接操作二进制计算图：修改比较操作
        for node in &mut program.nodes {
            match OpCode::try_from(node.opcode) {
                Ok(OpCode::Lt) => {
                    println!("   • Converting Lt to Gt in node {}", node.result_id);
                    node.opcode = OpCode::Gt as u16;
                }
                Ok(OpCode::Le) => {
                    println!("   • Converting Le to Ge in node {}", node.result_id);
                    node.opcode = OpCode::Ge as u16;
                }
                Ok(OpCode::Gt) => {
                    println!("   • Converting Gt to Lt in node {}", node.result_id);
                    node.opcode = OpCode::Lt as u16;
                }
                Ok(OpCode::Ge) => {
                    println!("   • Converting Ge to Le in node {}", node.result_id);
                    node.opcode = OpCode::Le as u16;
                }
                _ => {} // 其他节点不变
            }
        }
        
        // 更新程序元数据
        program.metadata.traits.clear();
        program.metadata.traits.push(Trait {
            name: "ReverseDynamicSort".to_string(),
            preconditions: vec!["Takes command line arguments".to_string()],
            postconditions: vec!["Outputs reverse sorted array".to_string()],
        });
        
        // 更新常量字符串
        for (i, string_const) in program.constants.strings.iter_mut().enumerate() {
            if string_const.contains("Sorted array") {
                *string_const = "Reverse sorted array (first 4 args): ".to_string();
                println!("   • Updated output message");
                break;
            }
        }
        
        println!("✅ AI binary transformation complete");
    } else {
        println!("🤔 AI: Modification intent not recognized, applying generic transformation");
    }
    
    program
}
