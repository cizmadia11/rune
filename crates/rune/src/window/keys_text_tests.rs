use super::*;
use crate::{Cx, El, Env, Ime, Runtime, TextEdit, TextStyle, text_input};
use winit::keyboard::NativeKey;

#[derive(Default)]
struct Field { edit: TextEdit }

fn field(s: &Field, _: &mut Cx<Field>) -> El {
    text_input(&s.edit, TextStyle::new(14.0, 400.0)).key("reply").size(240.0, 40.0)
        .on_edit(|s: &mut Field| &mut s.edit)
}

fn focused() -> Runtime<Field> {
    let mut rt = Runtime::new(Field::default(), field, (240.0, 40.0), Env::new(true, 1.0));
    rt.frame(0.0);
    let (x, y) = rt.find("reply").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    rt
}

fn packet(text: Option<&str>, mods: ModifiersState) -> Option<KeyEvent> {
    // These are the public KeyEvent fields winit reports for Windows VK_PACKET.
    map_key_text(&WKey::Unidentified(NativeKey::Windows(0xe7)), text, mods)
}

#[test]
fn unidentified_text_reaches_the_focused_field() {
    let mut rt = focused();
    for text in ["interrogatory", "é € ع", "😀", "two characters"] {
        let event = packet(Some(text), ModifiersState::empty())
            .expect("a winit unidentified key's text must reach the focused field");
        rt.key(event);
        rt.frame(0.0);
    }
    assert_eq!(rt.state.edit.text, "interrogatoryé € ع😀two characters");
}

#[test]
fn unidentified_text_does_not_invent_shortcuts_or_control_keys() {
    for text in [None, Some(""), Some("\r"), Some("\t"), Some("\u{1b}")] {
        assert!(packet(text, ModifiersState::empty()).is_none());
    }
    for mods in [ModifiersState::CONTROL, ModifiersState::SUPER, ModifiersState::CONTROL | ModifiersState::ALT] {
        assert!(packet(Some("x"), mods).is_none());
    }
    let enter = map_key_text(&WKey::Named(NamedKey::Enter), Some("\r"), ModifiersState::empty()).unwrap();
    assert_eq!(enter.key, Key::Enter);
    assert!(map_key_text(&WKey::Dead(Some('é')), Some("é"), ModifiersState::empty()).is_none());
}

#[test]
fn unidentified_text_respects_focus_and_ime_commit() {
    let mut rt = Runtime::new(Field::default(), field, (240.0, 40.0), Env::new(true, 1.0));
    rt.frame(0.0);
    rt.key(packet(Some("unfocused"), ModifiersState::empty()).unwrap());
    rt.frame(0.0);
    assert!(rt.state.edit.text.is_empty());
    let mut rt = focused();
    rt.ime(Ime::Preedit("に".into(), Some((0, 3))));
    rt.ime(Ime::Commit("日本".into()));
    rt.key(packet(Some("é"), ModifiersState::empty()).unwrap());
    rt.frame(0.0);
    assert_eq!(rt.state.edit.text, "日本é");
}
