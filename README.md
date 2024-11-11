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

<!-- START_SECTION:toolbox --><!-- STOP_SECTION:toolbox -->

Here is a
[real-life example](https://github.com/alexandre-abrioux#hammer_and_wrench-toolbox).

## Instructions

- Add the comment `<!-- START_SECTION:toolbox --><!-- STOP_SECTION:toolbox -->` within `README.md`.
  You can find an example
  [here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/main/README.md?plain=1).

- Add the following workflow file to your profile repository:

`.github/workflows/update-toolbox.yaml`

```yaml
name: Update Toolbox
on:
  push:
    branches: [ main ]
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
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - uses: peter-evans/create-pull-request@v7
        with:
          commit-message: "docs(readme): update toolbox"
          title: "docs(readme): update toolbox"
          body: "Toolbox updated by [github-profile-toolbox](https://github.com/alexandre-abrioux/github-profile-toolbox) GitHub action"
          branch: patch/toolbox
          sign-commits: true
          reviewers: ${{ github.actor }}
```

- Go to your profile repository and under "Settings > Actions > General > Workflow permissions"
  activate the option "Allow GitHub Actions to create and approve pull requests".

You can find an example
[here](https://github.com/alexandre-abrioux/alexandre-abrioux/blob/main/.github/workflows/update-toolbox.yaml).

### Override defaults

Use the following input parameters to customize it for your use case:

| Input Param | Description                                     | Default Value                         |
|-------------|-------------------------------------------------|---------------------------------------|
| `config`    | path to the YAML configuration file             | `.github/github-profile-toolbox.yaml` |
| `readme`    | path to the README file that should be modified | `README.md`                           |

For instance, if you which to place your configuration file somewhere else, use the following:

```yaml
- uses: alexandre-abrioux/github-profile-toolbox@v1
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    config: "my-custom-path.yaml"
```
