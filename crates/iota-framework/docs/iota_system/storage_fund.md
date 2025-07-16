---
title: Module `iota_system::storage_fund`
---



-  [Struct `StorageFund`](#iota_system_storage_fund_StorageFund)
-  [Function `new`](#iota_system_storage_fund_new)
-  [Function `advance_epoch`](#iota_system_storage_fund_advance_epoch)
-  [Function `total_object_storage_rebates`](#iota_system_storage_fund_total_object_storage_rebates)
-  [Function `total_balance`](#iota_system_storage_fund_total_balance)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
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
<b>use</b> <a href="../iota/iota.md#iota_iota">iota::iota</a>;
<b>use</b> <a href="../iota/table.md#iota_table">iota::table</a>;
<b>use</b> <a href="../iota/transfer.md#iota_transfer">iota::transfer</a>;
<b>use</b> <a href="../iota/tx_context.md#iota_tx_context">iota::tx_context</a>;
<b>use</b> <a href="../iota/types.md#iota_types">iota::types</a>;
<b>use</b> <a href="../iota/url.md#iota_url">iota::url</a>;
<b>use</b> <a href="../iota/vec_set.md#iota_vec_set">iota::vec_set</a>;
</code></pre>



<a name="iota_system_storage_fund_StorageFund"></a>

## Struct `StorageFund`

Struct representing the storage fund, containing two <code>Balance</code>s:
- <code><a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a></code> has the invariant that it's the sum of <code>storage_rebate</code> of
all objects currently stored on-chain. To maintain this invariant, the only inflow of this
balance is storage charges collected from transactions, and the only outflow is storage rebates
of transactions, including both the portion refunded to the transaction senders as well as
the non-refundable portion taken out and put into <code>non_refundable_balance</code>.
- <code>non_refundable_balance</code> contains any remaining inflow of the storage fund that should not
be taken out of the fund.


<pre><code><b>public</b> <b>struct</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>non_refundable_balance: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="iota_system_storage_fund_new"></a>

## Function `new`

Called by <code><a href="../iota_system/iota_system.md#iota_system_iota_system">iota_system</a></code> at genesis time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_new">new</a>(initial_fund: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;): <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">iota_system::storage_fund::StorageFund</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_new">new</a>(initial_fund: Balance&lt;IOTA&gt;): <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a> {
    <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a> {
        // At the beginning there's no object in the storage yet
        <a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: balance::zero(),
        non_refundable_balance: initial_fund,
    }
}
</code></pre>



</details>

<a name="iota_system_storage_fund_advance_epoch"></a>

## Function `advance_epoch`

Called by <code><a href="../iota_system/iota_system.md#iota_system_iota_system">iota_system</a></code> at epoch change times to process the inflows and outflows of storage fund.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">iota_system::storage_fund::StorageFund</a>, storage_charges: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, storage_fund_reinvestment: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, leftover_staking_rewards: <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, storage_rebate_amount: u64, non_refundable_storage_fee_amount: u64): <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a>,
    storage_charges: Balance&lt;IOTA&gt;,
    storage_fund_reinvestment: Balance&lt;IOTA&gt;,
    leftover_staking_rewards: Balance&lt;IOTA&gt;,
    storage_rebate_amount: u64,
    non_refundable_storage_fee_amount: u64,
): Balance&lt;IOTA&gt; {
    // Both the reinvestment and leftover rewards are not to be refunded so they go to the non-refundable balance.
    self.non_refundable_balance.join(storage_fund_reinvestment);
    self.non_refundable_balance.join(leftover_staking_rewards);
    // The storage charges <b>for</b> the epoch come from the storage rebate of the <a href="../iota_system/storage_fund.md#iota_system_storage_fund_new">new</a> objects created
    // and the <a href="../iota_system/storage_fund.md#iota_system_storage_fund_new">new</a> storage rebates of the objects modified during the epoch so we put the charges
    // into `<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    self.<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.join(storage_charges);
    // Split out the non-refundable portion of the storage rebate and put it into the non-refundable balance.
    <b>let</b> non_refundable_storage_fee = self.<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(non_refundable_storage_fee_amount);
    self.non_refundable_balance.join(non_refundable_storage_fee);
    // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
    // should be taken out of the `<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    <b>let</b> storage_rebate = self.<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(storage_rebate_amount);
    // The storage rebate <b>has</b> already been returned to individual transaction senders' gas coins
    // so we <b>return</b> the balance to be burnt at the very end of epoch change.
    storage_rebate
}
</code></pre>



</details>

<a name="iota_system_storage_fund_total_object_storage_rebates"></a>

## Function `total_object_storage_rebates`



<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">iota_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value()
}
</code></pre>



</details>

<a name="iota_system_storage_fund_total_balance"></a>

## Function `total_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">iota_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../iota_system/storage_fund.md#iota_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../iota_system/storage_fund.md#iota_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value() + self.non_refundable_balance.value()
}
</code></pre>



</details>
