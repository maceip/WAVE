use crate::{
    style,
    widget::{Container, Element, Quad},
};

use iced::Length;

/// Presentation priority determines how a command is displayed at different ribbon widths.
/// Top = large button (icon + text stacked), Medium = medium button (icon + text side-by-side),
/// Low = small button (icon only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PresentationPriority {
    Top,
    Medium,
    Low,
}

pub fn separator<'a, Message: 'a>() -> Element<'a, Message> {
    Container::new(
        Quad::new(None::<Element<Message>>)
            .width(1.0)
            .height(Length::Fill)
            .style(|theme, _status| {
                let palette = theme.palette();
                style::quad::Style {
                    background: Some(palette.divider_stroke_color_default.into()),
                    ..style::quad::Style::default()
                }
            }),
    )
    .padding([0, 4])
    .into()
}

/// Creates a horizontal separator line for use inside bands (between band rows).
#[allow(dead_code)]
pub fn horizontal_separator<'a, Message: 'a>() -> Element<'a, Message> {
    Container::new(
        Quad::new(None::<Element<Message>>)
            .width(Length::Fill)
            .height(1.0)
            .style(|theme, _status| {
                let palette = theme.palette();
                style::quad::Style {
                    background: Some(palette.divider_stroke_color_default.into()),
                    ..style::quad::Style::default()
                }
            }),
    )
    .padding([2, 0])
    .into()
}

pub mod button {

    use crate::{
        fluent_icon::FluentIcon,
        style,
        widget::{text, Button, Column, Row},
    };

    use iced::{
        alignment::{Horizontal, Vertical},
        widget::center,
        Length, Padding,
    };

    /// Large ribbon button: icon (28px) stacked above text label. 60x72 pixels.
    /// Corresponds to PresentationPriority::Top in the Aurora ribbon.
    pub fn large<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        Button::new(center(
            Column::new()
                .push(
                    text::icon(icon.codepoint())
                        .height(Length::FillPortion(1))
                        .size(28),
                )
                .push(text::caption1(label).height(Length::FillPortion(1)))
                .align_x(Horizontal::Center),
        ))
        .width(60)
        .height(72)
        .padding(0)
        .style(style::button::transparent)
    }

    /// Medium ribbon button: icon and text side-by-side. 24px height.
    /// Corresponds to PresentationPriority::Medium in the Aurora ribbon.
    pub fn medium<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        Button::new(
            Row::new()
                .push(text::icon(icon.codepoint()))
                .push(text::caption1(label))
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .spacing(4),
        )
        .width(Length::Shrink)
        .height(24)
        .padding(Padding::new(0.0).left(4).right(8))
        .style(style::button::transparent)
    }

    /// Small ribbon button: icon only. 24x24 pixels.
    /// Corresponds to PresentationPriority::Low in the Aurora ribbon.
    #[allow(dead_code)]
    pub fn small<'a, Message>(icon: FluentIcon) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        Button::new(center(text::icon(icon.codepoint())))
            .width(24)
            .height(24)
            .padding(4)
            .style(style::button::transparent)
    }

    /// Toggle variant of large button - shows selected state via accent styling.
    pub fn toggle_large<'a, Message>(
        label: &'a str,
        icon: FluentIcon,
        selected: bool,
    ) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        let btn = Button::new(center(
            Column::new()
                .push(
                    text::icon(icon.codepoint())
                        .height(Length::FillPortion(1))
                        .size(28),
                )
                .push(text::caption1(label).height(Length::FillPortion(1)))
                .align_x(Horizontal::Center),
        ))
        .width(60)
        .height(72)
        .padding(0);

        if selected {
            btn.style(style::button::secondary)
        } else {
            btn.style(style::button::transparent)
        }
    }

    /// Toggle variant of small button - shows selected state via accent styling.
    #[allow(dead_code)]
    pub fn toggle_small<'a, Message>(icon: FluentIcon, selected: bool) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        let btn = Button::new(center(text::icon(icon.codepoint())))
            .width(24)
            .height(24)
            .padding(4);

        if selected {
            btn.style(style::button::secondary)
        } else {
            btn.style(style::button::transparent)
        }
    }
}

