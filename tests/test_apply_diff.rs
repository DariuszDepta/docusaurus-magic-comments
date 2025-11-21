use docusaurus_magic_comments::{common, diff};

const INPUT: &str = r#"

bla bla bla


```rust title="tests/contract.rs"
+ use std::fmt::Error;

+ use cosmwasm_schema::cw_serde;
- use cosmwasm_std::{Addr, Coin, Empty, Event, Uint128};
+ use cosmwasm_std::{Addr, Coin, DepsMut, Empty, Env, Event, MessageInfo, Response, Uint128};
  ...
  use cw_my_collection_manager::{
-     contract::{execute, instantiate, query, reply, sudo},
-     msg::{ExecuteMsg, GetPaymentParamsResponse, InstantiateMsg, PaymentParams, QueryMsg, SudoMsg}
+     contract::{execute, instantiate, migrate, query, reply, sudo},
+     msg::{ExecuteMsg, GetPaymentParamsResponse, InstantiateMsg, MigrateMsg, PaymentParams, QueryMsg, SudoMsg},
  }
  ...
  fn test_sudo_update_payment_params() {
      ...
  }
+
+ #[test]
+ fn test_migrate_payment_params() {
+     // Arrange old smart contract
+     #[cw_serde]
+     struct OldInstantiateMsg {}
+     let mut mock_app = App::default();
+     let admin_addr = Addr::unchecked("admin");
+     let old_code = Box::new(
+         ContractWrapper::new(
+             execute,
+             |_: DepsMut, _: Env, _: MessageInfo, _: OldInstantiateMsg| -> Result<Response, Error> {
+                 Ok(Response::default())
+             },
+             query,
+         )
+         .with_reply(reply)
+         .with_sudo(sudo),
+     );
+     let manager_old_code_id = mock_app.store_code(old_code);
+     let addr_manager = mock_app
+         .instantiate_contract(
+             manager_old_code_id,
+             Addr::unchecked("deployer-manager"),
+             &OldInstantiateMsg {},
+             &[],
+             "my-collection-manager",
+             Some(admin_addr.to_string()),
+         )
+         .expect("Failed to instantiate old collection manager");
+     // Arrange migration
+     let new_code = Box::new(
+         ContractWrapper::new(execute, instantiate, query)
+             .with_reply(reply)
+             .with_sudo(sudo)
+             .with_migrate(migrate),
+     );
+     let manager_new_code_id = mock_app.store_code(new_code);
+     let beneficiary_addr = Addr::unchecked("beneficiary");
+     let new_payment_params = PaymentParams {
+         beneficiary: beneficiary_addr.to_owned(),
+         mint_price: Some(Coin {
+             denom: "silver".to_owned(),
+             amount: Uint128::from(23u16),
+         }),
+     };
+     let migrate_msg = MigrateMsg {
+         payment_params: new_payment_params.to_owned(),
+     };
+
+     // Act
+     let result = mock_app.migrate_contract(
+         admin_addr,
+         addr_manager.to_owned(),
+         &migrate_msg,
+         manager_new_code_id,
+     );
+
+     // Assert
+     assert!(result.is_ok(), "Failed to migrate the contract");
+     let result = result.unwrap();
+     let expected_migrate_event = Event::new("migrate")
+         .add_attribute("_contract_address", addr_manager.to_owned())
+         .add_attribute("code_id", "2".to_owned());
+     result.assert_event(&expected_migrate_event);
+     let expected_migrate_event2 = Event::new("wasm-my-collection-manager")
+         .add_attribute("_contract_address", addr_manager.to_owned())
+         .add_attribute("update-contract-version", "0.1.0")
+         .add_attribute("update-payment-params-beneficiary", beneficiary_addr)
+         .add_attribute("update-payment-params-mint-price-denom", "silver")
+         .add_attribute("update-payment-params-mint-price-amount", "23");
+     result.assert_event(&expected_migrate_event2);
+     let result = mock_app
+         .wrap()
+         .query_wasm_smart::<GetPaymentParamsResponse>(&addr_manager, &QueryMsg::GetPaymentParams);
+     assert!(result.is_ok(), "Failed to query payment params");
+     assert_eq!(
+         result.unwrap(),
+         GetPaymentParamsResponse {
+             payment_params: new_payment_params
+         }
+     );
+ }
```

bla bla bla

"#;

const OUTPUT: &str = r#"```rust title="tests/contract.rs"
// diff-add
+ use std::fmt::Error;

