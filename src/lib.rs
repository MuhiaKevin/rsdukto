mod dukto_download;
mod client_discovery;


use anyhow::Result;

pub fn run() -> Result<()> {
    client_discovery::discover_clients();
    dukto_download::download()?;
    Ok(())
}