pub mod split_button {

    use crate::{
        fluent_icon::FluentIcon,
        theme,
        widget::{text, Button, Column, DropDown, Element, Quad},
    };

    use iced::{
        alignment::{Horizontal, Vertical},
        border::Radius,
        widget::center,
        Border, Length, Shadow,
    };

    pub fn large<'a, Message>(
        label: &'a str,
        icon: FluentIcon,
        overlay: impl Into<Element<'a, Message>>,
        on_press: Message,
        on_open: Message,
        on_dismiss: Message,
        expanded: bool,
    ) -> DropDown<'a, Message>
    where
        Message: 'a + Clone,
    {
        type ButtonStatus = iced::widget::button::Status;
        type ButtonStyle = iced::widget::button::Style;
        type QuadStatus = crate::style::quad::Status;
        type QuadStyle = crate::style::quad::Style;

        fn base_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let palette = theme.palette();
            let base = ButtonStyle {
                background: Some(palette.subtle_fill_color_transparent.into()),
                text_color: palette.text_fill_color_primary,
                border: Border {
                    color: palette.control_stroke_color_default,
                    radius: Radius::new(4),
                    width: 1.0,
                },
                shadow: Shadow::default(),
            };

            match status {
                ButtonStatus::Active => ButtonStyle {
                    border: Border::default(),
                    ..base
                },
                ButtonStatus::Hovered => ButtonStyle {
                    background: Some(palette.subtle_fill_color_secondary.into()),
                    ..base
                },
                ButtonStatus::Pressed => ButtonStyle {
                    background: Some(palette.subtle_fill_color_tertiary.into()),
                    ..base
                },
                ButtonStatus::Disabled => ButtonStyle {
                    background: Some(palette.subtle_fill_color_disabled.into()),
                    text_color: palette.text_fill_color_disabled,
                    ..base
                },
            }
        }

        fn icon_button_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let base = base_style(theme, status);

            ButtonStyle {
                border: Border {
                    radius: base.border.radius.bottom(0),
                    ..base.border
                },
                ..base
            }
        }

        fn indicator_button_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let base = base_style(theme, status);

            ButtonStyle {
                border: Border {
                    radius: base.border.radius.top(0),
                    ..base.border
                },
                ..base
            }
        }

        fn quad_style(theme: &theme::Theme, status: QuadStatus) -> QuadStyle {
            let palette = theme.palette();
            let base = QuadStyle {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
            };

            match status {
                QuadStatus::Active => base,
                QuadStatus::Hovered => QuadStyle {
                    border: Border {
                        color: palette.control_stroke_color_default,
                        radius: Radius::new(4),
                        width: 1.0,
                    },
                    ..base
                },
            }
        }

        let icon_button = Button::new(center(text::icon(icon.codepoint()).size(28)))
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(0)
            .on_press(on_press)
            .style(icon_button_style);

        let indicator_button = Button::new(
            Column::new()
                .push(text::caption1(label))
                .push(
                    text::icon(FluentIcon::ChevronDown.codepoint())
                        .size(10)
                        .align_y(Vertical::Center),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .padding(0)
        .on_press(on_open)
        .style(indicator_button_style);

        let underlay = Quad::new(Some(
            Column::new()
                .push(icon_button)
                .push(indicator_button)
                .width(Length::Fill)
                .height(Length::Fill),
        ))
        .width(40)
        .height(72)
        .style(quad_style);

        DropDown::new(underlay, overlay, expanded).on_dismiss(on_dismiss)
    }
}

/// A horizontal strip of small icon-only buttons (like Bold/Italic/Underline or
/// alignment controls). Mirrors Aurora's CommandButtonStrip.
pub mod button_strip {
    use crate::{
        fluent_icon::FluentIcon,
        widget::{text, Button, Element, Row},
    };

