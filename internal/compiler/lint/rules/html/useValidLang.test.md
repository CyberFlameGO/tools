# `harness.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/lint/rules/harness.test.ts --update-snapshots` to update.

## `html/useValidLang`

### `0`

```

 lint/html/useValidLang/reject/1/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="foo"></html>
               ^^^^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `0: formatted`

```html
<html lang="foo">
</html>

```

### `1`

```

 lint/html/useValidLang/reject/2/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="ex"></html>
               ^^^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `1: formatted`

```html
<html lang="ex">
</html>

```

### `2`

```

 lint/html/useValidLang/reject/3/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="foo-bar"></html>
               ^^^^^^^^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `2: formatted`

```html
<html lang="foo-bar">
</html>

```

### `3`

```

 lint/html/useValidLang/reject/4/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="aa-zz"></html>
               ^^^^^^^

  ℹ Did you mean aa-AF?

  - aa-zz
  + aa-AF

  ℹ Or one of these?

  - aa-AL
  - aa-DZ
  - aa-AS
  - aa-AD
  - aa-AO
  - aa-AI
  - aa-AQ
  - aa-AG
  - aa-AR
  - aa-AM
  and 222 others...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠ Some diagnostics have been truncated. Use the --verbose-diagnostics flag to disable truncation.

✖ Found 1 problem

```

### `3: formatted`

```html
<html lang="aa-zz">
</html>

```

### `4`

```

 lint/html/useValidLang/reject/5/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="zz-AA"></html>
               ^^^^^^^

  ℹ Did you mean az-AF?

  - zz-AA
  + az-AF

  ℹ Or one of these?

  - az-AL
  - az-AS
  - az-AD
  - az-AO
  - az-AI
  - az-AQ
  - az-AG
  - az-AR
  - az-AM
  - az-AW
  and 37 others...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠ Some diagnostics have been truncated. Use the --verbose-diagnostics flag to disable truncation.

✖ Found 1 problem

```

### `4: formatted`

```html
<html lang="zz-AA">
</html>

```

### `5`

```

 lint/html/useValidLang/reject/6/file.html:1:11 lint/html/useValidLang ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a valid value for the lang attribute.

    <html lang="en2"></html>
               ^^^^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `5: formatted`

```html
<html lang="en2">
</html>

```

### `6`

```
✔ No known problems!

```

### `6: formatted`

```html
<html lang="en-US">
</html>

```

### `7`

```
✔ No known problems!

```

### `7: formatted`

```html
<html lang="en">
</html>

```