// diff-add
+ use cosmwasm_schema::cw_serde;
// diff-del
- use cosmwasm_std::{Addr, Coin, Empty, Event, Uint128};
// diff-add
+ use cosmwasm_std::{Addr, Coin, DepsMut, Empty, Env, Event, MessageInfo, Response, Uint128};
  ...
  use cw_my_collection_manager::{
// diff-del-start
-     contract::{execute, instantiate, query, reply, sudo},
-     msg::{ExecuteMsg, GetPaymentParamsResponse, InstantiateMsg, PaymentParams, QueryMsg, SudoMsg}
// diff-del-end
// diff-add-start
+     contract::{execute, instantiate, migrate, query, reply, sudo},
+     msg::{ExecuteMsg, GetPaymentParamsResponse, InstantiateMsg, MigrateMsg, PaymentParams, QueryMsg, SudoMsg},
// diff-add-end
  }
  ...
  fn test_sudo_update_payment_params() {
      ...
  }
// diff-add-start
+
+ #[test]
+ fn test_migrate_payment_params() {
+     // Arrange old smart contract
+     #[cw_serde]
+     struct OldInstantiateMsg {}
+     let mut mock_app = App::default();
+     let admin_addr = Addr::unchecked("admin");
+     let old_code = Box::new(
+         ContractWrapper::new(
+             execute,
+             |_: DepsMut, _: Env, _: MessageInfo, _: OldInstantiateMsg| -> Result<Response, Error> {
+                 Ok(Response::default())
+             },
+             query,
+         )
+         .with_reply(reply)
+         .with_sudo(sudo),
+     );
+     let manager_old_code_id = mock_app.store_code(old_code);
+     let addr_manager = mock_app
+         .instantiate_contract(
+             manager_old_code_id,
+             Addr::unchecked("deployer-manager"),
+             &OldInstantiateMsg {},
+             &[],
+             "my-collection-manager",
+             Some(admin_addr.to_string()),
+         )
+         .expect("Failed to instantiate old collection manager");
+     // Arrange migration
+     let new_code = Box::new(
+         ContractWrapper::new(execute, instantiate, query)
+             .with_reply(reply)
+             .with_sudo(sudo)
+             .with_migrate(migrate),
+     );
+     let manager_new_code_id = mock_app.store_code(new_code);
+     let beneficiary_addr = Addr::unchecked("beneficiary");
+     let new_payment_params = PaymentParams {
+         beneficiary: beneficiary_addr.to_owned(),
+         mint_price: Some(Coin {
+             denom: "silver".to_owned(),
+             amount: Uint128::from(23u16),
+         }),
+     };
+     let migrate_msg = MigrateMsg {
+         payment_params: new_payment_params.to_owned(),
+     };
+
+     // Act
+     let result = mock_app.migrate_contract(
+         admin_addr,
+         addr_manager.to_owned(),
+         &migrate_msg,
+         manager_new_code_id,
+     );
+
+     // Assert
+     assert!(result.is_ok(), "Failed to migrate the contract");
+     let result = result.unwrap();
+     let expected_migrate_event = Event::new("migrate")
+         .add_attribute("_contract_address", addr_manager.to_owned())
+         .add_attribute("code_id", "2".to_owned());
+     result.assert_event(&expected_migrate_event);
+     let expected_migrate_event2 = Event::new("wasm-my-collection-manager")
+         .add_attribute("_contract_address", addr_manager.to_owned())
+         .add_attribute("update-contract-version", "0.1.0")
+         .add_attribute("update-payment-params-beneficiary", beneficiary_addr)
+         .add_attribute("update-payment-params-mint-price-denom", "silver")
+         .add_attribute("update-payment-params-mint-price-amount", "23");
+     result.assert_event(&expected_migrate_event2);
+     let result = mock_app
+         .wrap()
+         .query_wasm_smart::<GetPaymentParamsResponse>(&addr_manager, &QueryMsg::GetPaymentParams);
+     assert!(result.is_ok(), "Failed to query payment params");
+     assert_eq!(
+         result.unwrap(),
+         GetPaymentParamsResponse {
+             payment_params: new_payment_params
+         }
+     );
+ }
// diff-add-end
```"#;

#[test]
fn _0001() {
    let lines = common::select_lines(INPUT, 5, 105);
    assert_eq!(OUTPUT, diff::apply(lines).join("\n"));
}
