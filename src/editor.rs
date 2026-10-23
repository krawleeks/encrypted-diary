use std::fs;
use std::io::Write;
use std::process::Command;

//put diary in /temp and open in nvim. this fn returns a string of edited text
pub fn open_in_editor(initial: &str, read_only: bool) -> Result<String, String> {
    let mut tmp = tempfile::Builder::new()
        .prefix("diary-")
        .suffix(".txt")
        .tempfile_in("/tmp")
        .map_err(|e| format!("could not create temp file: {}", e))?;

    tmp.write_all(initial.as_bytes())
        .map_err(|e| format!("could not write temp file: {}", e))?;
    tmp.flush()
        .map_err(|e| format!("could not write temp file: {}", e))?;

    let mut cmd = Command::new("nvim");
    //remove swap files, backups, and undo files, so that the diary file is not left behind in case of a crash
    cmd.args(["-n", "-i", "NONE", "-c", "set nobackup nowritebackup noundofile noswapfile"]);
    if read_only {
        cmd.arg("-R");
    }
    cmd.arg(tmp.path());

    let status = cmd
        .status()
        .map_err(|e| format!("could not start nvim: {}", e))?;
    if !status.success() {
        return Err("nvim exited with an error, changes discarded".to_string());
    }

    let edited = fs::read_to_string(tmp.path())
        .map_err(|e| format!("could not read temp file: {}", e))?;
    Ok(edited)

}