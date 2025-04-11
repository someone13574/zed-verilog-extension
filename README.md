
# Zed Verilog Extension

Adds Verilog and SystemVerilog support for [Zed](https://zed.dev/).


## Features

- Treesitter based syntax highlighting
- Diagnostics
- Auto-completion
- Hover documentation
- Formatting


## Installation

1. Open Zed
2. Open the command palette
3. Enter `zed: extensions`
4. Search for `Verilog` using the search bar on the extensions page.
5. Press `Install` and the language server will automatically be downloaded.
## Configuration

Configuration for this extension is done in two places. First is by adding "Verilog" your settings file and using the settings documented [here](https://zed.dev/docs/configuring-zed#languages) like so:

```json
"languages": {
    "Verilog": {

    }
}
```

Secondly, you can configure the veridian language server by creating a `veridian.yml` file in your project root and using the settings documented [here](https://github.com/vivekmalneedi/veridian?tab=readme-ov-file#configuration).

> [!NOTE]
> Since this extension doesn't use the verible language server through veridian, verible specific settings in `veridian.yml` will not work.

## Acknowledgements

 - [gmlarumbe/tree-sitter-systemverilog](https://github.com/gmlarumbe/tree-sitter-systemverilog) for the treesitter grammar.
 - [nvim-treesitter/nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries/verilog) for the queries.
 - [chipsalliance/verible](https://github.com/chipsalliance/verible) for the diagnostics and formatting language server.
 - [vivekmalneedi/veridian](https://github.com/vivekmalneedi/veridian) for the rest of the language server features.
## License

This project is licensed under the [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license or the [MIT](http://opensource.org/licenses/MIT) license, at your option. See [COPYRIGHT](./COPYRIGHT) for more details.
