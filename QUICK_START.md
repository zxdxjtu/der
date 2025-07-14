# DER Quick Start Guide

欢迎使用 DER (Dynamic Execution Representation) - 世界上第一个AI-Native编程语言！

## 什么是DER？

DER是一种专为AI设计的编程语言：
- 🤖 **AI是主要程序员** - AI直接生成和修改代码
- 📊 **二进制计算图** - 程序是高效的二进制格式，不是文本
- 🧠 **语义注释** - 每个程序都有配套的AI推理说明
- 👨‍💻 **人类通过意图交互** - 你说想要什么，AI来实现

## 环境准备

确保你有Rust环境：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

构建DER：
```bash
git clone <repository-url>
cd der
cargo build --release
```

## 第一步：运行预制的DER程序

我们先从运行一些预制的DER程序开始，体验DER的工作方式。

### 示例1：Hello World程序

```bash
# 生成hello world程序
./target/release/der hello

# 查看程序结构
./target/release/der visualize hello.der

# 运行程序
./target/release/der run hello.der
```

你会看到：
- `hello.der` - 二进制DER程序（高效执行）
- 程序的图形化结构显示
- 运行结果："Hello, World!"

### 示例2：数学计算程序

```bash
# 生成数学计算程序
./target/release/der sort

# 查看程序结构
./target/release/der visualize sort.der

# 运行程序
./target/release/der run sort.der
```

### 示例3：AI生成自定义程序

```bash
# 用自然语言生成程序
./target/release/der compile "calculate fibonacci of 5"

# 检查生成的文件
ls -la output.*
# output.der  - 二进制程序
# output.ders - 语义注释（AI的推理过程）

# 查看AI的解释
cat output.ders | head -50

# 运行程序
./target/release/der run output.der
```

## 第二步：让AI修改DER程序

现在我们进入DER的核心特性：让AI理解和修改现有的DER程序。

### 准备工作

确保你有AI工具（选择其一）：
```bash
# Claude CLI
pip install claude-cli

# 或者 Gemini CLI
pip install google-generativeai
```

### AI修改流程

#### 1. 让AI分析现有程序

```bash
# 使用Claude分析DER程序
claude -p "@hello.der @hello.ders 请分析这个DER程序的结构和功能"

# 或使用Gemini
gemini -p "@hello.der @hello.ders 请分析这个DER程序的结构和功能"
```

#### 2. 让AI修改程序

```bash
# 让AI修改hello程序，改成输出不同的消息
claude -p "@hello.der @hello.ders @CLAUDE.md 请修改这个DER程序，让它输出'Welcome to DER!'而不是'Hello, World!'"

# 检查AI生成的修改
ls -la hello_modified.*
```

#### 3. 测试修改后的程序

```bash
# 运行修改后的程序
./target/release/der run hello_modified.der

# 比较原程序和修改后的程序
./target/release/der visualize hello.der > original.txt
./target/release/der visualize hello_modified.der > modified.txt
diff original.txt modified.txt
```

### 更复杂的AI修改示例

#### 示例1：添加用户输入
```bash
claude -p "@output.der @output.ders @CLAUDE.md 请修改这个计算程序，添加用户输入功能，让用户可以输入两个数字进行计算"
```

#### 示例2：性能优化
```bash
claude -p "@sort.der @sort.ders @CLAUDE.md 请优化这个排序程序的性能，减少计算步骤"
```

#### 示例3：添加错误处理
```bash
claude -p "@output.der @output.ders @CLAUDE.md 请为这个程序添加除零错误检查和处理"
```

## 第三步：理解DER的AI-Native特性

### 语义注释系统

每个DER程序都有对应的`.ders`文件：

```bash
# 查看语义注释的结构
cat output.ders | jq '.'

# 重点关注这些部分：
# - program_semantics: 程序整体语义
# - node_annotations: 每个节点的作用
# - ai_reasoning_trace: AI的推理过程
# - human_explanation: 人类可读的解释
```

### AI理解能力演示

```bash
# 运行语义注释演示
cargo run --example semantic_demo
```

