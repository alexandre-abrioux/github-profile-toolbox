#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    static BIN_NAME: &'static str = "github-profile-toolbox";

    #[test]
    fn should_handle_missing_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("--config").arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Configuration file not found"));
        Ok(())
    }

    #[test]
    fn should_generate_toolbox() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("config.yaml")?;
        file.write_str(
            "sections:
  ides:
    - jetbrains
    - neovim
  languages:
    - javascript
    - cplusplus
    - rust
    - php",
        )?;

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("--config").arg(file.path());
        cmd.assert().success().stdout(predicate::eq("|ides|languages|
|-|-|
|[<img align=\"left\" alt=\"JetBrains\" src=\"https://img.shields.io/badge/-JetBrains-000000?logo=jetbrains&logoColor=white\">](#)|[<img align=\"left\" alt=\"JavaScript\" src=\"https://img.shields.io/badge/-JavaScript-F7DF1E?logo=javascript&logoColor=black\">](#)|
|[<img align=\"left\" alt=\"Neovim\" src=\"https://img.shields.io/badge/-Neovim-57A143?logo=neovim&logoColor=white\">](#)|[<img align=\"left\" alt=\"C++\" src=\"https://img.shields.io/badge/-C++-00599C?logo=cplusplus&logoColor=white\">](#)|
||[<img align=\"left\" alt=\"Rust\" src=\"https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white\">](#)|
||[<img align=\"left\" alt=\"PHP\" src=\"https://img.shields.io/badge/-PHP-777BB4?logo=php&logoColor=white\">](#)|
"));

        Ok(())
    }
}
