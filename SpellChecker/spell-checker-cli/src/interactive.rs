use std::{rc::Rc, sync::Arc};

use anyhow::Result;
use cursive::{
    backends::crossterm::crossterm::style::Stylize,
    event::{Event, Key},
    theme::{BaseColor, BorderStyle, Color, Palette},
    utils::markup::StyledString,
    view::{Finder, Nameable, Resizable},
    views::{self, Button, LinearLayout, OnEventView, Panel, TextArea, TextContent, TextView},
    Cursive, View, With,
};
use spell_checker::SpellChecker;

///////////////////////////////////////////////////////////////////////////////

pub fn start_interactive(spell_checker: SpellChecker) -> Result<()> {
    let spell_checker = Arc::from(spell_checker);

    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                palette[EditableTextCursor] = Style::from(White.light()).combine(Reverse);
                palette[EditableText] = Style::background();
            }
        }),
    });

    let mut contents = StyledString::plain("");
    let mut wrapper = TextContent::new(contents);
    let mut wrapper2 = wrapper.clone();

    siv.add_layer(
        LinearLayout::horizontal()
            .child(
                Panel::new(
                    OnEventView::new(TextArea::new().content("").with_name("text"))
                        .on_pre_event(Key::Enter, move |s| {
                            s.call_on_name("text", |v: &mut views::TextArea| {
                                let content = v.get_content().to_owned();
                                let mut res = StyledString::plain("");

                                for word in content.split_whitespace() {
                                    if !spell_checker.check(word) {
                                        res.append(StyledString::styled(
                                            word.to_owned() + "\n",
                                            Color::Light(BaseColor::Red),
                                        ));

                                        for suggestion in spell_checker.suggest(word) {
                                            res.append_plain(suggestion);
                                            res.append_plain('\n');
                                        }
                                    } else {
                                    }
                                }

                                // v.call_on_name("suggestions", |v: &mut TextView| {
                                //     v.set_content(res);
                                // });

                                wrapper2.set_content(res);

                                v.on_event(Event::Key(Key::Enter));

                                // v.set_content(content);
                            });
                        })
                        .on_event(Key::Esc, |s| s.quit()),
                )
                .full_width()
                .full_height(),
            )
            .child(
                Panel::new(TextView::new_with_content(wrapper.clone()).with_name("suggestions"))
                    .title("Suggestions")
                    .fixed_width(25)
                    .full_height(),
                // .child(
                //     Panel::new(Button::new("Quit", |s| s.quit()))
                //         .title("Suggestions")
                //         .fixed_width(25)
                //         .full_height(),
            ),
    );

    siv.run();

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
