# Wordle大作业报告

## 程序结构和说明

源文件结构：

```
├── all_state.rs
├── basic_function.rs
├── builtin_words.rs
├── calculate_average.rs
├── interactive_mode.rs
├── json_parse.rs
├── main.rs
├── solver.rs
├── test_mode.rs
└── tui.rs
```

- `main.rs`主程序，读取命令行参数，选择进入测试模式、交互模式或TUI模式
  - `opt: Opt`程序所有需要的命令行参数
- `basic_function.rs`wordle游戏中需要使用的数个最基本的函数，该部分函数或代码复用率较高，在其他文件中频繁使用
  - `pub enum LetterState`字母状态的枚举，包括`YRGX`四种
  - `pub struct KeyboardState`键盘状态，内部为`[LetterState; 26]`
  - `impl KeyboardState`键盘相关函数
    - `new()`创建空白`KeyboardState`对象
    - `update()`根据传入信息更新对象状态
    - `to_string()`将对象转换为`String`并用于输出
  - `pub struct GameData`总游戏相关数据，包括总胜场、总败场等
  - `impl GameData`主要为对`GameData`进行操作的函数，名称即为功能
  - `pub fn judge`根据答案以及猜词，返回`[LettetState; 5]`单词状态
  - `pub fn get_answer_word`获取答案单词，包括随机模式等，都在此函数内
  - `pub fn input_guess`获取猜测单词输入，并判断是否有效，无效需要重新输入
  - `pub fn get_acceptabel_set`获取候选词库，包括从文件中读取
  - `pub fn get_final_set`获取最终词库，包括文件中读取并判断是否满足条件并`panic`
- `test_mode.rs`测试模式主程序，在非tty情况下进入，主要用于自动测试
  - `pub fn test_mode`测试模式主函数，严格按照自动测试要求输入与输出
- `interactive_mode.rs`交互模式主程序，进入交互模式并输出彩色字符
  - `pun fn interactive_mode`测试模式主程序，输出彩色字符
  - `fn keyboard_to_color`参数为`KeyboardState`，输出彩色键盘状态
- `json_parse.rs`主要处理json文件读取与写入
  - `pub struct State` `pub struct Game`根据json结构构造的结构体，`State`中包含`Vec<Game>`
  - `impl State`
    - `pub fn load`从文件中加载并存入`State`中
    - `pun fn save`保存状态并写入json文件
    - `pun fn add_game`在结构体中添加游戏局数信息
  - `impl Game` `pub fn new`创建新的`Game`结构体
  - `pub fn parse_config`解析`config.json`并对命令行参数结构体`opt: Opt`进行修改
- `tui.rs`该文件部分代码仿照[官方文档](https://github.com/fdehau/tui-rs)示例代码并进行修改，实现了在tui上展示wordle的基础操作  
  - `enum InputMode`输入框状态，包括普通模式与编辑模式
  - `enum MessageMode`对外显示信息状态，包括合法输入、不合法输入、胜利、失败、输入、输入答案
  - `pub struct App`包含需要在tui上显示的各种信息
  - `pub fn tui`tui主函数，包括进入替代终端、隐藏光标等操作
  - `fn run_app`根据`struct App`信息运行app
  - `fn ui`创建tui界面，构造数个`widget`并进行`render`操作
  - `pun fn check_guess`检查用户输入答案是否合法，不合法随机单词作为答案
  - `fn app_to_string`将`app.message`的状态转为用户可见字符串
  - `fn word_to_spans`将单词转为`Spans`(`Spans`为tui中的彩色`widget`，若使用`colored`则不兼容)
  - `fn keyboard_to_spans`将键盘状态转为`Vec<Spans>`，一行为一个`Spans`
  - `fn char_to_span` 将字符转为`Span`，主要用于以上两个函数
- `solver.rs`求解器，根据已知信息给出最大熵单词的提示
  - `pub fn inter_solver`交互式求解器，类似于WordleServer
  - `pub fn solver` 普通求解器，在交互模式中给出提示词
  - `pub fn filter`筛选单词
  - `fn get_entropy`获取熵
- `calculate_average.rs`主要用于求解器最后一个步骤的运算，使用`rayon`进行加速，计算平均次数与最佳起始词
- `all_state.rs`该功能与`bultin_words.rs`作用类似，为计算熵提供了遍历对象`ALL_STATE`

## 程序主要功能

### 交互模式

![image](https://user-images.githubusercontent.com/92924106/186917396-ac20e601-e350-4fe5-bc9c-f89b425943b9.png)


有相应的输入提示及彩色显示

### 求解器

![image](https://user-images.githubusercontent.com/92924106/186917443-81a407b5-ff0b-4aa0-b9f9-b53177590d3c.png)


根据视频算法对`ACCEPTABLE`全局单词进行熵运算，成功率为100%，平均猜测次数为四次左右，且运算时间快，首轮使用约不到10秒

### TUI

实现了较为完整的TUI-Wordle界面

![image](https://user-images.githubusercontent.com/92924106/186917525-e60a63bb-d984-476a-92a3-9a9b9031050c.png)

### 交互式求解器

![image](https://user-images.githubusercontent.com/92924106/186917544-3d0305d8-c06a-4a84-8d28-3123354a198a.png)

完全类似于wordle-server的功能

## 提高要求实现方式

- 求解器使用了`rayon`进行并行运算，并使用`indicatif`作为进度条
- TUI使用了`crossterm`创建新终端并进行彩色输出

## 感想

rust社区是非常友好的社区，我在stackoverflow上提问都有很多人立马回复，助教也是有问必答，非常迅速。并且大部分文档都很详尽，每个函数都有markdown样式的介绍和使用例子。少部分不详细的库(~~tui~~)也有GitHub示例代码可供摸索。

刚开始写代码的时候一定做好文件分区以及函数解耦合，否则在完成基础任务重构的时候会非常耗时间，代码也无法得到重用
