use anyhow::Result;
use rsdukto::dukto_download;

fn main() -> Result<()> {
    dukto_download::download()?;

    Ok(())
}