    use iced::{
        border::Radius,
        widget::center,
        Border, Shadow,
    };

    use crate::theme;

    type ButtonStatus = iced::widget::button::Status;
    type ButtonStyle = iced::widget::button::Style;

    fn strip_button_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
        let palette = theme.palette();
        let base = ButtonStyle {
            background: Some(palette.subtle_fill_color_transparent.into()),
            text_color: palette.text_fill_color_primary,
            border: Border {
                color: palette.control_stroke_color_default,
                radius: Radius::new(0),
                width: 0.0,
            },
            shadow: Shadow::default(),
        };

        match status {
            ButtonStatus::Active => base,
            ButtonStatus::Hovered => ButtonStyle {
                background: Some(palette.subtle_fill_color_secondary.into()),
                border: Border {
                    width: 1.0,
                    radius: Radius::new(4),
                    ..base.border
                },
                ..base
            },
            ButtonStatus::Pressed => ButtonStyle {
                background: Some(palette.subtle_fill_color_tertiary.into()),
                ..base
            },
            ButtonStatus::Disabled => ButtonStyle {
                background: Some(palette.subtle_fill_color_disabled.into()),
                text_color: palette.text_fill_color_disabled,
                ..base
            },
        }
    }

    fn strip_button_selected_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
        let palette = theme.palette();
        let base = ButtonStyle {
            background: Some(palette.subtle_fill_color_secondary.into()),
            text_color: palette.text_fill_color_primary,
            border: Border {
                color: palette.control_stroke_color_default,
                radius: Radius::new(0),
                width: 1.0,
            },
            shadow: Shadow::default(),
        };

        match status {
            ButtonStatus::Active => base,
            ButtonStatus::Hovered => ButtonStyle {
                background: Some(palette.subtle_fill_color_tertiary.into()),
                border: Border {
                    radius: Radius::new(4),
                    ..base.border
                },
                ..base
            },
            ButtonStatus::Pressed => ButtonStyle {
                background: Some(palette.control_fill_color_tertiary.into()),
                ..base
            },
            ButtonStatus::Disabled => ButtonStyle {
                background: Some(palette.subtle_fill_color_disabled.into()),
                text_color: palette.text_fill_color_disabled,
                ..base
            },
        }
    }

    /// A single strip button (icon-only, no border radius) for use within a strip.
    pub fn icon_button<'a, Message>(icon: FluentIcon, selected: bool) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        let btn = Button::new(center(text::icon(icon.codepoint())))
            .width(24)
            .height(24)
            .padding(0);

        if selected {
            btn.style(strip_button_selected_style)
        } else {
            btn.style(strip_button_style)
        }
    }

    /// Creates a horizontal strip of icon buttons from a slice of (icon, selected) pairs.
    /// Returns the Row so callers can attach .on_press to each button individually.
    pub fn horizontal<'a, Message>(
        buttons: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        Row::from_vec(buttons)
            .spacing(0)
            .height(24)
            .into()
    }
}

/// A RibbonBand is a labeled group of controls within a task.
/// It wraps its content with a title label at the bottom, matching the Aurora pattern.
pub mod band {
    use crate::{
        widget::{text, Container, Element, Row},
    };

    use iced::{
        alignment::Horizontal,
        border::Radius,
        widget::column,
        Border, Length,
    };

    /// Creates a ribbon band with a title label at the bottom.
    /// The `content` is the main body of the band (buttons, strips, etc.).
    pub fn band<'a, Message: 'a + Clone>(
        title: &'a str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        let title_label = Container::new(
            text::caption1(title)
                .align_x(Horizontal::Center)
                .width(Length::Fill),
        )
        .padding([2, 4])
        .width(Length::Shrink);

        Container::new(
            column![content.into(), title_label,]
                .spacing(0)
                .width(Length::Shrink)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .padding([4, 2])
        .into()
    }

