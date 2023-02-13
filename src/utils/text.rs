use rustyline::line_buffer::LineBuffer;
use textwrap::core::Word;
use tui::{style::Style, text::Span};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Acquiring the horizontal position of the cursor so it can be rendered visually.
pub fn get_cursor_position(line_buffer: &LineBuffer) -> usize {
    line_buffer
        .as_str()
        .grapheme_indices(true)
        .take_while(|(offset, _)| *offset != line_buffer.pos())
        .map(|(_, cluster)| cluster.width())
        .sum()
}

pub enum TitleStyle<'a> {
    Combined(&'a str, &'a str),
    Single(&'a str),
    Custom(Span<'a>),
}

pub fn title_spans<'a>(contents: &'a [TitleStyle<'a>], style: Style) -> Vec<Span<'a>> {
    let mut complete = Vec::new();

    for (i, item) in contents.iter().enumerate() {
        let first_bracket = Span::raw(format!("{}[ ", if i == 0 { "" } else { " " }));

        complete.extend(match item {
            TitleStyle::Combined(title, value) => vec![
                first_bracket,
                Span::styled((*title).to_string(), style),
                Span::raw(format!(": {value} ]")),
            ],
            TitleStyle::Single(value) => vec![
                first_bracket,
                Span::styled((*value).to_string(), style),
                Span::raw(" ]"),
            ],
            TitleStyle::Custom(span) => vec![first_bracket, span.clone(), Span::raw(" ]")],
        });
    }

    complete
}

/// Within an array of strings, find the first partial or full match, if any.
pub fn first_similarity(possibilities: &[String], search: &str) -> Option<String> {
    possibilities
        .iter()
        .filter(|s| s.starts_with(search))
        .collect::<Vec<&String>>()
        .first()
        .and_then(|result| {
            if result.len() > search.len() {
                Some((*result).to_string())
            } else {
                None
            }
        })
}

/// Wraps the first line according to the width, letting the second line go as long as it would like.
/// Modified version of function
/// [`wrap_first_fit`](<https://github.com/mgeisler/textwrap/blob/74b55209a75a49e4fadde3e07a6a33cdd2f24f5d/src/wrap_algorithms.rs#L347-L371/>)
pub fn wrap_once<'a, 'b>(words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]> {
    let default_line_width = line_widths.last().copied().unwrap_or(0);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut width = 0;

    for (idx, word) in words.iter().enumerate() {
        let line_width = line_widths
            .get(lines.len())
            .copied()
            .unwrap_or(default_line_width);

        if width + word.width() > line_width && idx > start {
            lines.push(&words[start..idx]);
            start = idx;
            break;
        }

        width += word.width();
    }

    lines.push(&words[start..]);

    lines
}

#[cfg(test)]
mod tests {
    use textwrap::{wrap, Options, WrapAlgorithm};
    use tui::{
        style::{Color, Modifier},
        text::Spans,
    };

    use super::*;

    #[test]
    fn test_get_cursor_position_with_single_byte_graphemes() {
        let text = "never gonna give you up";
        let mut line_buffer = LineBuffer::with_capacity(25);
        line_buffer.insert_str(0, text);

        assert_eq!(get_cursor_position(&line_buffer), 0);
        line_buffer.move_forward(1);
        assert_eq!(get_cursor_position(&line_buffer), 1);
        line_buffer.move_forward(2);
        assert_eq!(get_cursor_position(&line_buffer), 3);
    }

    #[test]
    fn test_get_cursor_position_with_three_byte_graphemes() {
        let text = "绝对不会放弃你";
        let mut line_buffer = LineBuffer::with_capacity(25);
        line_buffer.insert_str(0, text);

        assert_eq!(get_cursor_position(&line_buffer), 0);
        line_buffer.move_forward(1);
        assert_eq!(get_cursor_position(&line_buffer), 2);
        line_buffer.move_forward(2);
        assert_eq!(get_cursor_position(&line_buffer), 6);
    }

    #[test]
    fn test_2_dimensional_vector_to_spans() {
        let s = Spans::from(title_spans(
            &[TitleStyle::Combined("Time", "Some time")],
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));

        assert_eq!(s.width(), "[ Time: Some time ]".len());
    }

    #[test]
    fn test_first_similarity_some_output() {
        let v = vec!["Nope".to_string()];

        let output = first_similarity(&v, "No");

        assert_eq!(output, Some("Nope".to_string()));
    }

    #[test]
    fn test_first_similarity_no_output() {
        let v = vec!["Something".to_string()];

        let output = first_similarity(&v, "blah");

        assert_eq!(output, None);
    }

    #[test]
    fn test_first_similarity_no_input_no_output() {
        let output = first_similarity(&[], "asdf");

        assert_eq!(output, None);
    }

    #[test]
    fn test_wrap_once_to_one_line() {
        let options = Options::new(20).wrap_algorithm(WrapAlgorithm::Custom(wrap_once));

        assert_eq!(
            wrap("Something, another", options),
            vec!["Something, another"]
        );
    }

    #[test]
    fn test_wrap_once_to_two_lines() {
        let options = Options::new(10).wrap_algorithm(WrapAlgorithm::Custom(wrap_once));

        assert_eq!(
            wrap("First, second, third, fourth, fifth, sixth", options),
            vec!["First,", "second, third, fourth, fifth, sixth"]
        );
    }

    #[test]
    fn test_wrap_once_one_long_word_to_two_lines() {
        let options = Options::new(10).wrap_algorithm(WrapAlgorithm::Custom(wrap_once));

        assert_eq!(
            wrap("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", options),
            vec!["aaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
        );
    }
}
