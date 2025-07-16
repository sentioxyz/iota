---
title: Module `iota_system::iota_system`
---

IOTA System State Type Upgrade Guide
<code><a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a></code> is a thin wrapper around <code>IotaSystemStateInner</code> that provides a versioned interface.
The <code><a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a></code> object has a fixed ID 0x5, and the <code>IotaSystemStateInner</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>IotaSystemStateInner</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>IotaSystemStateInner</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>IotaSystemStateInner</code> type, we need to ship a new framework that does the following:
1. Define a new <code>IotaSystemStateInner</code>type (e.g. <code>IotaSystemStateInnerV2</code>).
2. Define a data migration function that migrates the old <code>IotaSystemStateInner</code> to the new one (i.e. IotaSystemStateInnerV2).
3. Replace all uses of <code>IotaSystemStateInner</code> with <code>IotaSystemStateInnerV2</code> in both iota_system.move and iota_system_state_inner.move,
with the exception of the <code><a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_create">iota_system_state_inner::create</a></code> function, which should always return the genesis type.
4. Inside <code><a href="../iota_system/iota_system.md#iota_system_iota_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a></code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in iota/tests/framework_upgrades/mock_iota_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>IotaSystemStateInner</code> struct type that matches the new Move type, and implement the IotaSystemStateTrait.
2. Update the <code><a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a></code> struct to include the new version as a new enum variant.
3. Update the <code>get_iota_system_state</code> function to handle the new version.
To test that the upgrade will be successful, we need to modify <code>iota_system_state_production_upgrade_test</code> test in
protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
put the new framework in a new directory, and run the test to exercise the upgrade.

To upgrade Validator type, besides everything above, we also need to:
1. Define a new Validator type (e.g. ValidatorV2).
2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
3. Replace all uses of Validator with ValidatorV2 except the genesis creation function.
4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade it.
In Rust, we also need to add a new case in <code>get_validator_from_table</code>.
Note that it is possible to upgrade IotaSystemStateInner without upgrading Validator, but not the other way around.
And when we only upgrade IotaSystemStateInner, the version of Validator in the wrapper will not be updated, and hence may become
inconsistent with the version of IotaSystemStateInner. This is fine as long as we don't use the Validator version to determine
the IotaSystemStateInner version, or vice versa.