    /// Groups multiple bands side-by-side with vertical separators between them.
    /// When bands overflow the available width, the group scrolls horizontally.
    pub fn band_group<'a, Message: 'a + Clone>(
        bands: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        let mut row = Row::new().spacing(0).height(88);

        for (i, b) in bands.into_iter().enumerate() {
            if i > 0 {
                row = row.push(super::separator());
            }
            row = row.push(b);
        }

        let scrollable = crate::widget::scrollable::horizontal(row)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scrollable)
            .style(|theme| {
                let palette = theme.palette();
                iced::widget::container::Style::default()
                    .background(palette.layer_fill_color_alt)
                    .border(Border {
                        color: palette.card_stroke_color_default,
                        radius: Radius::new(4),
                        width: 1.0,
                    })
            })
            .height(100)
            .into()
    }
}

/// Tab bar for switching between ribbon tasks.
/// Mirrors Aurora's RibbonTask tabs.
pub mod tab_bar {
    use crate::{
        theme,
        widget::{text, Button, Container, Element, Row},
    };

    use iced::{
        alignment::Vertical,
        border::Radius,
        Border, Length, Padding, Shadow,
    };

    type ButtonStatus = iced::widget::button::Status;
    type ButtonStyle = iced::widget::button::Style;

    fn tab_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
        let palette = theme.palette();
        let base = ButtonStyle {
            background: Some(palette.subtle_fill_color_transparent.into()),
            text_color: palette.text_fill_color_secondary,
            border: Border::default(),
            shadow: Shadow::default(),
        };

