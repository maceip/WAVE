use crate::{
    fluent_icon::FluentIcon,
    page::{page, widget_example},
    style,
    widget::{button, ribbon, text, Checkbox, Element, Radio},
};

use iced::widget::{column, row, Container};
use iced::Length;

// ---------------------------------------------------------------------------
// State enums mirroring Aurora's RibbonState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Task {
    #[default]
    PageLayout,
    Write,
    Animations,
    ContextualA,
    ContextualB,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DocumentSaveLocation {
    #[default]
    Local,
    Remote,
    Saved,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Presentation {
    #[default]
    Comfortable,
    Cozy,
    Compact,
}

// ---------------------------------------------------------------------------
// Page state
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct Ribbon {
    // Task switching
    selected_task: Task,

    // Clipboard split button
    paste_flyout_open: bool,

    // Font formatting toggles (FlowRibbonBand equivalent)
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,

    // Alignment (mutually exclusive)
    align_left: bool,
    align_center: bool,
    align_right: bool,

    // Document band
    save_location: DocumentSaveLocation,

    // Show/Hide band (Write task)
    show_ruler: bool,
    show_gridlines: bool,
    show_document_map: bool,

    // Presentation band (Write task)
    presentation: Presentation,

    // Contextual task group visibility
    contextual_group_visible: bool,

    // Feedback
    last_action: Option<&'static str>,
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Message {
    // Task switching
    TaskSelected(Task),

    // Clipboard band
    PastePressed,
    PasteFlyoutOpened,
    PasteFlyoutClosed,
    CutPressed,
    CopyPressed,
    FormatPressed,

    // Font band
    ToggleBold,
    ToggleItalic,
    ToggleUnderline,
    ToggleStrikethrough,
    AlignLeftPressed,
    AlignCenterPressed,
    AlignRightPressed,
    FontIncreasePressed,
    FontDecreasePressed,

    // Document band
    SaveLocationChanged(DocumentSaveLocation),
    DocumentNewPressed,
    DocumentOpenPressed,
    #[allow(dead_code)]
    DocumentSavePressed,
    DocumentPrintPressed,

    // Find band
    SearchPressed,
    FindPressed,
    FindReplacePressed,
    SelectAllPressed,

    // Action band (Write task)
    AddressBookPressed,
    DocumentActionPressed,
    AppointmentPressed,
    BookmarkPressed,
    ContactPressed,

    // Preferences band (Write task)
    AccessibilityPressed,
    FontPrefPressed,
    ThemesPressed,

    // Show/Hide band (Write task)
    ToggleRuler,
    ToggleGridlines,
    ToggleDocumentMap,

    // Presentation band (Write task)
    PresentationChanged(Presentation),

    // Contextual group
    ToggleContextualGroup,
}

impl Message {
    fn action_label(&self) -> Option<&'static str> {
        match self {
            Message::PastePressed => Some("Paste"),
            Message::CutPressed => Some("Cut"),
            Message::CopyPressed => Some("Copy"),
            Message::FormatPressed => Some("Format"),
            Message::ToggleBold => Some("Bold toggled"),
            Message::ToggleItalic => Some("Italic toggled"),
            Message::ToggleUnderline => Some("Underline toggled"),
            Message::ToggleStrikethrough => Some("Strikethrough toggled"),
            Message::AlignLeftPressed => Some("Align Left"),
            Message::AlignCenterPressed => Some("Align Center"),
            Message::AlignRightPressed => Some("Align Right"),
            Message::FontIncreasePressed => Some("Font Increase"),
            Message::FontDecreasePressed => Some("Font Decrease"),
            Message::DocumentNewPressed => Some("New Document"),
            Message::DocumentOpenPressed => Some("Open Document"),
            Message::DocumentSavePressed => Some("Save Document"),
            Message::DocumentPrintPressed => Some("Print Document"),
            Message::SearchPressed => Some("Search"),
            Message::FindPressed => Some("Find"),
            Message::FindReplacePressed => Some("Find & Replace"),
            Message::SelectAllPressed => Some("Select All"),
            Message::AddressBookPressed => Some("Address Book"),
            Message::DocumentActionPressed => Some("Document"),
            Message::AppointmentPressed => Some("Appointment"),
            Message::BookmarkPressed => Some("Bookmark"),
            Message::ContactPressed => Some("Contact"),
            Message::AccessibilityPressed => Some("Accessibility"),
            Message::FontPrefPressed => Some("Font Preferences"),
            Message::ThemesPressed => Some("Themes"),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

impl Ribbon {
    pub fn update(&mut self, message: Message) {
        if let Some(label) = message.action_label() {
            self.last_action = Some(label);
        }

        match message {
            Message::TaskSelected(task) => self.selected_task = task,

            // Clipboard
            Message::PastePressed => self.paste_flyout_open = false,
            Message::PasteFlyoutOpened => self.paste_flyout_open = true,
            Message::PasteFlyoutClosed => self.paste_flyout_open = false,
            Message::CutPressed | Message::CopyPressed | Message::FormatPressed => {
                self.paste_flyout_open = false;
            }

            // Font toggles
            Message::ToggleBold => self.bold = !self.bold,
            Message::ToggleItalic => self.italic = !self.italic,
            Message::ToggleUnderline => self.underline = !self.underline,
            Message::ToggleStrikethrough => self.strikethrough = !self.strikethrough,

            // Alignment (mutually exclusive)
            Message::AlignLeftPressed => {
                self.align_left = true;
                self.align_center = false;
                self.align_right = false;
            }
            Message::AlignCenterPressed => {
                self.align_left = false;
                self.align_center = true;
                self.align_right = false;
            }
            Message::AlignRightPressed => {
                self.align_left = false;
                self.align_center = false;
                self.align_right = true;
            }

            // Font size
            Message::FontIncreasePressed | Message::FontDecreasePressed => {}

            // Document band
            Message::SaveLocationChanged(loc) => self.save_location = loc,
            Message::DocumentNewPressed
            | Message::DocumentOpenPressed
            | Message::DocumentSavePressed
            | Message::DocumentPrintPressed => {}

            // Find band
            Message::SearchPressed
            | Message::FindPressed
            | Message::FindReplacePressed
            | Message::SelectAllPressed => {}

            // Action band
            Message::AddressBookPressed
            | Message::DocumentActionPressed
            | Message::AppointmentPressed
            | Message::BookmarkPressed
            | Message::ContactPressed => {}

            // Preferences band
            Message::AccessibilityPressed
            | Message::FontPrefPressed
            | Message::ThemesPressed => {}

            // Show/Hide
            Message::ToggleRuler => self.show_ruler = !self.show_ruler,
            Message::ToggleGridlines => self.show_gridlines = !self.show_gridlines,
            Message::ToggleDocumentMap => self.show_document_map = !self.show_document_map,

            // Presentation
            Message::PresentationChanged(p) => self.presentation = p,

            // Contextual
            Message::ToggleContextualGroup => {
                self.contextual_group_visible = !self.contextual_group_visible
            }
        }
    }

    // -----------------------------------------------------------------------
    // View
    // -----------------------------------------------------------------------

    pub fn view(&self) -> Element<Message> {
        // Build tab bar
        let tab_bar = self.build_tab_bar();

        // Build band content for active task
        let band_content = match self.selected_task {
            Task::PageLayout => self.build_page_layout_task(),
            Task::Write => self.build_write_task(),
            Task::Animations => self.build_animations_task(),
            Task::ContextualA | Task::ContextualB => self.build_contextual_task(),
        };

        // Assemble full ribbon
        let full_ribbon = ribbon::ribbon_bar::ribbon(tab_bar, band_content);

        // Contextual group toggle checkbox
        let contextual_toggle = Checkbox::new(
            "Show contextual tabs",
            self.contextual_group_visible,
        )
        .on_toggle(|_| Message::ToggleContextualGroup);

        let status_text = if let Some(action) = self.last_action {
            format!("Last action: {action}")
        } else {
            String::from("Click a ribbon button to see its action here.")
        };

        page(
            "Ribbon",
            [
                widget_example(
                    "A full ribbon component with tasks, bands, and adaptive controls.",
                    column![
                        full_ribbon,
                        Container::new(text::body1(status_text))
                            .padding([8, 12])
                            .width(Length::Fill),
                    ]
                    .spacing(8)
                    .width(Length::Fill),
                    Some(
                        column![contextual_toggle,]
                            .spacing(8)
                            .width(160),
                    ),
                    None::<Element<Message>>,
                ),
            ],
        )
    }

    // -----------------------------------------------------------------------
    // Tab bar
    // -----------------------------------------------------------------------

    fn build_tab_bar(&self) -> Element<Message> {
        let mut tabs: Vec<Element<Message>> = vec![
            ribbon::tab_bar::tab(
                "Page Layout",
                self.selected_task == Task::PageLayout,
                Message::TaskSelected(Task::PageLayout),
            ),
            ribbon::tab_bar::tab(
                "Write",
                self.selected_task == Task::Write,
                Message::TaskSelected(Task::Write),
            ),
            ribbon::tab_bar::tab(
                "Animations",
                self.selected_task == Task::Animations,
                Message::TaskSelected(Task::Animations),
            ),
        ];

        // Contextual task group tabs (colored)
        if self.contextual_group_visible {
            tabs.push(ribbon::tab_bar::contextual_tab(
                "Context A",
                iced::Color::from_rgb(0.9, 0.2, 0.2),
                self.selected_task == Task::ContextualA,
                Message::TaskSelected(Task::ContextualA),
            ));
            tabs.push(ribbon::tab_bar::contextual_tab(
                "Context B",
                iced::Color::from_rgb(0.2, 0.7, 0.3),
                self.selected_task == Task::ContextualB,
                Message::TaskSelected(Task::ContextualB),
            ));
        }

        ribbon::tab_bar::tab_bar(tabs)
    }

    // -----------------------------------------------------------------------
    // Page Layout task (Clipboard + Font + Document + Find)
    // -----------------------------------------------------------------------

    fn build_page_layout_task(&self) -> Element<Message> {
        let clipboard_band = self.build_clipboard_band();
        let font_band = self.build_font_band();
        let document_band = self.build_document_band();
        let find_band = self.build_find_band();

        ribbon::band::band_group(vec![clipboard_band, font_band, document_band, find_band])
    }

    // -----------------------------------------------------------------------
    // Clipboard band: Paste (Top), Cut/Copy/Format (Medium)
    // -----------------------------------------------------------------------

    fn build_clipboard_band(&self) -> Element<Message> {
        let paste_flyout = Container::new(column![
            button::menu_icon("Paste Special", FluentIcon::Paste)
                .on_press(Message::PastePressed),
            button::menu_icon("Paste as Text", FluentIcon::Paste)
                .on_press(Message::PastePressed),
        ])
        .style(style::container::overlay);

        let paste_btn = ribbon::split_button::large(
            "Paste",
            FluentIcon::Paste,
            paste_flyout,
            Message::PastePressed,
            Message::PasteFlyoutOpened,
            Message::PasteFlyoutClosed,
            self.paste_flyout_open,
        )
        .width(68);

        let medium_buttons = column![
            ribbon::button::medium("Cut", FluentIcon::Cut)
                .on_press(Message::CutPressed),
            ribbon::button::medium("Copy", FluentIcon::Copy)
                .on_press(Message::CopyPressed),
            ribbon::button::medium("Format", FluentIcon::Edit)
                .on_press(Message::FormatPressed),
        ]
        .spacing(0);

        ribbon::band::band(
            "Clipboard",
            row![paste_btn, medium_buttons,].spacing(4).height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Font band (FlowRibbonBand): formatting toggles + alignment + font size
    // -----------------------------------------------------------------------

    fn build_font_band(&self) -> Element<Message> {
        // Font style strip: Bold, Italic, Underline, Strikethrough
        let style_strip = ribbon::button_strip::horizontal(vec![
            ribbon::button_strip::icon_button(FluentIcon::Bold, self.bold)
                .on_press(Message::ToggleBold)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::Italic, self.italic)
                .on_press(Message::ToggleItalic)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::Underline, self.underline)
                .on_press(Message::ToggleUnderline)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::Strikethrough, self.strikethrough)
                .on_press(Message::ToggleStrikethrough)
                .into(),
        ]);

        // Alignment strip: Left, Center, Right
        let align_strip = ribbon::button_strip::horizontal(vec![
            ribbon::button_strip::icon_button(FluentIcon::AlignLeft, self.align_left)
                .on_press(Message::AlignLeftPressed)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::AlignCenter, self.align_center)
                .on_press(Message::AlignCenterPressed)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::AlignRight, self.align_right)
                .on_press(Message::AlignRightPressed)
                .into(),
        ]);

        // Font size strip: Decrease, Increase
        let size_strip = ribbon::button_strip::horizontal(vec![
            ribbon::button_strip::icon_button(FluentIcon::FontDecrease, false)
                .on_press(Message::FontDecreasePressed)
                .into(),
            ribbon::button_strip::icon_button(FluentIcon::FontIncrease, false)
                .on_press(Message::FontIncreasePressed)
                .into(),
        ]);

        ribbon::band::band(
            "Font",
            column![
                row![style_strip, size_strip,].spacing(8),
                align_strip,
            ]
            .spacing(4)
            .height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Document band: Save location toggles (Top) + operations (Medium)
    // -----------------------------------------------------------------------

    fn build_document_band(&self) -> Element<Message> {
        // Save location toggles (Top priority)
        let local_btn = ribbon::button::toggle_large(
            "Local",
            FluentIcon::Save,
            self.save_location == DocumentSaveLocation::Local,
        )
        .on_press(Message::SaveLocationChanged(DocumentSaveLocation::Local))
        .width(52);

        let remote_btn = ribbon::button::toggle_large(
            "Remote",
            FluentIcon::Share,
            self.save_location == DocumentSaveLocation::Remote,
        )
        .on_press(Message::SaveLocationChanged(DocumentSaveLocation::Remote))
        .width(56);

        let saved_btn = ribbon::button::toggle_large(
            "Saved",
            FluentIcon::Document,
            self.save_location == DocumentSaveLocation::Saved,
        )
        .on_press(Message::SaveLocationChanged(DocumentSaveLocation::Saved))
        .width(52);

        // Document operations (Medium priority)
        let ops = column![
            ribbon::button::medium("New", FluentIcon::Add)
                .on_press(Message::DocumentNewPressed),
            ribbon::button::medium("Open", FluentIcon::NewWindow)
                .on_press(Message::DocumentOpenPressed),
            ribbon::button::medium("Print", FluentIcon::Print)
                .on_press(Message::DocumentPrintPressed),
        ]
        .spacing(0);

        ribbon::band::band(
            "Document",
            row![local_btn, remote_btn, saved_btn, ops,]
                .spacing(2)
                .height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Find band: Search (Top), Find/Replace/SelectAll (Medium)
    // -----------------------------------------------------------------------

    fn build_find_band(&self) -> Element<Message> {
        let search_btn = ribbon::button::large("Search", FluentIcon::Search)
            .on_press(Message::SearchPressed)
            .width(52);

        let medium_cmds = column![
            ribbon::button::medium("Find", FluentIcon::Search)
                .on_press(Message::FindPressed),
            ribbon::button::medium("Replace", FluentIcon::Edit)
                .on_press(Message::FindReplacePressed),
            ribbon::button::medium("Select All", FluentIcon::SelectAll)
                .on_press(Message::SelectAllPressed),
        ]
        .spacing(0);

        ribbon::band::band(
            "Find",
            row![search_btn, medium_cmds,]
                .spacing(4)
                .height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Write task (Action + Preferences + Show/Hide + Presentation)
    // -----------------------------------------------------------------------

    fn build_write_task(&self) -> Element<Message> {
        let action_band = self.build_action_band();
        let preferences_band = self.build_preferences_band();
        let show_hide_band = self.build_show_hide_band();
        let presentation_band = self.build_presentation_band();

        ribbon::band::band_group(vec![
            action_band,
            preferences_band,
            show_hide_band,
            presentation_band,
        ])
    }

    // -----------------------------------------------------------------------
    // Action band: Address Book (Top) + Document/Appointment/Bookmark/Contact (Medium)
    // -----------------------------------------------------------------------

    fn build_action_band(&self) -> Element<Message> {
        let address_btn = ribbon::button::large("Address\nBook", FluentIcon::Mail)
            .on_press(Message::AddressBookPressed)
            .width(56);

        let medium_cmds = column![
            ribbon::button::medium("Document", FluentIcon::Document)
                .on_press(Message::DocumentActionPressed),
            ribbon::button::medium("Appointment", FluentIcon::Calendar)
                .on_press(Message::AppointmentPressed),
            ribbon::button::medium("Bookmark", FluentIcon::Like)
                .on_press(Message::BookmarkPressed),
        ]
        .spacing(0);

        let contact_btn = ribbon::button::large("Contact", FluentIcon::Group)
            .on_press(Message::ContactPressed)
            .width(52);

        ribbon::band::band(
            "Action",
            row![address_btn, medium_cmds, contact_btn,]
                .spacing(2)
                .height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Preferences band: Accessibility/Font/Themes (Medium)
    // -----------------------------------------------------------------------

    fn build_preferences_band(&self) -> Element<Message> {
        let font_btn = ribbon::button::large("Font", FluentIcon::Font)
            .on_press(Message::FontPrefPressed)
            .width(48);

        let medium_cmds = column![
            ribbon::button::medium("Accessibility", FluentIcon::Settings)
                .on_press(Message::AccessibilityPressed),
            ribbon::button::medium("Themes", FluentIcon::Color)
                .on_press(Message::ThemesPressed),
        ]
        .spacing(0);

        ribbon::band::band(
            "Preferences",
            row![font_btn, medium_cmds,]
                .spacing(4)
                .height(Length::Fill),
        )
    }

    // -----------------------------------------------------------------------
    // Show/Hide band: checkboxes (RibbonBandComponentGroup equivalent)
    // -----------------------------------------------------------------------

    fn build_show_hide_band(&self) -> Element<Message> {
        let ruler = Checkbox::new("Ruler", self.show_ruler)
            .on_toggle(|_| Message::ToggleRuler)
            .size(14)
            .spacing(4);

        let gridlines = Checkbox::new("Gridlines", self.show_gridlines)
            .on_toggle(|_| Message::ToggleGridlines)
            .size(14)
            .spacing(4);

        let doc_map = Checkbox::new("Doc Map", self.show_document_map)
            .on_toggle(|_| Message::ToggleDocumentMap)
            .size(14)
            .spacing(4);

        ribbon::band::band(
            "Show/Hide",
            column![ruler, gridlines, doc_map,]
                .spacing(4)
                .height(Length::Fill)
                .width(Length::Shrink),
        )
    }

    // -----------------------------------------------------------------------
    // Presentation band: radio buttons (mutually exclusive selectors)
    // -----------------------------------------------------------------------

    fn build_presentation_band(&self) -> Element<Message> {
        let comfortable: Radio<Message> = Radio::new(
            "Comfortable",
            Presentation::Comfortable,
            Some(self.presentation),
            Message::PresentationChanged,
        )
        .size(14)
        .spacing(4);

        let cozy: Radio<Message> = Radio::new(
            "Cozy",
            Presentation::Cozy,
            Some(self.presentation),
            Message::PresentationChanged,
        )
        .size(14)
        .spacing(4);

        let compact: Radio<Message> = Radio::new(
            "Compact",
            Presentation::Compact,
            Some(self.presentation),
            Message::PresentationChanged,
        )
        .size(14)
        .spacing(4);

        ribbon::band::band(
            "Presentation",
            column![comfortable, cozy, compact,]
                .spacing(4)
                .height(Length::Fill)
                .width(Length::Shrink),
        )
    }

    // -----------------------------------------------------------------------
    // Animations task (reuses Action band patterns)
    // -----------------------------------------------------------------------

    fn build_animations_task(&self) -> Element<Message> {
        // Simplified animations task with action and document bands
        let action_band = self.build_action_band();
        let document_band = self.build_document_band();

        ribbon::band::band_group(vec![action_band, document_band])
    }

    // -----------------------------------------------------------------------
    // Contextual task (shown when contextual group is visible)
    // -----------------------------------------------------------------------

    fn build_contextual_task(&self) -> Element<Message> {
        // Reuse action and preferences bands for contextual tasks
        let action_band = self.build_action_band();
        let preferences_band = self.build_preferences_band();

        ribbon::band::band_group(vec![action_band, preferences_band])
    }
}
