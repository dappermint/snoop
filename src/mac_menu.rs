//! Native macOS application menu bar (File, Edit, View, Playback, Window, Help).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuCommand {
    PlayPause,
    Next,
    Previous,
    SeekForward,
    SeekBackward,
    ToggleShuffle,
    CycleRepeat,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    Home,
    Search,
    LikedSongs,
    Queue,
    Settings,
    Shortcuts,
    Back,
    Forward,
    OpenRepo,
}

#[cfg(not(target_os = "macos"))]
pub fn init() {}

#[cfg(not(target_os = "macos"))]
pub fn set_waker(_wake: impl Fn() + Send + Sync + 'static) {}

#[cfg(not(target_os = "macos"))]
pub fn drain_commands() -> Vec<MenuCommand> {
    Vec::new()
}

#[cfg(target_os = "macos")]
pub use mac_impl::*;

#[cfg(target_os = "macos")]
mod mac_impl {
    use std::sync::Mutex;
    use objc2::rc::Retained;
    use objc2::runtime::Sel;
    use objc2::{declare_class, mutability, sel, ClassType, DeclaredClass};
    use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem, NSApplication};
    use objc2_foundation::{ns_string, MainThreadMarker, NSObject, NSString};

    use super::MenuCommand;

    static COMMANDS: Mutex<Vec<MenuCommand>> = Mutex::new(Vec::new());
    static WAKER: Mutex<Option<Box<dyn Fn() + Send + Sync>>> = Mutex::new(None);

    pub fn set_waker(wake: impl Fn() + Send + Sync + 'static) {
        if let Ok(mut w) = WAKER.lock() {
            *w = Some(Box::new(wake));
        }
    }

    fn push_command(cmd: MenuCommand) {
        if let Ok(mut list) = COMMANDS.lock() {
            list.push(cmd);
        }
        if let Ok(w) = WAKER.lock() {
            if let Some(wake) = w.as_ref() {
                wake();
            }
        }
    }

    pub fn drain_commands() -> Vec<MenuCommand> {
        if let Ok(mut list) = COMMANDS.lock() {
            std::mem::take(&mut *list)
        } else {
            Vec::new()
        }
    }

    declare_class!(
        pub struct SnoopMenuHandler;

        unsafe impl ClassType for SnoopMenuHandler {
            type Super = NSObject;
            type Mutability = mutability::MainThreadOnly;
            const NAME: &'static str = "SnoopMenuHandler";
        }

        impl DeclaredClass for SnoopMenuHandler {}

        unsafe impl SnoopMenuHandler {
            #[method(openSettings:)]
            fn open_settings(&self, _sender: &NSObject) {
                push_command(MenuCommand::Settings);
            }

            #[method(playPause:)]
            fn play_pause(&self, _sender: &NSObject) {
                push_command(MenuCommand::PlayPause);
            }

            #[method(nextTrack:)]
            fn next_track(&self, _sender: &NSObject) {
                push_command(MenuCommand::Next);
            }

            #[method(previousTrack:)]
            fn previous_track(&self, _sender: &NSObject) {
                push_command(MenuCommand::Previous);
            }

            #[method(seekForward:)]
            fn seek_forward(&self, _sender: &NSObject) {
                push_command(MenuCommand::SeekForward);
            }

            #[method(seekBackward:)]
            fn seek_backward(&self, _sender: &NSObject) {
                push_command(MenuCommand::SeekBackward);
            }

            #[method(toggleShuffle:)]
            fn toggle_shuffle(&self, _sender: &NSObject) {
                push_command(MenuCommand::ToggleShuffle);
            }

            #[method(cycleRepeat:)]
            fn cycle_repeat(&self, _sender: &NSObject) {
                push_command(MenuCommand::CycleRepeat);
            }

            #[method(volumeUp:)]
            fn volume_up(&self, _sender: &NSObject) {
                push_command(MenuCommand::VolumeUp);
            }

            #[method(volumeDown:)]
            fn volume_down(&self, _sender: &NSObject) {
                push_command(MenuCommand::VolumeDown);
            }

            #[method(toggleMute:)]
            fn toggle_mute(&self, _sender: &NSObject) {
                push_command(MenuCommand::ToggleMute);
            }

            #[method(openHome:)]
            fn open_home(&self, _sender: &NSObject) {
                push_command(MenuCommand::Home);
            }

            #[method(focusSearch:)]
            fn focus_search(&self, _sender: &NSObject) {
                push_command(MenuCommand::Search);
            }

            #[method(openLikedSongs:)]
            fn open_liked_songs(&self, _sender: &NSObject) {
                push_command(MenuCommand::LikedSongs);
            }

            #[method(toggleQueue:)]
            fn toggle_queue(&self, _sender: &NSObject) {
                push_command(MenuCommand::Queue);
            }

            #[method(goBack:)]
            fn go_back(&self, _sender: &NSObject) {
                push_command(MenuCommand::Back);
            }

            #[method(goForward:)]
            fn go_forward(&self, _sender: &NSObject) {
                push_command(MenuCommand::Forward);
            }

            #[method(showShortcuts:)]
            fn show_shortcuts(&self, _sender: &NSObject) {
                push_command(MenuCommand::Shortcuts);
            }

            #[method(openRepo:)]
            fn open_repo(&self, _sender: &NSObject) {
                push_command(MenuCommand::OpenRepo);
            }
        }
    );

    static mut HANDLER: Option<Retained<SnoopMenuHandler>> = None;

    fn create_item(
        mtm: MainThreadMarker,
        title: &NSString,
        action: Option<Sel>,
        key: &NSString,
        masks: Option<NSEventModifierFlags>,
        target: Option<&NSObject>,
    ) -> Retained<NSMenuItem> {
        let item = unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(
                mtm.alloc(),
                title,
                action,
                key,
            )
        };
        if let Some(masks) = masks {
            item.setKeyEquivalentModifierMask(masks);
        }
        if let Some(target) = target {
            unsafe { item.setTarget(Some(target)) };
        }
        item
    }

    fn create_menu(
        mtm: MainThreadMarker,
        title: &NSString,
    ) -> (Retained<NSMenuItem>, Retained<NSMenu>) {
        let container_item = unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(
                mtm.alloc(),
                title,
                None,
                ns_string!(""),
            )
        };
        let menu = unsafe { NSMenu::initWithTitle(mtm.alloc(), title) };
        unsafe { menu.setAutoenablesItems(false); }
        container_item.setSubmenu(Some(&menu));
        (container_item, menu)
    }

    pub fn init() {
        let Some(mtm) = MainThreadMarker::new() else { return };
        let app = NSApplication::sharedApplication(mtm);
        let Some(menubar) = (unsafe { app.mainMenu() }) else { return };

        static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if INITIALIZED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        let handler: Retained<SnoopMenuHandler> = unsafe {
            objc2::msg_send_id![mtm.alloc::<SnoopMenuHandler>(), init]
        };
        let target: &NSObject = &handler;
        unsafe {
            HANDLER = Some(handler.clone());
        }

        // 1. Settings item in app menu (first menu)
        if let Some(app_menu_item) = unsafe { menubar.itemAtIndex(0) } {
            if let Some(app_menu) = unsafe { app_menu_item.submenu() } {
                let settings_item = create_item(
                    mtm,
                    ns_string!("Settings…"),
                    Some(sel!(openSettings:)),
                    ns_string!(","),
                    None,
                    Some(target),
                );
                let sep = NSMenuItem::separatorItem(mtm);
                unsafe {
                    app_menu.insertItem_atIndex(&settings_item, 1);
                    app_menu.insertItem_atIndex(&sep, 2);
                }
            }
        }

        // 2. File menu
        let (file_item, file_menu) = create_menu(mtm, ns_string!("File"));
        file_menu.addItem(&create_item(
            mtm,
            ns_string!("Close Window"),
            Some(sel!(performClose:)),
            ns_string!("w"),
            None,
            None,
        ));
        menubar.addItem(&file_item);

        // 3. Edit menu
        let (edit_item, edit_menu) = create_menu(mtm, ns_string!("Edit"));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Undo"),
            Some(sel!(undo:)),
            ns_string!("z"),
            None,
            None,
        ));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Redo"),
            Some(sel!(redo:)),
            ns_string!("Z"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand | NSEventModifierFlags::NSEventModifierFlagShift),
            None,
        ));
        edit_menu.addItem(&NSMenuItem::separatorItem(mtm));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Cut"),
            Some(sel!(cut:)),
            ns_string!("x"),
            None,
            None,
        ));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Copy"),
            Some(sel!(copy:)),
            ns_string!("c"),
            None,
            None,
        ));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Paste"),
            Some(sel!(paste:)),
            ns_string!("v"),
            None,
            None,
        ));
        edit_menu.addItem(&create_item(
            mtm,
            ns_string!("Select All"),
            Some(sel!(selectAll:)),
            ns_string!("a"),
            None,
            None,
        ));
        menubar.addItem(&edit_item);

        // 4. Playback menu
        let (playback_item, playback_menu) = create_menu(mtm, ns_string!("Playback"));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Play / Pause"),
            Some(sel!(playPause:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Next Track"),
            Some(sel!(nextTrack:)),
            &NSString::from_str("\u{F703}"), // Right arrow
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Previous Track"),
            Some(sel!(previousTrack:)),
            &NSString::from_str("\u{F702}"), // Left arrow
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Seek Forward (10s)"),
            Some(sel!(seekForward:)),
            &NSString::from_str("\u{F703}"),
            Some(NSEventModifierFlags::NSEventModifierFlagShift),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Seek Backward (10s)"),
            Some(sel!(seekBackward:)),
            &NSString::from_str("\u{F702}"),
            Some(NSEventModifierFlags::NSEventModifierFlagShift),
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Shuffle"),
            Some(sel!(toggleShuffle:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Repeat"),
            Some(sel!(cycleRepeat:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Increase Volume"),
            Some(sel!(volumeUp:)),
            &NSString::from_str("\u{F700}"), // Up arrow
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Decrease Volume"),
            Some(sel!(volumeDown:)),
            &NSString::from_str("\u{F701}"), // Down arrow
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            ns_string!("Mute"),
            Some(sel!(toggleMute:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        menubar.addItem(&playback_item);

        // 5. View menu
        let (view_item, view_menu) = create_menu(mtm, ns_string!("View"));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Back"),
            Some(sel!(goBack:)),
            ns_string!("["),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Forward"),
            Some(sel!(goForward:)),
            ns_string!("]"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        view_menu.addItem(&NSMenuItem::separatorItem(mtm));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Home"),
            Some(sel!(openHome:)),
            ns_string!("h"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Search"),
            Some(sel!(focusSearch:)),
            ns_string!("f"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Liked Songs"),
            Some(sel!(openLikedSongs:)),
            ns_string!("l"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Queue"),
            Some(sel!(toggleQueue:)),
            ns_string!("Q"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand | NSEventModifierFlags::NSEventModifierFlagShift),
            Some(target),
        ));
        view_menu.addItem(&NSMenuItem::separatorItem(mtm));
        view_menu.addItem(&create_item(
            mtm,
            ns_string!("Toggle Full Screen"),
            Some(sel!(toggleFullScreen:)),
            ns_string!("f"),
            Some(NSEventModifierFlags::NSEventModifierFlagControl | NSEventModifierFlags::NSEventModifierFlagCommand),
            None,
        ));
        menubar.addItem(&view_item);

        // 6. Window menu
        let (window_item, window_menu) = create_menu(mtm, ns_string!("Window"));
        window_menu.addItem(&create_item(
            mtm,
            ns_string!("Minimize"),
            Some(sel!(performMiniaturize:)),
            ns_string!("m"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            None,
        ));
        window_menu.addItem(&create_item(
            mtm,
            ns_string!("Zoom"),
            Some(sel!(performZoom:)),
            ns_string!(""),
            None,
            None,
        ));
        window_menu.addItem(&NSMenuItem::separatorItem(mtm));
        window_menu.addItem(&create_item(
            mtm,
            ns_string!("Bring All to Front"),
            Some(sel!(arrangeInFront:)),
            ns_string!(""),
            None,
            None,
        ));
        menubar.addItem(&window_item);

        // 7. Help menu
        let (help_item, help_menu) = create_menu(mtm, ns_string!("Help"));
        help_menu.addItem(&create_item(
            mtm,
            ns_string!("Keyboard Shortcuts"),
            Some(sel!(showShortcuts:)),
            ns_string!("/"),
            Some(NSEventModifierFlags::NSEventModifierFlagCommand),
            Some(target),
        ));
        help_menu.addItem(&create_item(
            mtm,
            ns_string!("Snoop on GitHub"),
            Some(sel!(openRepo:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        menubar.addItem(&help_item);
    }
}
