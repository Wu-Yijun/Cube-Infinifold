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