use copypasta::{ClipboardContext, ClipboardProvider};
use eyre::eyre;
use std::fmt::Debug;

#[inline(always)]
fn map_err(err: impl Debug) -> eyre::Report {
    eyre!("{:?}", err)
}

pub fn get_clipboard() -> eyre::Result<String> {
    let mut context = ClipboardContext::new().map_err(map_err)?;
    let content = context.get_contents().map_err(map_err)?;
    Ok(content)
}

pub fn set_clipboard(s: String) -> eyre::Result<()> {
    let mut context = ClipboardContext::new().map_err(map_err)?;
    context.set_contents(s).map_err(map_err)?;
    Ok(())
}
