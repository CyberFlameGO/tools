# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `comments > basic > function-trailing-comma-shorthand`

### `ast`

```javascript
JSRoot {
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "comments/basic/function-trailing-comma-shorthand/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "comments/basic/function-trailing-comma-shorthand/input.js"
		end: Object {
			column: 0
			index: 47
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	comments: Array [
		CommentBlock {
			id: "0"
			value: " comment 1 "
			loc: Object {
				filename: "comments/basic/function-trailing-comma-shorthand/input.js"
				end: Object {
					column: 28
					index: 28
					line: 1
				}
				start: Object {
					column: 13
					index: 13
					line: 1
				}
			}
		}
		CommentBlock {
			id: "1"
			value: " comment 2 "
			loc: Object {
				filename: "comments/basic/function-trailing-comma-shorthand/input.js"
				end: Object {
					column: 45
					index: 45
					line: 1
				}
				start: Object {
					column: 30
					index: 30
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "comments/basic/function-trailing-comma-shorthand/input.js"
				end: Object {
					column: 46
					index: 46
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			expression: JSCallExpression {
				trailingComments: Array ["1"]
				loc: Object {
					filename: "comments/basic/function-trailing-comma-shorthand/input.js"
					end: Object {
						column: 29
						index: 29
						line: 1
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				callee: JSReferenceIdentifier {
					name: "fn"
					loc: Object {
						filename: "comments/basic/function-trailing-comma-shorthand/input.js"
						identifierName: "fn"
						end: Object {
							column: 2
							index: 2
							line: 1
						}
						start: Object {
							column: 0
							index: 0
							line: 1
						}
					}
				}
				arguments: Array [
					JSReferenceIdentifier {
						name: "a"
						loc: Object {
							filename: "comments/basic/function-trailing-comma-shorthand/input.js"
							identifierName: "a"
							end: Object {
								column: 4
								index: 4
								line: 1
							}
							start: Object {
								column: 3
								index: 3
								line: 1
							}
						}
					}
					JSObjectExpression {
						trailingComments: Array []
						loc: Object {
							filename: "comments/basic/function-trailing-comma-shorthand/input.js"
							end: Object {
								column: 11
								index: 11
								line: 1
							}
							start: Object {
								column: 6
								index: 6
								line: 1
							}
						}
						properties: Array [
							JSObjectProperty {
								key: JSStaticPropertyKey {
									value: JSIdentifier {
										name: "b"
										loc: Object {
											filename: "comments/basic/function-trailing-comma-shorthand/input.js"
											identifierName: "b"
											end: Object {
												column: 9
												index: 9
												line: 1
											}
											start: Object {
												column: 8
												index: 8
												line: 1
											}
										}
									}
									loc: Object {
										filename: "comments/basic/function-trailing-comma-shorthand/input.js"
										end: Object {
											column: 9
											index: 9
											line: 1
										}
										start: Object {
											column: 8
											index: 8
											line: 1
										}
									}
								}
								value: JSReferenceIdentifier {
									name: "b"
									loc: Object {
										filename: "comments/basic/function-trailing-comma-shorthand/input.js"
										identifierName: "b"
										end: Object {
											column: 9
											index: 9
											line: 1
										}
										start: Object {
											column: 8
											index: 8
											line: 1
										}
									}
								}
								loc: Object {
									filename: "comments/basic/function-trailing-comma-shorthand/input.js"
									end: Object {
										column: 9
										index: 9
										line: 1
									}
									start: Object {
										column: 8
										index: 8
										line: 1
									}
								}
							}
						]
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```