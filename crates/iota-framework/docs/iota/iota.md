---
title: Module `iota::iota`
---

Coin<IOTA> is the token used to pay for gas in IOTA.
It has 9 decimals, and the smallest unit (10^-9) is called "nanos".


-  [Struct `IOTA`](#iota_iota_IOTA)
-  [Constants](#@Constants_0)
-  [Function `new`](#iota_iota_new)
-  [Function `transfer`](#iota_iota_transfer)


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
<b>use</b> <a href="../iota/table.md#iota_table">iota::table</a>;
<b>use</b> <a href="../iota/transfer.md#iota_transfer">iota::transfer</a>;
<b>use</b> <a href="../iota/tx_context.md#iota_tx_context">iota::tx_context</a>;
<b>use</b> <a href="../iota/types.md#iota_types">iota::types</a>;
<b>use</b> <a href="../iota/url.md#iota_url">iota::url</a>;
<b>use</b> <a href="../iota/vec_set.md#iota_vec_set">iota::vec_set</a>;
</code></pre>



<a name="iota_iota_IOTA"></a>

## Struct `IOTA`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../iota/iota.md#iota_iota_IOTA">IOTA</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="iota_iota_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../iota/iota.md#iota_iota_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="iota_iota_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../iota/iota.md#iota_iota_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="iota_iota_NANOS_PER_IOTA"></a>

The amount of Nanos per IOTA token based on the fact that nanos is
10^-9 of a IOTA token


<pre><code><b>const</b> <a href="../iota/iota.md#iota_iota_NANOS_PER_IOTA">NANOS_PER_IOTA</a>: u64 = 1000000000;
</code></pre>



<a name="iota_iota_TOTAL_SUPPLY_NANOS"></a>

The total supply of IOTA denominated in Nanos (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../iota/iota.md#iota_iota_TOTAL_SUPPLY_NANOS">TOTAL_SUPPLY_NANOS</a>: u64 = 10000000000000000000;
</code></pre>



<a name="iota_iota_TOTAL_SUPPLY_IOTA"></a>

The total supply of IOTA denominated in whole IOTA tokens (10 Billion)


<pre><code><b>const</b> <a href="../iota/iota.md#iota_iota_TOTAL_SUPPLY_IOTA">TOTAL_SUPPLY_IOTA</a>: u64 = 10000000000;
</code></pre>



<a name="iota_iota_new"></a>

## Function `new`

Register the <code><a href="../iota/iota.md#iota_iota_IOTA">IOTA</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../iota/iota.md#iota_iota_new">new</a>(ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota/balance.md#iota_balance_Balance">iota::balance::Balance</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../iota/iota.md#iota_iota_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../iota/iota.md#iota_iota_IOTA">IOTA</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../iota/iota.md#iota_iota_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../iota/iota.md#iota_iota_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (treasury, metadata) = <a href="../iota/coin.md#iota_coin_create_currency">coin::create_currency</a>(
        <a href="../iota/iota.md#iota_iota_IOTA">IOTA</a> {},
        9,
        b"<a href="../iota/iota.md#iota_iota_IOTA">IOTA</a>",
        b"IOTA",
        // TODO: add appropriate description and logo <a href="../iota/url.md#iota_url">url</a>
        b"",
        option::none(),
        ctx,
    );
    <a href="../iota/transfer.md#iota_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_iota = supply.increase_supply(<a href="../iota/iota.md#iota_iota_TOTAL_SUPPLY_NANOS">TOTAL_SUPPLY_NANOS</a>);
    supply.destroy_supply();
    total_iota
}
</code></pre>



</details>

<a name="iota_iota_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota/transfer.md#iota_transfer">transfer</a>(c: <a href="../iota/coin.md#iota_coin_Coin">iota::coin::Coin</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">iota::iota::IOTA</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../iota/transfer.md#iota_transfer">transfer</a>(c: <a href="../iota/coin.md#iota_coin_Coin">coin::Coin</a>&lt;<a href="../iota/iota.md#iota_iota_IOTA">IOTA</a>&gt;, recipient: <b>address</b>) {
    <a href="../iota/transfer.md#iota_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
