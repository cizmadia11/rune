//! Rune-only text-weight reproduction. Capture with `--snap pairs.png --scale 1`.
use rune::{AppSpec, Color, Cx, El, Family, TextStyle, col, row, text};

fn view(_: &(), _: &mut Cx<()>) -> El {
    let mut page = col()
        .w_full()
        .h_full()
        .padding(16.0)
        .gap(0.0)
        .fill(Color::BLACK);
    let cases = [
        ("h1", 24.0, 600.0, false, "Segoe UI Variable Text"),
        ("h2", 21.0, 600.0, false, "Segoe UI Variable Text"),
        ("h3", 19.0, 600.0, false, "Segoe UI Variable Text"),
        ("bold", 17.5, 700.0, false, "Cambria"),
        ("italic", 17.5, 400.0, true, "Cambria"),
        ("body", 17.5, 400.0, false, "Cambria"),
        ("title", 13.5, 600.0, false, "Segoe UI Variable Text"),
        ("toolbar", 12.5, 500.0, false, "Segoe UI Variable Text"),
        (
            "properties-header",
            11.0,
            400.0,
            false,
            "Segoe UI Variable Text",
        ),
        (
            "properties-key",
            11.5,
            500.0,
            false,
            "Segoe UI Variable Text",
        ),
        ("palette", 13.0, 500.0, false, "Segoe UI Variable Text"),
        ("inter", 13.0, 500.0, false, "Inter"),
    ];
    for (name, size, weight, italic, family) in cases {
        for (index, suffix) in [
            "'cafe' - ...",
            "’cafe' - ...",
            "“cafe” - ...",
            "'cafe' — ...",
            "'cafe' - …",
            "'café' - ...",
        ]
        .iter()
        .enumerate()
        {
            let mut style = TextStyle::new(size, weight)
                .lh(32.0)
                .family(Family::Named(family));
            if italic {
                style = style.italic();
            }
            page = page.child(
                row().h(32.0).child(
                    text(format!("Sample motion {suffix}"))
                        .key(format!("{name}-{index}"))
                        .font(style)
                        .color(Color::WHITE),
                ),
            );
        }
    }
    page
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use rune::text::TextSystem;

    #[test]
    fn ascii_and_unicode_choose_the_same_system_face() {
        let mut system = TextSystem::new(&[], ("Segoe UI", "Consolas"));
        let style = TextStyle::new(13.0, 500.0).family(Family::Named("Segoe UI"));
        let ascii = system.layout_rich("Sample motion ...", &[], style, 1.0, None);
        let unicode = system.layout_rich("Sample motion …", &[], style, 1.0, None);
        let a = ascii.lines().next().unwrap().runs().next().unwrap();
        let b = unicode.lines().next().unwrap().runs().next().unwrap();
        assert_eq!(
            (a.font().data.id(), a.font().index),
            (b.font().data.id(), b.font().index),
            "punctuation must not change the system face for shared ASCII text"
        );
        assert_eq!(
            a.normalized_coords(),
            b.normalized_coords(),
            "variation coordinates must agree"
        );
    }
}

fn main() {
    rune::run(AppSpec {
        families: ("Segoe UI", "Consolas"),
        ..AppSpec::new("Text weight reproduction", (600.0, 2336.0), || (), view)
    });
}
