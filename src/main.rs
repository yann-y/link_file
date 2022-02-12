use iced::{
    button, executor, scrollable, Application, Button, Clipboard, Command, Element, Row,
    Scrollable, Settings, Text,
};
// 用 include_bytes 如果路径错误，还会提示的
// const XQFONT: Font = Font::External {
//     name: "思源宋体",
//     // 导入字体文件，没有字体无法显示中文
//     bytes: include_bytes!("../source/SourceHanSerifCN-Bold.otf"),
// };
pub fn main() -> iced::Result {
    //let icon_rgb = vec![255,0,0,1];

    App::run(Settings {
        // window: window::Settings{
        //     icon: match window::icon::Icon::from_rgba(icon_rgb,10, 10){
        //         Ok(i) =>{Some(i)},
        //         Err(e) =>{
        //             println!("{}",&e);
        //             None
        //         },
        //     },
        //     ..Default::default()
        // },
        // 全局默认字体
        default_font: Some(include_bytes!("../source/font/SourceHanSerifCN-Bold.otf")),
        ..Settings::default()
    })
}
struct App {
    // 按钮初始状态状态
    button_state: button::State,
    text_str: String,
    // input_str: String,
    // input_state: text_input::State,

    // slider_value: i32,
    // slider_state: slider::State,

    // is_checked: bool,
    // pick_list_str: String,
    // pick_list_state: pick_list::State<String>,

    // radio_value: bool,
    scrollable_state: scrollable::State,
    about_button: button::State,
}
// App 的 message 类型
#[derive(Debug, Clone)]
enum AppMessage {
    Button,
    // TextInput(String),
    // TextInputSubmit,

    // Slider(i32),
    // SliderRelease,

    // CheckBox(bool),

    // PaneDragged(pane_grid::DragEvent),
    // PaneResized(pane_grid::ResizeEvent),
    // PaneClick(pane_grid::Pane),

    // PickList(String),

    // Radio(bool),
}
impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (
            App {
                button_state: button::State::new(),
                text_str: String::from("我是 Text"),
                // input_str: String::from(""),
                // input_state: text_input::State::new(),
                about_button: button::State::new(),
                // slider_value: 30,
                // slider_state: slider::State::new(),

                // is_checked: false,

                // pick_list_str: String::from(""),
                // pick_list_state: pick_list::State::default(),

                // radio_value: false,
                scrollable_state: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        // 窗口名称
        String::from("fileLink")
    }
    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            AppMessage::Button => {
                self.text_str = String::from("Touch Button(Text)");
            }
        }
        Command::none()
    }
    fn view(&mut self) -> Element<Self::Message> {
        // 顶部布局
        let about_button =
            Button::new(&mut self.about_button, Text::new("关于")).on_press(AppMessage::Button);
        let content_text = Text::new(&self.text_str);
        let chick_button =
            Button::new(&mut self.button_state, Text::new("迁移")).on_press(AppMessage::Button);

        let raw = Row::new().push(chick_button).push(about_button);
        // Scrollable  滚动视图
        Scrollable::new(&mut self.scrollable_state)
            .push(raw) // 添加控件
            .push(content_text)
            .into()
    }
}
// fn link_to_files(original: &str, link: &str) -> io::Result<()> {
//     //let original = "F:\\env\\protoc";   // 源目录
//     //let link = "C:\\Users\\yanfive\\protoc";  // 软连接地址

//     let metadata = files::metadata(original)?;
//     let file_type = metadata.file_type();
//     //match file_type.is_dir(){
//     // true => println!("yes"),
//     //   false => println!("no"),
//     // };
//     // a -> b

//     if file_type.is_dir() == true {
//         fs::symlink_dir(original, link)?;
//     };
//     Ok(())
// }