-  [Struct `IotaSystemState`](#iota_system_iota_system_IotaSystemState)
-  [Constants](#@Constants_0)
-  [Function `create`](#iota_system_iota_system_create)
-  [Function `request_add_validator_candidate`](#iota_system_iota_system_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#iota_system_iota_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#iota_system_iota_system_request_add_validator)
-  [Function `request_remove_validator`](#iota_system_iota_system_request_remove_validator)
-  [Function `request_set_gas_price`](#iota_system_iota_system_request_set_gas_price)
-  [Function `set_candidate_validator_gas_price`](#iota_system_iota_system_set_candidate_validator_gas_price)
-  [Function `request_set_commission_rate`](#iota_system_iota_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#iota_system_iota_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#iota_system_iota_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#iota_system_iota_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#iota_system_iota_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#iota_system_iota_system_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_iota`](#iota_system_iota_system_convert_to_fungible_staked_iota)
-  [Function `redeem_fungible_staked_iota`](#iota_system_iota_system_redeem_fungible_staked_iota)
-  [Function `request_withdraw_stake_non_entry`](#iota_system_iota_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#iota_system_iota_system_report_validator)
-  [Function `undo_report_validator`](#iota_system_iota_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#iota_system_iota_system_rotate_operation_cap)
-  [Function `update_validator_name`](#iota_system_iota_system_update_validator_name)
-  [Function `update_validator_description`](#iota_system_iota_system_update_validator_description)
-  [Function `update_validator_image_url`](#iota_system_iota_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#iota_system_iota_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#iota_system_iota_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#iota_system_iota_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#iota_system_iota_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#iota_system_iota_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_primary_address`](#iota_system_iota_system_update_validator_next_epoch_primary_address)
-  [Function `update_candidate_validator_primary_address`](#iota_system_iota_system_update_candidate_validator_primary_address)
-  [Function `update_validator_next_epoch_worker_address`](#iota_system_iota_system_update_validator_next_epoch_worker_address)
-  [Function `update_candidate_validator_worker_address`](#iota_system_iota_system_update_candidate_validator_worker_address)
-  [Function `update_validator_next_epoch_protocol_pubkey`](#iota_system_iota_system_update_validator_next_epoch_protocol_pubkey)
-  [Function `update_candidate_validator_protocol_pubkey`](#iota_system_iota_system_update_candidate_validator_protocol_pubkey)
-  [Function `update_validator_next_epoch_worker_pubkey`](#iota_system_iota_system_update_validator_next_epoch_worker_pubkey)
-  [Function `update_candidate_validator_worker_pubkey`](#iota_system_iota_system_update_candidate_validator_worker_pubkey)
-  [Function `update_validator_next_epoch_network_pubkey`](#iota_system_iota_system_update_validator_next_epoch_network_pubkey)
-  [Function `update_candidate_validator_network_pubkey`](#iota_system_iota_system_update_candidate_validator_network_pubkey)
-  [Function `validator_address_by_pool_id`](#iota_system_iota_system_validator_address_by_pool_id)
-  [Function `pool_exchange_rates`](#iota_system_iota_system_pool_exchange_rates)
-  [Function `active_validator_addresses`](#iota_system_iota_system_active_validator_addresses)
-  [Function `advance_epoch`](#iota_system_iota_system_advance_epoch)
-  [Function `load_system_state`](#iota_system_iota_system_load_system_state)
-  [Function `load_system_state_mut`](#iota_system_iota_system_load_system_state_mut)
-  [Function `load_inner_maybe_upgrade`](#iota_system_iota_system_load_inner_maybe_upgrade)
-  [Function `validator_voting_powers`](#iota_system_iota_system_validator_voting_powers)
-  [Function `store_execution_time_estimates`](#iota_system_iota_system_store_execution_time_estimates)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../iota/address.md#iota_address">iota::address</a>;
<b>use</b> <a href="../iota/bag.md#iota_bag">iota::bag</a>;
<b>use</b> <a href="../iota/balance.md#iota_balance">iota::balance</a>;
<b>use</b> <a href="../iota/coin.md#iota_coin">iota::coin</a>;
<b>use</b> <a href="../iota/config.md#iota_config">iota::config</a>;
<b>use</b> <a href="../iota/deny_list.md#iota_deny_list">iota::deny_list</a>;
<b>use</b> <a href="../iota/dynamic_field.md#iota_dynamic_field">iota::dynamic_field</a>;
<b>use</b> <a href="../iota/dynamic_object_field.md#iota_dynamic_object_field">iota::dynamic_object_field</a>;
<b>use</b> <a href="../iota/event.md#iota_event">iota::event</a>;
<b>use</b> <a href="../iota/hex.md#iota_hex">iota::hex</a>;
<b>use</b> <a href="../iota/object.md#iota_object">iota::object</a>;
<b>use</b> <a href="../iota/pay.md#iota_pay">iota::pay</a>;
<b>use</b> <a href="../iota/priority_queue.md#iota_priority_queue">iota::priority_queue</a>;
<b>use</b> <a href="../iota/iota.md#iota_iota">iota::iota</a>;
<b>use</b> <a href="../iota/table.md#iota_table">iota::table</a>;
<b>use</b> <a href="../iota/table_vec.md#iota_table_vec">iota::table_vec</a>;
<b>use</b> <a href="../iota/transfer.md#iota_transfer">iota::transfer</a>;
<b>use</b> <a href="../iota/tx_context.md#iota_tx_context">iota::tx_context</a>;
<b>use</b> <a href="../iota/types.md#iota_types">iota::types</a>;
<b>use</b> <a href="../iota/url.md#iota_url">iota::url</a>;
<b>use</b> <a href="../iota/vec_map.md#iota_vec_map">iota::vec_map</a>;
<b>use</b> <a href="../iota/vec_set.md#iota_vec_set">iota::vec_set</a>;
<b>use</b> <a href="../iota/versioned.md#iota_versioned">iota::versioned</a>;
<b>use</b> <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">iota_system::stake_subsidy</a>;
<b>use</b> <a href="../iota_system/staking_pool.md#iota_system_staking_pool">iota_system::staking_pool</a>;
<b>use</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund">iota_system::storage_fund</a>;
<b>use</b> <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner">iota_system::iota_system_state_inner</a>;
<b>use</b> <a href="../iota_system/validator.md#iota_system_validator">iota_system::validator</a>;
<b>use</b> <a href="../iota_system/validator_cap.md#iota_system_validator_cap">iota_system::validator_cap</a>;
<b>use</b> <a href="../iota_system/validator_set.md#iota_system_validator_set">iota_system::validator_set</a>;
<b>use</b> <a href="../iota_system/validator_wrapper.md#iota_system_validator_wrapper">iota_system::validator_wrapper</a>;
<b>use</b> <a href="../iota_system/voting_power.md#iota_system_voting_power">iota_system::voting_power</a>;
</code></pre>



<a name="iota_system_iota_system_IotaSystemState"></a>

## Struct `IotaSystemState`



<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../iota/object.md#iota_object_UID">iota::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="iota_system_iota_system_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="iota_system_iota_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="iota_system_iota_system_create"></a>

## Function `create`

Create a new IotaSystemState object and make it shared.
This function will be called only once in genesis.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_create">create</a>(id: <a href="../iota/object.md#iota_object_UID">iota::object::UID</a>, validators: vector&lt;<a href="../iota_system/validator.md#iota_system_validator_Validator">iota_system::validator::Validator</a>&gt;, <a href="../iota_system/storage_fund.md#iota_system_storage_fund">storage_fund</a>: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, protocol_version: u64, epoch_start_timestamp_ms: u64, parameters: <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_SystemParameters">iota_system::iota_system_state_inner::SystemParameters</a>, <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">stake_subsidy</a>: <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy_StakeSubsidy">iota_system::stake_subsidy::StakeSubsidy</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_create">create</a>(
    id: UID,
    validators: vector&lt;Validator&gt;,
    <a href="../iota_system/storage_fund.md#iota_system_storage_fund">storage_fund</a>: Balance&lt;IOTA&gt;,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParameters,
    <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">stake_subsidy</a>: StakeSubsidy,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_create">iota_system_state_inner::create</a>(
        validators,
        <a href="../iota_system/storage_fund.md#iota_system_storage_fund">storage_fund</a>,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
    <b>let</b> version = <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_genesis_system_state_version">iota_system_state_inner::genesis_system_state_version</a>();
    <b>let</b> <b>mut</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a> {
        id,
        version,
    };
    dynamic_field::add(&<b>mut</b> self.id, version, system_state);
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession</code> MUST be a valid signature using iota_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession].


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator_candidate">request_add_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, worker_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, net_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, primary_address: vector&lt;u8&gt;, worker_address: vector&lt;u8&gt;, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    worker_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    net_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    primary_address: vector&lt;u8&gt;,
    worker_address: vector&lt;u8&gt;,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator_candidate">request_add_validator_candidate</a>(
        pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        gas_price,
        commission_rate,
        ctx,
    )
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator">request_add_validator</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator">request_add_validator</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_validator">request_add_validator</a>(ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the iota_address in the validator).
At the end of the epoch, the <code><a href="../iota_system/validator.md#iota_system_validator">validator</a></code> object will be returned to the iota_address
of the validator.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator">request_remove_validator</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator">request_remove_validator</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_remove_validator">request_remove_validator</a>(ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_set_gas_price"></a>

## Function `request_set_gas_price`

A validator can call this entry function to submit a new gas price quote, to be
used for the reference gas price calculation at the end of the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_gas_price">request_set_gas_price</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, cap: &<a href="../iota_system/validator_cap.md#iota_system_validator_cap_UnverifiedValidatorOperationCap">iota_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_gas_price">request_set_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_gas_price">request_set_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="iota_system_iota_system_set_candidate_validator_gas_price"></a>

## Function `set_candidate_validator_gas_price`

This entry function is used to set new gas price for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, cap: &<a href="../iota_system/validator_cap.md#iota_system_validator_cap_UnverifiedValidatorOperationCap">iota_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_commission_rate">request_set_commission_rate</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_commission_rate">request_set_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake">request_add_stake</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, stake: <a href="../iota/coin.md#iota_coin_Coin">iota::coin::Coin</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake">request_add_stake</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    stake: Coin&lt;IOTA&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> staked_iota = <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper, stake, validator_address, ctx);
    transfer::public_transfer(staked_iota, ctx.sender());
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code><a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake">request_add_stake</a></code>, which returns the staked IOTA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, stake: <a href="../iota/coin.md#iota_coin_Coin">iota::coin::Coin</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota_system/staking_pool.md#iota_system_staking_pool_StakedIota">iota_system::staking_pool::StakedIota</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    stake: Coin&lt;IOTA&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedIota {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake">request_add_stake</a>(stake, validator_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, stakes: vector&lt;<a href="../iota/coin.md#iota_coin_Coin">iota::coin::Coin</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;&gt;, stake_amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    stakes: vector&lt;Coin&lt;IOTA&gt;&gt;,
    stake_amount: option::Option&lt;u64&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <b>let</b> staked_iota = self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(stakes, stake_amount, validator_address, ctx);
    transfer::public_transfer(staked_iota, ctx.sender());
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake">request_withdraw_stake</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, staked_iota: <a href="../iota_system/staking_pool.md#iota_system_staking_pool_StakedIota">iota_system::staking_pool::StakedIota</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake">request_withdraw_stake</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    staked_iota: StakedIota,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = <a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper, staked_iota, ctx);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}
</code></pre>



</details>

<a name="iota_system_iota_system_convert_to_fungible_staked_iota"></a>

## Function `convert_to_fungible_staked_iota`

Convert StakedIota into a FungibleStakedIota object.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_convert_to_fungible_staked_iota">convert_to_fungible_staked_iota</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, staked_iota: <a href="../iota_system/staking_pool.md#iota_system_staking_pool_StakedIota">iota_system::staking_pool::StakedIota</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota_system/staking_pool.md#iota_system_staking_pool_FungibleStakedIota">iota_system::staking_pool::FungibleStakedIota</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_convert_to_fungible_staked_iota">convert_to_fungible_staked_iota</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    staked_iota: StakedIota,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIota {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_convert_to_fungible_staked_iota">convert_to_fungible_staked_iota</a>(staked_iota, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_redeem_fungible_staked_iota"></a>

## Function `redeem_fungible_staked_iota`

Convert FungibleStakedIota into a StakedIota object.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_redeem_fungible_staked_iota">redeem_fungible_staked_iota</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, fungible_staked_iota: <a href="../iota_system/staking_pool.md#iota_system_staking_pool_FungibleStakedIota">iota_system::staking_pool::FungibleStakedIota</a>, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_redeem_fungible_staked_iota">redeem_fungible_staked_iota</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    fungible_staked_iota: FungibleStakedIota,
    ctx: &TxContext,
): Balance&lt;IOTA&gt; {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_redeem_fungible_staked_iota">redeem_fungible_staked_iota</a>(fungible_staked_iota, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code><a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake">request_withdraw_stake</a></code> that returns the withdrawn IOTA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, staked_iota: <a href="../iota_system/staking_pool.md#iota_system_staking_pool_StakedIota">iota_system::staking_pool::StakedIota</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    staked_iota: StakedIota,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;IOTA&gt; {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_request_withdraw_stake">request_withdraw_stake</a>(staked_iota, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_addr</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_report_validator">report_validator</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, cap: &<a href="../iota_system/validator_cap.md#iota_system_validator_cap_UnverifiedValidatorOperationCap">iota_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_report_validator">report_validator</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_report_validator">report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="iota_system_iota_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../iota_system/iota_system.md#iota_system_iota_system_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_addr</code>, or
3. the cap is not valid


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_undo_report_validator">undo_report_validator</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, cap: &<a href="../iota_system/validator_cap.md#iota_system_validator_cap_UnverifiedValidatorOperationCap">iota_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_undo_report_validator">undo_report_validator</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_undo_report_validator">undo_report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="iota_system_iota_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_rotate_operation_cap">rotate_operation_cap</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_rotate_operation_cap">rotate_operation_cap</a>(ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, name: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    name: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_name">update_validator_name</a>(name, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, description: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    description: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_description">update_validator_description</a>(description, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, image_url: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    image_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_image_url">update_validator_image_url</a>(image_url, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, project_url: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    project_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_project_url">update_validator_project_url</a>(project_url, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_primary_address"></a>

## Function `update_validator_next_epoch_primary_address`

Update a validator's narwhal primary address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_primary_address"></a>

## Function `update_candidate_validator_primary_address`

Update candidate validator's narwhal primary address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_worker_address"></a>

## Function `update_validator_next_epoch_worker_address`

Update a validator's narwhal worker address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_worker_address"></a>

## Function `update_candidate_validator_worker_address`

Update candidate validator's narwhal worker address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_protocol_pubkey"></a>

## Function `update_validator_next_epoch_protocol_pubkey`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_protocol_pubkey"></a>

## Function `update_candidate_validator_protocol_pubkey`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_worker_pubkey"></a>

## Function `update_validator_next_epoch_worker_pubkey`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_worker_pubkey"></a>

## Function `update_candidate_validator_worker_pubkey`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_validator_next_epoch_network_pubkey"></a>

## Function `update_validator_next_epoch_network_pubkey`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_update_candidate_validator_network_pubkey"></a>

## Function `update_candidate_validator_network_pubkey`

Update candidate validator's public key of network key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(
    self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="iota_system_iota_system_validator_address_by_pool_id"></a>

## Function `validator_address_by_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, pool_id: &<a href="../iota/object.md#iota_object_ID">iota::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>, pool_id: &ID): <b>address</b> {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(pool_id)
}
</code></pre>



</details>

<a name="iota_system_iota_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a staking pool. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_pool_exchange_rates">pool_exchange_rates</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, pool_id: &<a href="../iota/object.md#iota_object_ID">iota::object::ID</a>): &<a href="../iota/table.md#iota_table_Table">iota::table::Table</a>&lt;u64, <a href="../iota_system/staking_pool.md#iota_system_staking_pool_PoolTokenExchangeRate">iota_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_pool_exchange_rates">pool_exchange_rates</a>(
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    pool_id: &ID
): &Table&lt;u64, PoolTokenExchangeRate&gt;  {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_pool_exchange_rates">pool_exchange_rates</a>(pool_id)
}
</code></pre>



</details>

<a name="iota_system_iota_system_active_validator_addresses"></a>

## Function `active_validator_addresses`

Getter returning addresses of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>): vector&lt;<b>address</b>&gt; {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state">load_system_state</a>(wrapper);
    self.<a href="../iota_system/iota_system.md#iota_system_iota_system_active_validator_addresses">active_validator_addresses</a>()
}
</code></pre>



</details>

<a name="iota_system_iota_system_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_advance_epoch">advance_epoch</a>(storage_reward: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, computation_reward: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, new_epoch: u64, next_protocol_version: u64, storage_rebate: u64, non_refundable_storage_fee: u64, storage_fund_reinvest_rate: u64, reward_slashing_rate: u64, epoch_start_timestamp_ms: u64, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_advance_epoch">advance_epoch</a>(
    storage_reward: Balance&lt;IOTA&gt;,
    computation_reward: Balance&lt;IOTA&gt;,
    wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>,
    new_epoch: u64,
    next_protocol_version: u64,
    storage_rebate: u64,
    non_refundable_storage_fee: u64,
    storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                        // into storage fund, in basis point.
    reward_slashing_rate: u64, // how much rewards are slashed to punish a <a href="../iota_system/validator.md#iota_system_validator">validator</a>, in bps.
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &<b>mut</b> TxContext,
): Balance&lt;IOTA&gt; {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../iota_system/iota_system.md#iota_system_iota_system_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> storage_rebate = self.<a href="../iota_system/iota_system.md#iota_system_iota_system_advance_epoch">advance_epoch</a>(
        new_epoch,
        next_protocol_version,
        storage_reward,
        computation_reward,
        storage_rebate,
        non_refundable_storage_fee,
        storage_fund_reinvest_rate,
        reward_slashing_rate,
        epoch_start_timestamp_ms,
        ctx,
    );
    storage_rebate
}
</code></pre>



</details>

<a name="iota_system_iota_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>): &<a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_IotaSystemStateInnerV2">iota_system::iota_system_state_inner::IotaSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>): &IotaSystemStateInnerV2 {
    <a href="../iota_system/iota_system.md#iota_system_iota_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="iota_system_iota_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>): &<b>mut</b> <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_IotaSystemStateInnerV2">iota_system::iota_system_state_inner::IotaSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>): &<b>mut</b> IotaSystemStateInnerV2 {
    <a href="../iota_system/iota_system.md#iota_system_iota_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="iota_system_iota_system_load_inner_maybe_upgrade"></a>

## Function `load_inner_maybe_upgrade`



<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>): &<b>mut</b> <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_IotaSystemStateInnerV2">iota_system::iota_system_state_inner::IotaSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>): &<b>mut</b> IotaSystemStateInnerV2 {
    <b>if</b> (self.version == 1) {
        <b>let</b> v1: IotaSystemStateInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
        <b>let</b> v2 = v1.v1_to_v2();
        self.version = 2;
        dynamic_field::add(&<b>mut</b> self.id, self.version, v2);
    };
    <b>let</b> inner: &<b>mut</b> IotaSystemStateInnerV2 = dynamic_field::borrow_mut(
        &<b>mut</b> self.id,
        self.version
    );
    <b>assert</b>!(inner.system_state_version() == self.version, <a href="../iota_system/iota_system.md#iota_system_iota_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="iota_system_iota_system_validator_voting_powers"></a>

## Function `validator_voting_powers`

Returns the voting power of the active validators, values are voting power in the scale of 10000.


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>): <a href="../iota/vec_map.md#iota_vec_map_VecMap">iota::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>): VecMap&lt;<b>address</b>, u64&gt; {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_active_validator_voting_powers">iota_system_state_inner::active_validator_voting_powers</a>(self)
}
</code></pre>



</details>

<a name="iota_system_iota_system_store_execution_time_estimates"></a>

## Function `store_execution_time_estimates`

Saves the given execution time estimate blob to the IotaSystemState object, for system use
at the start of the next epoch.


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_store_execution_time_estimates">store_execution_time_estimates</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">iota_system::iota_system::IotaSystemState</a>, estimates_bytes: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_store_execution_time_estimates">store_execution_time_estimates</a>(wrapper: &<b>mut</b> <a href="../iota_system/iota_system.md#iota_system_iota_system_IotaSystemState">IotaSystemState</a>, estimates_bytes: vector&lt;u8&gt;) {
    <b>let</b> self = <a href="../iota_system/iota_system.md#iota_system_iota_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_store_execution_time_estimates">iota_system_state_inner::store_execution_time_estimates</a>(self, estimates_bytes)
}
</code></pre>



</details>