        match status {
            ButtonStatus::Active => base,
            ButtonStatus::Hovered => ButtonStyle {
                background: Some(palette.subtle_fill_color_secondary.into()),
                text_color: palette.text_fill_color_primary,
                border: Border {
                    radius: Radius::new(4).bottom(0),
                    ..base.border
                },
                ..base
            },
            ButtonStatus::Pressed => ButtonStyle {
                background: Some(palette.subtle_fill_color_tertiary.into()),
                text_color: palette.text_fill_color_primary,
                ..base
            },
            ButtonStatus::Disabled => ButtonStyle {
                text_color: palette.text_fill_color_disabled,
                ..base
            },
        }
    }

    fn tab_active_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
        let palette = theme.palette();
        let base = ButtonStyle {
            background: Some(palette.layer_fill_color_alt.into()),
            text_color: palette.text_fill_color_primary,
            border: Border {
                color: palette.card_stroke_color_default,
                radius: Radius::new(4).bottom(0),
                width: 1.0,
            },
            shadow: Shadow::default(),
        };

        match status {
            ButtonStatus::Active => base,
            ButtonStatus::Hovered => base,
            ButtonStatus::Pressed => ButtonStyle {
                background: Some(palette.subtle_fill_color_tertiary.into()),
                ..base
            },
            ButtonStatus::Disabled => ButtonStyle {
                text_color: palette.text_fill_color_disabled,
                ..base
            },
        }
    }

    fn contextual_tab_style(
        hue: iced::Color,
    ) -> impl Fn(&theme::Theme, ButtonStatus) -> ButtonStyle {
        move |theme: &theme::Theme, status: ButtonStatus| {
            let palette = theme.palette();
            let tinted = iced::Color {
                a: 0.15,
                ..hue
            };
            let base = ButtonStyle {
                background: Some(tinted.into()),
                text_color: palette.text_fill_color_primary,
                border: Border::default(),
                shadow: Shadow::default(),
            };

            match status {
                ButtonStatus::Active => base,
                ButtonStatus::Hovered => ButtonStyle {
                    background: Some(iced::Color { a: 0.25, ..hue }.into()),
                    border: Border {
                        radius: Radius::new(4).bottom(0),
                        ..base.border
                    },
                    ..base
                },
                ButtonStatus::Pressed => ButtonStyle {
                    background: Some(iced::Color { a: 0.35, ..hue }.into()),
                    ..base
                },
                ButtonStatus::Disabled => ButtonStyle {
                    text_color: palette.text_fill_color_disabled,
                    ..base
                },
            }
        }
    }

    fn contextual_tab_active_style(
        hue: iced::Color,
    ) -> impl Fn(&theme::Theme, ButtonStatus) -> ButtonStyle {
        move |theme: &theme::Theme, status: ButtonStatus| {
            let palette = theme.palette();
            let tinted = iced::Color {
                a: 0.3,
                ..hue
            };
            let base = ButtonStyle {
                background: Some(tinted.into()),
                text_color: palette.text_fill_color_primary,
                border: Border {
                    color: iced::Color { a: 0.5, ..hue },
                    radius: Radius::new(4).bottom(0),
                    width: 1.0,
                },
                shadow: Shadow::default(),
            };

            match status {
                ButtonStatus::Active => base,
                ButtonStatus::Hovered => base,
                ButtonStatus::Pressed => ButtonStyle {
                    background: Some(iced::Color { a: 0.4, ..hue }.into()),
                    ..base
                },
                ButtonStatus::Disabled => ButtonStyle {
                    text_color: palette.text_fill_color_disabled,
                    ..base
                },
            }
        }
    }

    /// A single tab button for the task bar.
    pub fn tab<'a, Message: 'a + Clone>(
        label: &'a str,
        active: bool,
        on_press: Message,
    ) -> Element<'a, Message> {
        let btn = Button::new(
            text::caption1(label)
                .align_y(Vertical::Center)
                .height(Length::Fill),
        )
        .height(28)
        .padding(Padding::new(0.0).left(12).right(12))
        .on_press(on_press);

        if active {
            btn.style(tab_active_style).into()
        } else {
            btn.style(tab_style).into()
        }
    }

    /// A contextual tab with a colored tint.
    pub fn contextual_tab<'a, Message: 'a + Clone>(
        label: &'a str,
        hue: iced::Color,
        active: bool,
        on_press: Message,
    ) -> Element<'a, Message> {
        let btn = Button::new(
            text::caption1(label)
                .align_y(Vertical::Center)
                .height(Length::Fill),
        )
        .height(28)
        .padding(Padding::new(0.0).left(12).right(12))
        .on_press(on_press);

        if active {
            btn.style(contextual_tab_active_style(hue)).into()
        } else {
            btn.style(contextual_tab_style(hue)).into()
        }
    }

    /// Creates a tab bar row from a list of tab elements.
    pub fn tab_bar<'a, Message: 'a + Clone>(
        tabs: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        Container::new(
            Row::from_vec(tabs)
                .spacing(2)
                .height(Length::Fill)
                .align_y(Vertical::Bottom),
        )
        .height(32)
        .padding(Padding::new(0.0).left(4).top(4))
        .into()
    }
}

/// Complete ribbon widget combining tab bar + band area.
/// This assembles the full ribbon structure similar to AuroraRibbonWindow's ribbon area.
pub mod ribbon_bar {
    use crate::widget::{Container, Element};

    use iced::{
        border::Radius,
        widget::column,
        Border, Length,
    };

    /// Assembles a full ribbon: tab bar on top, band content below.
    pub fn ribbon<'a, Message: 'a + Clone>(
        tab_bar: Element<'a, Message>,
        band_content: Element<'a, Message>,
    ) -> Element<'a, Message> {
        Container::new(
            column![tab_bar, band_content,]
                .spacing(0)
                .width(Length::Fill),
        )
        .style(|theme| {
            let palette = theme.palette();
            iced::widget::container::Style::default()
                .background(palette.solid_background_fill_color_quarternary)
                .border(Border {
                    color: palette.card_stroke_color_default,
                    radius: Radius::new(0),
                    width: 0.0,
                })
        })
        .width(Length::Fill)
        .into()
    }
}
