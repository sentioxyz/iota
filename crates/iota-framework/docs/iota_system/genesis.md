---
title: Module `iota_system::genesis`
---



-  [Struct `GenesisValidatorMetadata`](#iota_system_genesis_GenesisValidatorMetadata)
-  [Struct `GenesisChainParameters`](#iota_system_genesis_GenesisChainParameters)
-  [Struct `TokenDistributionSchedule`](#iota_system_genesis_TokenDistributionSchedule)
-  [Struct `TokenAllocation`](#iota_system_genesis_TokenAllocation)
-  [Constants](#@Constants_0)
-  [Function `create`](#iota_system_genesis_create)
-  [Function `allocate_tokens`](#iota_system_genesis_allocate_tokens)
-  [Function `activate_validators`](#iota_system_genesis_activate_validators)


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
<b>use</b> <a href="../iota_system/iota_system.md#iota_system_iota_system">iota_system::iota_system</a>;
<b>use</b> <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner">iota_system::iota_system_state_inner</a>;
<b>use</b> <a href="../iota_system/validator.md#iota_system_validator">iota_system::validator</a>;
<b>use</b> <a href="../iota_system/validator_cap.md#iota_system_validator_cap">iota_system::validator_cap</a>;
<b>use</b> <a href="../iota_system/validator_set.md#iota_system_validator_set">iota_system::validator_set</a>;
<b>use</b> <a href="../iota_system/validator_wrapper.md#iota_system_validator_wrapper">iota_system::validator_wrapper</a>;
<b>use</b> <a href="../iota_system/voting_power.md#iota_system_voting_power">iota_system::voting_power</a>;
</code></pre>



<a name="iota_system_genesis_GenesisValidatorMetadata"></a>

## Struct `GenesisValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/genesis.md#iota_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>image_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>project_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>iota_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>gas_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>commission_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>protocol_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proof_of_possession: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>primary_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="iota_system_genesis_GenesisChainParameters"></a>

## Struct `GenesisChainParameters`



<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/genesis.md#iota_system_genesis_GenesisChainParameters">GenesisChainParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>protocol_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>chain_start_timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_initial_distribution_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
</dd>
<dt>
<code>max_validator_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_validator_joining_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_very_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_grace_period: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="iota_system_genesis_TokenDistributionSchedule"></a>

## Struct `TokenDistributionSchedule`



<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/genesis.md#iota_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stake_subsidy_fund_nanos: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>allocations: vector&lt;<a href="../iota_system/genesis.md#iota_system_genesis_TokenAllocation">iota_system::genesis::TokenAllocation</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="iota_system_genesis_TokenAllocation"></a>

## Struct `TokenAllocation`



<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/genesis.md#iota_system_genesis_TokenAllocation">TokenAllocation</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>recipient_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_nanos: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_with_validator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Indicates if this allocation should be staked at genesis and with which validator
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="iota_system_genesis_EDuplicateValidator"></a>

The <code><a href="../iota_system/genesis.md#iota_system_genesis_create">create</a></code> function was called with duplicate validators.


<pre><code><b>const</b> <a href="../iota_system/genesis.md#iota_system_genesis_EDuplicateValidator">EDuplicateValidator</a>: u64 = 1;
</code></pre>



<a name="iota_system_genesis_ENotCalledAtGenesis"></a>

The <code><a href="../iota_system/genesis.md#iota_system_genesis_create">create</a></code> function was called at a non-genesis epoch.


<pre><code><b>const</b> <a href="../iota_system/genesis.md#iota_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>: u64 = 0;
</code></pre>



<a name="iota_system_genesis_create"></a>

## Function `create`

This function will be explicitly called once at genesis.
It will create a singleton IotaSystemState object, which contains
all the information we need in the system.


<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_create">create</a>(iota_system_state_id: <a href="../iota/object.md#iota_object_UID">iota::object::UID</a>, iota_supply: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, genesis_chain_parameters: <a href="../iota_system/genesis.md#iota_system_genesis_GenesisChainParameters">iota_system::genesis::GenesisChainParameters</a>, genesis_validators: vector&lt;<a href="../iota_system/genesis.md#iota_system_genesis_GenesisValidatorMetadata">iota_system::genesis::GenesisValidatorMetadata</a>&gt;, token_distribution_schedule: <a href="../iota_system/genesis.md#iota_system_genesis_TokenDistributionSchedule">iota_system::genesis::TokenDistributionSchedule</a>, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_create">create</a>(
    iota_system_state_id: UID,
    <b>mut</b> iota_supply: Balance&lt;IOTA&gt;,
    genesis_chain_parameters: <a href="../iota_system/genesis.md#iota_system_genesis_GenesisChainParameters">GenesisChainParameters</a>,
    genesis_validators: vector&lt;<a href="../iota_system/genesis.md#iota_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a>&gt;,
    token_distribution_schedule: <a href="../iota_system/genesis.md#iota_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>,
    ctx: &<b>mut</b> TxContext,
) {
    // Ensure this is only called at <a href="../iota_system/genesis.md#iota_system_genesis">genesis</a>
    <b>assert</b>!(ctx.epoch() == 0, <a href="../iota_system/genesis.md#iota_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>);
    <b>let</b> <a href="../iota_system/genesis.md#iota_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a> {
        stake_subsidy_fund_nanos,
        allocations,
    } = token_distribution_schedule;
    <b>let</b> subsidy_fund = iota_supply.split(stake_subsidy_fund_nanos);
    <b>let</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund">storage_fund</a> = balance::zero();
    // Create all the `Validator` structs
    <b>let</b> <b>mut</b> validators = vector[];
    <b>let</b> count = genesis_validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> <a href="../iota_system/genesis.md#iota_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> {
            name,
            description,
            image_url,
            project_url,
            iota_address,
            gas_price,
            commission_rate,
            protocol_public_key,
            proof_of_possession,
            network_public_key,
            worker_public_key,
            network_address,
            p2p_address,
            primary_address,
            worker_address,
        } = genesis_validators[i];
        <b>let</b> <a href="../iota_system/validator.md#iota_system_validator">validator</a> = <a href="../iota_system/validator.md#iota_system_validator_new">validator::new</a>(
            iota_address,
            protocol_public_key,
            network_public_key,
            worker_public_key,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            network_address,
            p2p_address,
            primary_address,
            worker_address,
            gas_price,
            commission_rate,
            ctx
        );
        // Ensure that each <a href="../iota_system/validator.md#iota_system_validator">validator</a> is unique
        <b>assert</b>!(
            !<a href="../iota_system/validator_set.md#iota_system_validator_set_is_duplicate_validator">validator_set::is_duplicate_validator</a>(&validators, &<a href="../iota_system/validator.md#iota_system_validator">validator</a>),
            <a href="../iota_system/genesis.md#iota_system_genesis_EDuplicateValidator">EDuplicateValidator</a>,
        );
        validators.push_back(<a href="../iota_system/validator.md#iota_system_validator">validator</a>);
        i = i + 1;
    };
    // Allocate tokens and staking operations
    <a href="../iota_system/genesis.md#iota_system_genesis_allocate_tokens">allocate_tokens</a>(
        iota_supply,
        allocations,
        &<b>mut</b> validators,
        ctx
    );
    // Activate all validators
    <a href="../iota_system/genesis.md#iota_system_genesis_activate_validators">activate_validators</a>(&<b>mut</b> validators);
    <b>let</b> system_parameters = <a href="../iota_system/iota_system_state_inner.md#iota_system_iota_system_state_inner_create_system_parameters">iota_system_state_inner::create_system_parameters</a>(
        genesis_chain_parameters.epoch_duration_ms,
        genesis_chain_parameters.stake_subsidy_start_epoch,
        // Validator committee parameters
        genesis_chain_parameters.max_validator_count,
        genesis_chain_parameters.min_validator_joining_stake,
        genesis_chain_parameters.validator_low_stake_threshold,
        genesis_chain_parameters.validator_very_low_stake_threshold,
        genesis_chain_parameters.validator_low_stake_grace_period,
        ctx,
    );
    <b>let</b> <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">stake_subsidy</a> = <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy_create">stake_subsidy::create</a>(
        subsidy_fund,
        genesis_chain_parameters.stake_subsidy_initial_distribution_amount,
        genesis_chain_parameters.stake_subsidy_period_length,
        genesis_chain_parameters.stake_subsidy_decrease_rate,
        ctx,
    );
    iota_system::create(
        iota_system_state_id,
        validators,
        <a href="../iota_system/storage_fund.md#iota_system_storage_fund">storage_fund</a>,
        genesis_chain_parameters.protocol_version,
        genesis_chain_parameters.chain_start_timestamp_ms,
        system_parameters,
        <a href="../iota_system/stake_subsidy.md#iota_system_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
}
</code></pre>



</details>

<a name="iota_system_genesis_allocate_tokens"></a>

## Function `allocate_tokens`



<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_allocate_tokens">allocate_tokens</a>(iota_supply: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, allocations: vector&lt;<a href="../iota_system/genesis.md#iota_system_genesis_TokenAllocation">iota_system::genesis::TokenAllocation</a>&gt;, validators: &<b>mut</b> vector&lt;<a href="../iota_system/validator.md#iota_system_validator_Validator">iota_system::validator::Validator</a>&gt;, ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_allocate_tokens">allocate_tokens</a>(
    <b>mut</b> iota_supply: Balance&lt;IOTA&gt;,
    <b>mut</b> allocations: vector&lt;<a href="../iota_system/genesis.md#iota_system_genesis_TokenAllocation">TokenAllocation</a>&gt;,
    validators: &<b>mut</b> vector&lt;Validator&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>while</b> (!allocations.is_empty()) {
        <b>let</b> <a href="../iota_system/genesis.md#iota_system_genesis_TokenAllocation">TokenAllocation</a> {
            recipient_address,
            amount_nanos,
            staked_with_validator,
        } = allocations.pop_back();
        <b>let</b> allocation_balance = iota_supply.split(amount_nanos);
        <b>if</b> (staked_with_validator.is_some()) {
            <b>let</b> validator_address = staked_with_validator.destroy_some();
            <b>let</b> <a href="../iota_system/validator.md#iota_system_validator">validator</a> = <a href="../iota_system/validator_set.md#iota_system_validator_set_get_validator_mut">validator_set::get_validator_mut</a>(validators, validator_address);
            <a href="../iota_system/validator.md#iota_system_validator">validator</a>.request_add_stake_at_genesis(
                allocation_balance,
                recipient_address,
                ctx
            );
        } <b>else</b> {
            <a href="../iota/transfer.md#iota_transfer">iota::transfer</a>(
                allocation_balance.into_coin(ctx),
                recipient_address,
            );
        };
    };
    allocations.destroy_empty();
    // Provided allocations must fully allocate the iota_supply and there
    // should be none left at this point.
    iota_supply.destroy_zero();
}
</code></pre>



</details>

<a name="iota_system_genesis_activate_validators"></a>

## Function `activate_validators`



<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;<a href="../iota_system/validator.md#iota_system_validator_Validator">iota_system::validator::Validator</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota_system/genesis.md#iota_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;Validator&gt;) {
    // Activate all <a href="../iota_system/genesis.md#iota_system_genesis">genesis</a> validators
    <b>let</b> count = validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> <a href="../iota_system/validator.md#iota_system_validator">validator</a> = &<b>mut</b> validators[i];
        <a href="../iota_system/validator.md#iota_system_validator">validator</a>.activate(0);
        i = i + 1;
    };
}
</code></pre>



</details>
