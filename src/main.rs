use iced::{Align, Column, Container, Element, Length, Sandbox, Settings, Text};
use iced_aw::{TabLabel, Tabs};
mod about;
use about::{AboutMessage, AboutTab};

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

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
    active_tab: usize,
    about_tab: AboutTab,
}
// App 的 message 类型
#[derive(Clone, Debug)]
enum AppMessage {
    TabSelected(usize),
    About(AboutMessage),
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
impl Sandbox for App {
    type Message = AppMessage;
    fn new() -> Self {
        App {
            active_tab: 0,
            about_tab: AboutTab::new(),
        }
    }

    fn title(&self) -> String {
        // 窗口名称
        String::from("link_file")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::TabSelected(selected) => self.active_tab = selected,
            AppMessage::About(_message) => self.about_tab.update(),
            // Message::Login(message) => self.login_tab.update(message),
            // Message::Ferris(message) => self.ferris_tab.update(message),
            // Message::Counter(message) => self.counter_tab.update(message),
            // Message::Settings(message) => self.settings_tab.update(message),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // let position = self
        //    // .settings_tab
        //     .settings()
        //     .tab_bar_position
        //     .unwrap_or_default();
        // let theme = self
        //    // .settings_tab
        //     .settings()
        //     .tab_bar_theme
        //     .unwrap_or_default();

        Tabs::new(self.active_tab, AppMessage::TabSelected)
            .push(self.about_tab.tab_label(), self.about_tab.view())
            // .push(self.about_tab.tab_label(), self.about_tab.view())
            // .push(self.ferris_tab.tab_label(), self.ferris_tab.view())
            // .push(self.counter_tab.tab_label(), self.counter_tab.view())
            // .push(self.settings_tab.tab_label(), self.settings_tab.view())
            // .tab_bar_style(theme)
            // .icon_font(ICON_FONT)
            // .tab_bar_position(match position {
            //     TabBarPosition::Top => iced_aw::TabBarPosition::Top,
            //     TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            // })
            .into()
    }
}
trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&mut self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&mut self) -> Element<'_, Self::Message>;
}
