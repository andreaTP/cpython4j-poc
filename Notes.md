
I should be able to:

- compile a minimal python interpreter statically, exposing the eval and invoke functions to wasm
- compile to Java bytecode and initialize it
- compile a "guest module code" to wasm using the same toolchain - e.g. using `extension-module`
- now I have 2 wasm payloads, an heavy one with the Python interpreter and a little one with just the user code

WIP host side:
https://github.com/dylibso/chicory/compare/main...andreaTP:chicory:cpython-test-dyn-binding

- Switch to Python 3.13 (officail build?)
- Embed the disk in the plugin payload? - looks like tools have mismatching versions, lower priority - embed the files in the resources should be enough for now
- remove the dependency on wlr-libpy and port everything to this repository: https://github.com/vmware-labs/webassembly-language-runtimes/blob/6e7674cf52edb8299bf34d4f7cb0a385c6ff728d/python/tools/wlr-libpy/README.md
