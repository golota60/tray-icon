use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};

use iced::{
    executor,
    widget::{button, column, text},
    Application, Command, Element, Settings, Theme,
};

pub struct Counter {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn view(&self) -> Element<Message> {
        column![
            button("Increase").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrease").on_press(Message::DecrementPressed),
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        (Counter { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("tray-icon/iced example")
    }
}

fn main() -> iced::Result {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
    let icon = load_icon(std::path::Path::new(path));

    // Linux needs additional gtk directives for system tray to work
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let tray_menu = Menu::new();

    let quit_i = MenuItem::new("Quit", true, None);
    let quit_id = quit_i.id().to_owned();
    tray_menu
        .append_items(&[
            &MenuItem::new("show", true, None),
            &PredefinedMenuItem::separator(),
            &quit_i,
        ])
        .unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_icon(icon)
        .build()
        .unwrap();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    std::thread::spawn(move || loop {
        if let Ok(event) = tray_channel.try_recv() {
            println!("tray event: {:?}", event);
        }

        if let Ok(event) = menu_channel.try_recv() {
            println!("menu event: {:?}", event);
            if event.id == quit_id {
                std::process::exit(1);
            }
        }

        #[cfg(target_os = "linux")]
        gtk::main_iteration_do(false);
        std::thread::sleep(std::time::Duration::from_millis(500));
    });

    Counter::run(Settings::default())
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
