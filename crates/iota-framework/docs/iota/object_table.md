---
title: Module `iota::object_table`
---

Similar to <code><a href="../iota/table.md#iota_table">iota::table</a></code>, an <code><a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code> is a map-like collection. But unlike
<code><a href="../iota/table.md#iota_table">iota::table</a></code>, the values bound to these dynamic fields _must_ be objects themselves. This allows
for the objects to still exist within in storage, which may be important for external tools.
The difference is otherwise not observable from within Move.


-  [Struct `ObjectTable`](#iota_object_table_ObjectTable)
-  [Constants](#@Constants_0)
-  [Function `new`](#iota_object_table_new)
-  [Function `add`](#iota_object_table_add)
-  [Function `borrow`](#iota_object_table_borrow)
-  [Function `borrow_mut`](#iota_object_table_borrow_mut)
-  [Function `remove`](#iota_object_table_remove)
-  [Function `contains`](#iota_object_table_contains)
-  [Function `length`](#iota_object_table_length)
-  [Function `is_empty`](#iota_object_table_is_empty)
-  [Function `destroy_empty`](#iota_object_table_destroy_empty)
-  [Function `value_id`](#iota_object_table_value_id)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../iota/address.md#iota_address">iota::address</a>;
<b>use</b> <a href="../iota/dynamic_field.md#iota_dynamic_field">iota::dynamic_field</a>;
<b>use</b> <a href="../iota/dynamic_object_field.md#iota_dynamic_object_field">iota::dynamic_object_field</a>;
<b>use</b> <a href="../iota/hex.md#iota_hex">iota::hex</a>;
<b>use</b> <a href="../iota/object.md#iota_object">iota::object</a>;
<b>use</b> <a href="../iota/tx_context.md#iota_tx_context">iota::tx_context</a>;
</code></pre>



<a name="iota_object_table_ObjectTable"></a>

## Struct `ObjectTable`



<pre><code><b>public</b> <b>struct</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;<b>phantom</b> K: <b>copy</b>, drop, store, <b>phantom</b> V: key, store&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../iota/object.md#iota_object_UID">iota::object::UID</a></code>
</dt>
<dd>
 the ID of this table
</dd>
<dt>
<code>size: u64</code>
</dt>
<dd>
 the number of key-value pairs in the table
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="iota_object_table_ETableNotEmpty"></a>



<pre><code><b>const</b> <a href="../iota/object_table.md#iota_object_table_ETableNotEmpty">ETableNotEmpty</a>: u64 = 0;
</code></pre>



<a name="iota_object_table_new"></a>

## Function `new`

Creates a new, empty table


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_new">new</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(ctx: &<b>mut</b> <a href="../iota/tx_context.md#iota_tx_context_TxContext">iota::tx_context::TxContext</a>): <a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_new">new</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(ctx: &<b>mut</b> TxContext): <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt; {
    <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a> {
        id: <a href="../iota/object.md#iota_object_new">object::new</a>(ctx),
        size: 0,
    }
}
</code></pre>



</details>

<a name="iota_object_table_add"></a>

## Function `add`

Adds a key-value pair to the table <code><a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code>
Aborts with <code><a href="../iota/dynamic_field.md#iota_dynamic_field_EFieldAlreadyExists">iota::dynamic_field::EFieldAlreadyExists</a></code> if the table already has an entry with
that key <code>k: K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_add">add</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K, v: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_add">add</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;, k: K, v: V) {
    ofield::add(&<b>mut</b> <a href="../iota/table.md#iota_table">table</a>.id, k, v);
    <a href="../iota/table.md#iota_table">table</a>.size = <a href="../iota/table.md#iota_table">table</a>.size + 1;
}
</code></pre>



</details>

<a name="iota_object_table_borrow"></a>

## Function `borrow`

Immutable borrows the value associated with the key in the table <code><a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code>.
Aborts with <code><a href="../iota/dynamic_field.md#iota_dynamic_field_EFieldDoesNotExist">iota::dynamic_field::EFieldDoesNotExist</a></code> if the table does not have an entry with
that key <code>k: K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../iota/borrow.md#iota_borrow">borrow</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/borrow.md#iota_borrow">borrow</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;, k: K): &V {
    ofield::borrow(&<a href="../iota/table.md#iota_table">table</a>.id, k)
}
</code></pre>



</details>

<a name="iota_object_table_borrow_mut"></a>

## Function `borrow_mut`

Mutably borrows the value associated with the key in the table <code><a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code>.
Aborts with <code><a href="../iota/dynamic_field.md#iota_dynamic_field_EFieldDoesNotExist">iota::dynamic_field::EFieldDoesNotExist</a></code> if the table does not have an entry with
that key <code>k: K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(
    <a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;,
    k: K,
): &<b>mut</b> V {
    ofield::borrow_mut(&<b>mut</b> <a href="../iota/table.md#iota_table">table</a>.id, k)
}
</code></pre>



</details>

<a name="iota_object_table_remove"></a>

## Function `remove`

Removes the key-value pair in the table <code><a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code> and returns the value.
Aborts with <code><a href="../iota/dynamic_field.md#iota_dynamic_field_EFieldDoesNotExist">iota::dynamic_field::EFieldDoesNotExist</a></code> if the table does not have an entry with
that key <code>k: K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_remove">remove</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_remove">remove</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<b>mut</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;, k: K): V {
    <b>let</b> v = ofield::remove(&<b>mut</b> <a href="../iota/table.md#iota_table">table</a>.id, k);
    <a href="../iota/table.md#iota_table">table</a>.size = <a href="../iota/table.md#iota_table">table</a>.size - 1;
    v
}
</code></pre>



</details>

<a name="iota_object_table_contains"></a>

## Function `contains`

Returns true if there is a value associated with the key <code>k: K</code> in table
<code><a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;</code>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_contains">contains</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_contains">contains</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;, k: K): bool {
    ofield::exists_&lt;K&gt;(&<a href="../iota/table.md#iota_table">table</a>.id, k)
}
</code></pre>



</details>

<a name="iota_object_table_length"></a>

## Function `length`

Returns the size of the table, the number of key-value pairs


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_length">length</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_length">length</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;): u64 {
    <a href="../iota/table.md#iota_table">table</a>.size
}
</code></pre>



</details>

<a name="iota_object_table_is_empty"></a>

## Function `is_empty`

Returns true if the table is empty (if <code><a href="../iota/object_table.md#iota_object_table_length">length</a></code> returns <code>0</code>)


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_is_empty">is_empty</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_is_empty">is_empty</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;): bool {
    <a href="../iota/table.md#iota_table">table</a>.size == 0
}
</code></pre>



</details>

<a name="iota_object_table_destroy_empty"></a>

## Function `destroy_empty`

Destroys an empty table
Aborts with <code><a href="../iota/object_table.md#iota_object_table_ETableNotEmpty">ETableNotEmpty</a></code> if the table still contains values


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_destroy_empty">destroy_empty</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: <a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_destroy_empty">destroy_empty</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../iota/table.md#iota_table">table</a>: <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;) {
    <b>let</b> <a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a> { id, size } = <a href="../iota/table.md#iota_table">table</a>;
    <b>assert</b>!(size == 0, <a href="../iota/object_table.md#iota_object_table_ETableNotEmpty">ETableNotEmpty</a>);
    id.delete()
}
</code></pre>



</details>

<a name="iota_object_table_value_id"></a>

## Function `value_id`

Returns the ID of the object associated with the key if the table has an entry with key <code>k: K</code>
Returns none otherwise


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_value_id">value_id</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">iota::object_table::ObjectTable</a>&lt;K, V&gt;, k: K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../iota/object.md#iota_object_ID">iota::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../iota/object_table.md#iota_object_table_value_id">value_id</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(
    <a href="../iota/table.md#iota_table">table</a>: &<a href="../iota/object_table.md#iota_object_table_ObjectTable">ObjectTable</a>&lt;K, V&gt;,
    k: K,
): Option&lt;ID&gt; {
    ofield::id(&<a href="../iota/table.md#iota_table">table</a>.id, k)
}
</code></pre>



</details>
