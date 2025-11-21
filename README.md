# Docusaurus magic comments

**Helper library for applying Docusaurus magic comments to diff files**

## Problem

When writing documentation (especially with Docusaurus), you may want to show readers how to modify existing code,
such as adding new lines or removing outdated ones. One effective way to do this is by including a diff block
that highlights the exact changes.

For example, suppose your `diff` tool produces the following output: 

```diff
  use crate::{
      error::ContractError,
-     msg::{ExecuteMsg, InstantiateMsg},
+     msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
      state::{NameRecord, NAME_RESOLVER},
  };
- use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
+ use cosmwasm_std::{
+     entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
+ };
```

Now, when you paste it do Markdown file in Docusaurus, like this:

````md
```rust
  use crate::{
      error::ContractError,
-     msg::{ExecuteMsg, InstantiateMsg},
+     msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
      state::{NameRecord, NAME_RESOLVER},
  };
- use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
+ use cosmwasm_std::{
+     entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
+ };
```
````

unfortunately it will not be highlighted as you would expect.

To highlight lines marked with `+` or `-` you need to use
[Custom magic comments](https://docusaurus.io/docs/markdown-features/code-blocks#custom-magic-comments). 

This crate helps to automate adding custom magic comments to diff code snippets.

## Usage

Let's assume that your Markdown file is named **intro.md** and looks like this:

````md
## The query function

You define the message handling in `src/contract.rs`. Adjust the imports:

```rust title="src/contract.rs"
  use crate::{
      error::ContractError,
-     msg::{ExecuteMsg, InstantiateMsg},
+     msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
      state::{NameRecord, NAME_RESOLVER},
  };
- use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
+ use cosmwasm_std::{
+     entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
+ };
```

Then, below the `execute` function, you add the query functions.
````

When you call:

```shell
$ docusaurus-magic-comments intro.md 5 16
```

then the output will be augmented with custom magic comments like shown below:

````text
```rust title="src/contract.rs"
  use crate::{
      error::ContractError,
// diff-del      
-     msg::{ExecuteMsg, InstantiateMsg},
// diff-add
+     msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
      state::{NameRecord, NAME_RESOLVER},
  };
// diff-del
- use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
// diff-add-start
+ use cosmwasm_std::{
+     entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
+ };
// diff-add-end
```
````

Just replace the original code block with this generated content and voilà, 
Docusaurus highlights lines properly!

**Does not work???!!!** Then try to configure following custom magic comments in Docusaurus:
- **diff-del**
- **diff-del-start**
- **diff-del-end**
- **diff-add**
- **diff-add-start**
- **diff-add-end**

You can find an example configuration in [this project](https://github.com/CosmWasm/cosmwasm.github.io).
