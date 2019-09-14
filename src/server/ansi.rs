use ansi_parser::AnsiParser;
use ansi_parser::AnsiSequence;
use ansi_parser::Output as AnsiOutput;

#[derive(Debug)]
pub enum Ansi {
    Text(String),
    Escape(AnsiSequence),
}

pub fn parse_bytes(bytes: &[u8]) -> Result<Vec<Ansi>, std::io::Error> {
    let text = String::from_utf8(Vec::from(bytes)).unwrap();

    Ok(text
        .ansi_parse()
        .map(|x| match x {
            AnsiOutput::TextBlock(text) => {
                match text {
                    "\u{1b}[A" => {
                        Ansi::Escape(AnsiSequence::CursorUp(1))
                    }
                    "\u{1b}[B" => {
                        Ansi::Escape(AnsiSequence::CursorDown(1))
                    }
                    "\u{1b}[C" => {
                        Ansi::Escape(AnsiSequence::CursorForward(1))
                    }
                    "\u{1b}[D" => {
                        Ansi::Escape(AnsiSequence::CursorBackward(1))
                    }
                    _ => {
                        Ansi::Text(String::from(text))
                    }
                }
            }
            AnsiOutput::Escape(esc) => {
                Ansi::Escape(esc)
            }
        })
        .collect::<Vec<Ansi>>())
}

pub fn to_string(ansi: Ansi) -> String {
    match ansi {
        Ansi::Text(text) => text,
        Ansi::Escape(esc) => {
            format!("{}", esc)
        }
    }
}