这个演示展示了：
- 传统二进制程序的理解困难
- DER语义注释如何解决这个问题
- AI如何利用语义注释理解代码
- 人类如何通过语义注释理解AI生成的代码

## 第四步：高级AI协作

### 让AI创建复杂程序

```bash
# 创建一个Web服务程序
claude -p "@CLAUDE.md 请创建一个DER程序，实现简单的HTTP服务器，监听8080端口，返回JSON响应"

# 创建数据处理程序
claude -p "@CLAUDE.md 请创建一个DER程序，读取CSV文件并计算平均值"

# 创建算法实现
claude -p "@CLAUDE.md 请创建一个DER程序，实现快速排序算法"
```

### AI程序调试

```bash
# 如果程序有问题，让AI调试
claude -p "@problematic.der @problematic.ders @CLAUDE.md 这个程序运行时出现错误：[错误信息]，请分析并修复"
```

### AI代码重构

```bash
# 让AI重构代码提高可读性
claude -p "@complex.der @complex.ders @CLAUDE.md 请重构这个程序，使其更容易理解和维护"

# 让AI优化性能
claude -p "@slow.der @slow.ders @CLAUDE.md 请优化这个程序的性能，减少内存使用"
```

## 第五步：验证和测试

### 完整的修改-测试循环

```bash
# 1. AI修改程序
claude -p "@program.der @program.ders @CLAUDE.md [修改要求]"

# 2. 验证语法和语义
./target/release/der visualize modified_program.der

# 3. 运行测试
./target/release/der run modified_program.der

# 4. 如果有问题，继续让AI修复
claude -p "@modified_program.der @modified_program.ders @CLAUDE.md 程序运行结果不正确：[问题描述]，请修复"

# 5. 重复直到满意
```

### 性能基准测试

```bash
# 比较不同版本的性能
time ./target/release/der run original.der
time ./target/release/der run optimized.der
```

## 理解DER的哲学

DER代表了编程的新范式：

### 传统编程
```
人类 → 写代码 → 编译器 → 可执行程序
```

### DER编程
```
人类意图 → AI理解 → DER计算图 ← AI修改
                    ↓
                  高效执行
```

### 核心优势

1. **AI友好**: 计算图格式天然适合AI理解和生成
2. **人类监督**: 语义注释让人类理解AI的工作
3. **高性能**: 二进制格式执行效率高
4. **可验证**: 形式化验证确保正确性
5. **迭代优化**: AI可以持续改进代码

## 常见工作流程

### 创建新程序
```bash
./target/release/der compile "你的需求描述"
./target/release/der run output.der
```

### 修改现有程序
```bash
claude -p "@program.der @program.ders @CLAUDE.md 修改要求"
./target/release/der run modified_program.der
```

### 程序调试
```bash
claude -p "@broken.der @broken.ders @CLAUDE.md 错误描述和修复要求"
./target/release/der run fixed_program.der
```

### 性能优化
```bash
claude -p "@slow.der @slow.ders @CLAUDE.md 请优化性能"
time ./target/release/der run optimized.der
```

## 下一步

恭喜！你已经掌握了DER的基本用法。现在你可以：

1. **探索高级特性**: 查看 `examples/` 目录下的更多示例
2. **阅读设计文档**: 了解DER的设计哲学 `docs/`
3. **贡献代码**: DER是开源项目，欢迎贡献
4. **创建自己的AI工具**: 基于DER构建AI编程助手

## 故障排除

### 常见问题

**Q: AI生成的程序无法运行？**
A: 检查 `.ders` 文件中的AI推理，通常可以发现问题所在。

**Q: 如何让AI生成更好的代码？**
A: 提供更详细的需求描述，包括性能要求、错误处理等。

**Q: 程序运行很慢？**
A: 让AI优化程序，或检查算法复杂度。

**Q: AI无法理解我的修改要求？**
A: 确保 `CLAUDE.md` 规则文件是最新的，并提供具体的例子。

记住：DER是AI-Native语言，拥抱AI协作是关键！🚀