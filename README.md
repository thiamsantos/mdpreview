# mdpreview

> Preview markdown files.

Markdown files are rendered using [Github API](https://docs.github.com/en/rest/reference/markdown#render-a-markdown-document-in-raw-mode).
GitHub Markdown style are provided by [github-markdown-css](https://github.com/sindresorhus/github-markdown-css).

## Installation

You need `go` installed and `GOBIN` in your `PATH`. Once that is done, run the
command:

```shell
$ go get -u github.com/thiamsantos/mdpreview
```

## Usage

```
Usage
  $ mdpreview [options]

Options
  -i string
        Markdown file to render (default "README.md")

Examples
  $ mdpreview
  $ mdpreview -f some/file.md
```
