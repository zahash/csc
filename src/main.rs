mod eval;
mod lex;
mod parse;
mod prompt;

fn main() -> anyhow::Result<()> {
    prompt::run()
}
