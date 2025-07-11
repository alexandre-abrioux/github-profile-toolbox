# 🔧 GitHub Profile Toolbox

Use this GitHub action to display a stylish Markdown table on your GitHub profile
that summarizes your skill set.

## Example

Giving the following configuration passed as an input to the GitHub action:

```yaml
tools:
  Language:
    - html5
    - css
    - javascript
  Tool:
    - git
    - github
    - githubactions
  IDE:
    - vscodium
    - jetbrains
    - neovim
```

; you would get the following output:

<!-- @formatter:off -->
<!-- prettier-ignore-start -->
<!-- START_SECTION:toolbox -->
<!-- Generated by github-profile-toolbox GitHub action -->
| Language                                                                                                                       | Tool                                                                                                                                      | IDE                                                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| [<img align="left" alt="HTML5" src="https://img.shields.io/badge/-HTML5-E34F26?logoColor=white&logo=html5">](#)                | [<img align="left" alt="Git" src="https://img.shields.io/badge/-Git-F05032?logoColor=white&logo=git">](#)                                 | [<img align="left" alt="VSCodium" src="https://img.shields.io/badge/-VSCodium-2F80ED?logoColor=white&logo=vscodium">](#)    |
| [<img align="left" alt="CSS" src="https://img.shields.io/badge/-CSS-663399?logoColor=white&logo=css">](#)                      | [<img align="left" alt="GitHub" src="https://img.shields.io/badge/-GitHub-181717?logoColor=white&logo=github">](#)                        | [<img align="left" alt="JetBrains" src="https://img.shields.io/badge/-JetBrains-000000?logoColor=white&logo=jetbrains">](#) |
| [<img align="left" alt="JavaScript" src="https://img.shields.io/badge/-JavaScript-F7DF1E?logoColor=black&logo=javascript">](#) | [<img align="left" alt="GitHub Actions" src="https://img.shields.io/badge/-GitHub Actions-2088FF?logoColor=white&logo=githubactions">](#) | [<img align="left" alt="Neovim" src="https://img.shields.io/badge/-Neovim-57A143?logoColor=white&logo=neovim">](#)          |
<!-- STOP_SECTION:toolbox -->
<!-- prettier-ignore-end -->
<!-- @formatter:on -->

Here is a
[real-life example](https://github.com/alexandre-abrioux#hammer_and_wrench-toolbox).

## Initial Setup

- Add the comment `<!-- START_SECTION:toolbox --><!-- STOP_SECTION:toolbox -->` within `README.md`.
  You can find an example
  [here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/main/README.md?plain=1).

- Add the configuration file to your GitHub profile repository:
  `.github/github-profile-toolbox.yaml`

```yaml
tools:
  Language:
    - html5
    - css3
    - javascript
  Tools:
    - git
    - github
    - githubactions
  IDEs:
    - vscodium
    - jetbrains
    - neovim
```

- Add the following workflow file to your GitHub profile repository:
  `.github/workflows/update-toolbox.yaml`

```yaml
name: Update Toolbox
on:
  push:
    branches: [main]
  workflow_dispatch:
jobs:
  update-toolbox:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - uses: actions/checkout@v4
      - uses: alexandre-abrioux/github-profile-toolbox@v1
      - uses: peter-evans/create-pull-request@v7
        with:
          commit-message: "docs(readme): update toolbox"
          title: "docs(readme): update toolbox"
          body: "Toolbox updated by [github-profile-toolbox](https://github.com/alexandre-abrioux/github-profile-toolbox) GitHub action"
          branch: patch/toolbox
          sign-commits: true
          reviewers: ${{ github.actor }}
```

You can find an example
[here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/main/.github/workflows/update-toolbox.yaml).

- In your GitHub profile repository, under "Settings > Actions > General > Workflow permissions",
  activate the option "Allow GitHub Actions to create and approve pull requests".

You're all set! Next time you push a commit on `main`,
you should receive a pull request to update the toolbox section in your profile's README.

## Configuration

### List of tools

This GitHub action uses <https://shields.io> to generate icons in your profile's README,
and `shield.io` uses <https://simpleicons.org/> to provide logos for brands.
You can find a list of all available brand names here:
<https://github.com/simple-icons/simple-icons/blob/develop/slugs.md>.
Use the `Brand slug` name to furnish your configuration file.

### Custom tool

If the tool is not available on [SimpleIcons](https://simpleicons.org/) or
if you'd like to reference an existing tool with a custom label,
you can do so with the following:

```yaml
tools:
  IDEs:
    # VSCode is not available on SimpleIcons,
    # but you can still reference it without an icon:
    - label: VSCode
      color: "29a9f2"
    # You can also prepend the hex color with the hash symbol, this is valid:
    - label: VSCode
      color: "#29a9f2"
    # RustRover is not available on SimpleIcons,
    # but you could use JetBrains' icon with a custom label and color:
    - label: RustRover
      icon: jetbrains
      color: "34c97a"
```

## GitHub Action Inputs

Use the following input parameters to override defaults and customize the action to your use case:

| Input Param | Description                                     | Default Value                         |
| ----------- | ----------------------------------------------- | ------------------------------------- |
| `config`    | path to the YAML configuration file             | `.github/github-profile-toolbox.yaml` |
| `readme`    | path to the README file that should be modified | `README.md`                           |

For instance, if you which to place your configuration file somewhere else, use the following:

```yaml
- uses: alexandre-abrioux/github-profile-toolbox@v1
  with:
    config: "my-custom-path.yaml"
```

## Contributing

I've bootstrapped this project on a Sunday to learn the basics of Rust.
I'm by no means knowledgeable about the language so feel free to suggest things I could have done better!
I'm open to contributions, although this will probably stay as a slow-maintained side project.
