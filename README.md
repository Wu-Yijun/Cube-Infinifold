# Cube Infinifold

This is a puzzle and indie game. The player leads the character through mazes of optical illusions and impossible objects while rotating the world to reach various platforms.

# 🚧 <span style="color: red;">WORK IN PROGRESS</span>

! [@important](@line) 这是继承自 ! [@file](https://Wu-Yijun.github.io/articles/CubeInfinifold/README.md) 的文件，是游戏开发过程的忠实记录，也是学习各种程序设计方法的笔记。

## 程序草稿

### 界面逻辑

在渲染循环内，直接根据当前页面记录，使用 if else 切换到对应的页面渲染函数即可。

渲染器由两部分组成，页面渲染器和OpenGL渲染器。
页面渲染原则为，获取当前的页面 `self.my_view: MyView` ，调用它的 paint 函数绘制。

```Rust
pub enum MyView {
    MyMenu(menu::MyMenu),
    MyLogo(cube_infinifold_logo::MyInfinifoldLogo),
    None,
}
pub trait MyViewImpl {
    fn destory(&mut self);
    fn paint(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn to_change(&self) -> Option<String>;
}
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    match &mut self.my_view {
        MyView::MyMenu(v) => {
            v.paint(ctx, frame);
            if let Some(aim) = v.to_change() {
                self.change_to(aim, ctx);
            }
        }
        ...
    }
}
```

而对于一个页面，其绘制时，先绘制 opengl 作为底层，再在之上使用透明背景绘制UI。

```Rust
fn paint(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    let gl_layer = egui::containers::Frame {
        fill: egui::Color32::WHITE,
        ..Default::default()
    };
    let layout_layers = egui::containers::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        ..Default::default()
    };
    egui::CentralPanel::default()
        .frame(gl_layer)
        .show(ctx, |ui| {
            self.paint_opengl(ui);
        });
    egui::CentralPanel::default()
        .frame(layout_layers)
        .show(ctx, |ui| {
            // paint ui
        });
    ctx.input(|k| {
        // process input
    });
    ctx.request_repaint();
}
```

对应的绘制 opengl 的函数如下，先分配区域，再获取角度，最后通过 `self.game_view: Arc<Mutex<GLGameView>>` 绘制Opengl
```Rust
fn paint_opengl(&mut self, ui: &mut egui::Ui) {
    let (rect, response) = ui.allocate_exact_size(ui.max_rect().size(), egui::Sense::drag());
    self.angle += response.drag_delta().x * 0.01;
    let angle = self.angle;
    let game_view = self.game_view.clone();
    let callback = egui::PaintCallback {
        rect,
        callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
            game_view.lock().paint(painter.gl(), &rect, angle);
        })),
    };
    ui.painter().add(callback);
}
```
而 GLGameView 是包含在 MyGLView 且 impl 过 GLGameBase 的，因此可以调用它的 paint 函数。
```Rust
pub struct MyGLView {
    pub basic: Arc<Mutex<GLGameView>>,
    pub lines: Arc<Mutex<GLLinesView>>,
}
pub trait GLGameBase {
    fn new(gl: &glow::Context) -> Self;
    fn destroy(&self, gl: &glow::Context);
    fn paint(&self, gl: &glow::Context, rect: &egui::Rect, angle: f32);
}
```

具体Opengl如何初始化如何绘制参见下一节。

### OpenGl 
我们从更为完整的 GLLinesView 分析。它的实现包含一下几个函数
```Rust
pub struct GLLinesView {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    lines: Vec<items::Line>,
    musk_enabled: bool,
}
impl GLLinesView {
    pub fn set_lines(&mut self, line_vec: Vec<items::Line>) {
        self.lines = line_vec;
    }
    pub fn add_line(&mut self, line: items::Line) {
        self.lines.push(line);
    }
    pub fn add_lines(&mut self, mut lines: Vec<items::Line>) {
        self.lines.append(&mut lines);
    }
    pub fn set_musk_enabled(&mut self, musk: bool) {
        self.musk_enabled = musk;
    }
}
```
在初始化的过程中，加载编译着色器，添加顶点缓冲，保存在 `self.program` 和 `self.vertex_array` 中。
然后我们通过 `set_lines/add_line(s)` 来控制要绘制的直线。
最终在 paint 中将每一条直线绘制出来。

直线元素包含如下要素：起点、终点、遮罩、着色。起点和终点为两个向量，遮罩（目前为止）为一个点和一个方向组成的射线，着色最为复杂，分为默认（黑色）、纯色、顶点颜色、颜色函数。同时，着色 `Colored` 还实现了 Get 方法用于获取顶点颜色。
```Rust
pub struct Line {
    pub pos1: V3,
    pub pos2: V3,
    pub msk: Option<Musk>,
    pub color: Colored,
}
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Musk {
    pub pos: V3,
    pub dir: V3,
}
pub enum Colored {
    Default,
    Pure(Color),
    Vertex(Vec<Color>),
    Fun(Arc<dyn Fn(usize) -> Color>),
}
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colored {
    pub fn get(&self, id: usize) -> Color {
        ...
    }
}
```

### 游戏原理

接下来我准备创建 MyView/GameView
考虑到 game 可能内容较多，因此我创建了一个 game 文件夹

### 关卡程序结构

我准备将每一个独立的关卡作为一个 dll ，通过 json 控制访问

#### 生成和使用动态链接

因为我们允许自己编写关卡，因而在程序编译时关卡是未知的，因此需要主程序与关卡的动态交互。

一种方式是使用静态关卡，各种信息静态地保存在预先设置好的结构中，然后主程序读取文件以加载逻辑。
这是一种可行的方案，但是我现在还没想好文件结构，以及各种状况带来的复杂性，使得编写这个交互脚本异常困难。

因此，我决定采取第二种方式，将关卡编译为动态链接库，通过特定程序接口来实现两者的交互。
这样我可以在更新后快速升级我的关卡，且可以通过代码实现复杂的行为。

链接有很多种，.dll .a .so .dylib .rlib 等等，但只有动态链接的才可以在编译后由程序控制加载。

加载 .dll 的库经过调查，我认为比较好用的是 `libloading` .

具体方法如下:
对于创建库的代码, 简单使用 cargo new NAME --lib 可以生成一个简单的库项目, 我们在 Cargo.toml 中添加多个类似于下面的项目,就可以在 build 时生成多个动态链接库
```TOML
[lib]
# 必选, Rust 动态链接库
crate-type = ["dylib"]
# 输出的库名称(文件名)
name = "OUTPUT_NAME"
# 可选, 文件入口路径
path = "src/NAME.rs"
```

在代码中, 我们通过如下的方法标记导出 C 风格函数, 其中 `no_mangle` 标记表示不改变函数名称, 不然我们无法用名称索引获取函数. 在另一个文件中, 调用此函数的方法如下, 我们先加载这个动态链接库, 然后再显式地通过类型调用 `get` 函数, 返回结果是一个包裹内的函数, 可以直接通过括号调用. 当然, 数据类型也可以是结构体, 只不过要改成 C 可兼容的类型, 因此要使用 `repr(C)` 来标记 C 风格, 结构体的函数类型也可以通过 `extern "C"` 标记.
```Rust
//----- lib.rs -----
#[no_mangle]
pub extern "C" fn NAME(var:type,...) -> type{
    // codes here
}
// type define
#[repr(C)]
pub struct Stru {
    pub num: i32,
    pub fun: extern "C" fn(i32) -> bool,
}

//----- main.rs -----
fn unsafe main(){
    let lib = libloading::Library::new("path/to/LIBNAME.lib").unwrap();
    let fun: libloading::Symbol<extern "C" fn(type,...) -> type> = lib.get(b"NAME\0").unwrap();
    // call by: fun(var,...)
}
```

但是, 我们既然两边都是 Rust , 为何不能直接使用 Rust 风格, 而非要找 C 作为中间人呢?
当然可以!
我们只需要将 `extern "C"` 替换为 `extern "Rust"` 就可以表示这个是 Rust 风格的, 此外 `extern "Rust"` 是作为默认值存在, 可以省略. 因此, 我们可以通过下面的方法导出 Rust 类型的函数和全局变量. 将 `Symbol` 的类型直接设定为函数类型, 就可以获取到函数, 将它的类型设定为 `*mut Type` 就可以获取到 `Type` 类型的变量了.
```Rust
//----- lib.rs -----
// export function
#[no_mangle]
pub fn NAME(var:type,...) -> type{
    // codes here
}
// export static mut
#[no_mangle]
static mut NAME_VAR: type = INITIALIZER;

//----- main.rs -----
fn unsafe main(){
    let lib = libloading::Library::new("path/to/LIBNAME.lib").unwrap();
    let fun: libloading::Symbol<fn(type,...) -> type> = lib.get(b"NAME\0").unwrap();
    // call by: fun(var,...)
    let var: libloading::Symbol<*mut type> = lib.get(b"NAME_VAR\0").unwrap();
    // get (_: *mut type) by applying: *var
    // and get (_: type) by applying: **var
}
```

