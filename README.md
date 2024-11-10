# 🔧 GitHub Profile Toolbox

With this GitHub action you can include a nice Markdown table in your GitHub profile
to summarize your skill set.

## Example

Giving the following configuration passed as an input to the GitHub action:

```yaml
tools:
  IDE:
    - jetbrains
    - neovim
  Language:
    - javascript
    - cplusplus
    - rust
    - php
```

; you would get the following output:

| IDE                                                                                                                         | Language                                                                                                                       |
|-----------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| [<img align="left" alt="JetBrains" src="https://img.shields.io/badge/-JetBrains-000000?logo=jetbrains&logoColor=white">](#) | [<img align="left" alt="JavaScript" src="https://img.shields.io/badge/-JavaScript-F7DF1E?logo=javascript&logoColor=black">](#) |
| [<img align="left" alt="Neovim" src="https://img.shields.io/badge/-Neovim-57A143?logo=neovim&logoColor=white">](#)          | [<img align="left" alt="C++" src="https://img.shields.io/badge/-C++-00599C?logo=cplusplus&logoColor=white">](#)                |
|                                                                                                                             | [<img align="left" alt="Rust" src="https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white">](#)                   |
|                                                                                                                             | [<img align="left" alt="PHP" src="https://img.shields.io/badge/-PHP-777BB4?logo=php&logoColor=white">](#)                      |                                                                                                                       | [<img align="left" alt="PHP" src="https://img.shields.io/badge/-PHP-777BB4?logo=php&logoColor=white">](#)                      |

Here is a real-life example:
https://github.com/alexandre-abrioux#hammer_and_wrench-toolbox

## Instructions

- Add the comment `<!--START_SECTION:toolbox -->` (entry point) within `README.md`. You can find an
  example [here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/master/README.md).

- Add the following workflow file to your profile repository:

`.github/workflows/update-toolbox.yml`

```yml
name: Update Toolbox
on:
  push: [ main ]
  workflow_dispatch:
jobs:
  update-toolbox:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - uses: alexandre-abrioux/github-profile-toolbox@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - uses: peter-evans/create-pull-request@v7
        with:
          commit-message: "docs(readme): update toolbox"
          title: "docs(readme): update toolbox"
          branch: patch/toolbox
          reviewers: ${{ github.actor }}
```

You can find an
example [here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/master/.github/workflows/update-toolbox.yml).

### Override defaults

Use the following input parameters to customize it for your use case:

| Input Param | Description                                     | Default Value                        |
|-------------|-------------------------------------------------|--------------------------------------|
| `config`    | path to the YAML configuration file             | `.github/github-profile-toolbox.yml` |
| `readme`    | path to the README file that should be modified | `README.md`                          |

For instance, if you which to place your configuration file somewhere else, use the following:

```yml
- uses: alexandre-abrioux/github-profile-toolbox@v1
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    config: "my-custom-path.yaml"
```
