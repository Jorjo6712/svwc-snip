use iced::{
    alignment::{
        self,
        Horizontal
    },
    widget::{
        button,
        container,
        row,
        text,
        pick_list
    },
    window,
    Application,
    Command,
    Element,
    Error,
    Font,
    Length,
    Settings,
    Size,
    Subscription,
    Theme,
};


pub struct App {
    new_screenshot: bool,
    save_screenshot: bool,
    copy_screenshot: bool,
    selected_option: Option,
}

pub struct Flags {
    pub window_size: Size,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            window_size: Size {
                width: 500.0,
                height: 45.0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NewScreenshot,
    SaveScreenshot,
    CopyScreenshot,
    SelectOption(Option),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Option {
    Rectangle,
    Window,
    Fullscreen,
}

impl std::fmt::Display for Option {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Option::Rectangle => write!(f, "Rectangle"),
            Option::Window => write!(f, "Window"),
            Option::Fullscreen => write!(f, "Full screen"),
        }
    }
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = Flags;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                new_screenshot: false,
                save_screenshot: false,
                copy_screenshot: false,
                selected_option: Option::Rectangle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("svwc-snip")
    }

    fn theme(&self) -> Self::Theme {
        Theme::TokyoNight
    }

    fn view(&self) -> Element<Self::Message> {
        let options = vec![Option::Rectangle, Option::Window, Option::Fullscreen];
       
        // This container holds the option pick list and new screen shot button

        let left_container = container(
            row![
                pick_list(
                    options,
                    Some(self.selected_option.clone()),
                    Message::SelectOption,
                )
                .width(115), 

                button(container(new_icon()).center_x()).on_press(Message::NewScreenshot)
                .width(50)
            ]
            .spacing(10),
        )
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Left);

        let right_container = container(
            row![
                button(container(save_icon()).center_x()).on_press(Message::SaveScreenshot)
                .width(50),
                button(container(copy_icon()).center_x()).on_press(Message::CopyScreenshot)
                .width(50),
            ]
            .spacing(7.5),
        )
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Right);

        container(
            row![left_container, right_container].spacing(10),
        )
        .padding(6.5)
        .width(Length::Fill)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NewScreenshot => {}
            Message::SaveScreenshot => {}
            Message::CopyScreenshot => {}
            Message::SelectOption(option) => {
                self.selected_option = option;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}


fn new_icon<'a>() -> Element<'a, Message> {
    icon('\u{E802}')
}

fn copy_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

fn save_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

// Icons made with a custom font set to contain mainly icons, which are referenced by unicodes
fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("svwc-icons");

    text(codepoint).font(ICON_FONT).into()
}

/*  
* Custom run function for running specific window setttings & custom font.
* Preferably the window should stay as `resizable: false`
* This is because some WMs will force the window to resize 
* to a tiled state.
*/
pub fn run() -> Result<(), Error> {
    let flags = Flags::default();

    let settings = Settings {
        window: window::Settings {
            size: flags.window_size,
            resizable: false,
            position: window::Position::Centered,
            ..Default::default()
        },
        fonts: vec![include_bytes!("../fonts/svwc-icons.ttf").as_slice().into()],
        ..Settings::default()
    };

    App::run(settings)
}
