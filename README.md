# Zed Verilog Extension

Adds Verilog and SystemVerilog support for [Zed](https://zed.dev/).

## Features

- Treesitter based syntax highlighting
- Diagnostics
- Auto-completion
- Hover documentation
- Formatting (format-on-save is disabled by default)

## Installation

1. Open Zed
2. Open the command palette
3. Enter `zed: extensions`
4. Search for `Verilog` using the search bar on the extensions page.
5. Press `Install` and the language servers will automatically be downloaded.

## Language Servers

This extension has three language servers: [Verible](https://github.com/chipsalliance/verible), [Veridian](https://github.com/vivekmalneedi/veridian), and [slang-server](https://github.com/hudson-trading/slang-server). **By default, only Verible and Veridian are enabled** due to slang-server being relatively new and having an issue where it produces errors on non-verilog files. To enable slang-server, add the following to your settings file:

```json
"languages": {
  "SystemVerilog": {
    "language_servers": ["..."]
  }
}
```

## Configuration

Configuration for this extension is done in two places. First is by adding "SystemVerilog" your settings file and using the [language settings](https://zed.dev/docs/configuring-zed#languages).

Secondly, you can configure the veridian language server by creating a `veridian.yml` file in your project root and using the settings documented [here](https://github.com/vivekmalneedi/veridian?tab=readme-ov-file#configuration).

> [!NOTE]
> Since this extension doesn't use the verible language server through veridian, verible specific settings in `veridian.yml` will not work.

## License

This project is licensed under the [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license or the [MIT](http://opensource.org/licenses/MIT) license, at your option. See [COPYRIGHT](./COPYRIGHT) for more details.
