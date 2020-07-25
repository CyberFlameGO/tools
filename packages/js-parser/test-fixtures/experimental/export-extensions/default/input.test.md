# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > export-extensions > default`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "experimental/export-extensions/default/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array []
	loc: Object {
		filename: "experimental/export-extensions/default/input.js"
		end: Object {
			column: 0
			index: 22
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/default/input.js"
				end: Object {
					column: 21
					index: 21
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			source: JSStringLiteral {
				value: "test"
				loc: Object {
					filename: "experimental/export-extensions/default/input.js"
					end: Object {
						column: 20
						index: 20
						line: 1
					}
					start: Object {
						column: 14
						index: 14
						line: 1
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/default/input.js"
					end: Object {
						column: 8
						index: 8
						line: 1
					}
					start: Object {
						column: 7
						index: 7
						line: 1
					}
				}
				exported: JSIdentifier {
					name: "A"
					loc: Object {
						filename: "experimental/export-extensions/default/input.js"
						identifierName: "A"
						end: Object {
							column: 8
							index: 8
							line: 1
						}
						start: Object {
							column: 7
							index: 7
							line: 1
						}
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```