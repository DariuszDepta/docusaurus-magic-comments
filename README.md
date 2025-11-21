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
