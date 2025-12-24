[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Customers should control where encryption context is stored

# Definitions

## Conventions used in this document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

# Background

The current keystore DDB table relies on the branch key id and version being random.

By making the branch key ID the partition key we can easily group versions together.
By using GUIDs as the branch key ID, e.g. the partition key,
we have a strong control that each branch key is unique.
However, customers want to use their own id for the branch key to simplify binding branch keys to their customers.
They could take control of creation and implement their own creation process.
As a customer did this it lead them to ask
“how can I ensure that there does not already exist a branch key for this id?”
This question is fundamental to any branch key creation process.
Since a GUID is very generally unique --
but it is possible to have such a collision.

The problem then is, in the very unlikely case of such a collision, how would the system know?
The current design would treat create request that duplicated a branch key as two concurrent version requests.
This means that there would not be an error and the create caller would have no indication of a problem.

This compounds because this means that the create caller may not have an opportunity
to raise an error before some other code attempts to use this duplicate.
Once different customer data is encrypted with duplicates,
while possible to disambiguate, it becomes a very complicated problem.
It would be much better if the system was correct by construction and there could only be a single `active` version.

# Requirements

1. Branch key ids and versions creation is deterministically unique

1. Additional hooks to authorize access to branch keys

1. An optional branch key id added to the create input

# Out of Scope

- Migration from the developer preview version
- Local policy evaluation for use of branch keys

# Design questions

_Preferred options are identified like this._

## How can duplicate branch keys be rejected by construction?

### _Make the `active` version identifier part of the sort key._

This would mean that there is a single active sort key for a branch key.
Because the partition key/sort key is now a unique value this means that dynamoDB enforces uniqueness naturally.
By enforcing a consistent write that has a conditional update that requires that this partition/sort key not exist it enforces both uniqueness and creation,
as opposed to update of an existing branch key.

### Query the table for the existing branch key

Here we would query only the partition key.
By only returning the partition key and sort keys we can can be comfortable that we will always return all records in every successful case.
This is because 1 DDB read unit can read an item up to 4KB, much more than ~200 bytes for 6 GUIDs (3 branch key/version records).

However, this function has a fundamental race.
Even if the read is strongly consistent, while this means that there will be a winner, this still means that we have created duplicate records.
The system will eventually recover, but there exists a possibility that some other caller may encrypt values.
Since the hierarchy has a cache, this problem may not even be small.

It is significantly better for the request to be as atomic as possible.

## How do we distinguish active from non-active versions?

### _Insert 2 records, the active record and the version record_

Here we insert the version record and that data is immutable.
Then we also insert a “version” that is the active version.
Having an immutable record is a nice property.
The downside here is that there are 2 records to insert.
The other advantage here is that getting this record by version is easy.
This is important because on decrypt we get the branch key by version and not by “active”.

### ReEncrypt the active version to no longer be active

Here we would insert a single record and then reEncrypt this value to be no longer active.
This means that the transaction to update is more complicated.
I think that it is still possible to update everything, but now there is a single copy of the branch key that is getting updated.
It seems impossible, but not ever updating a row in the database is the safest way to go.
Especially if there is ever a need to recover, this record always exists.

## How do we create these two ciphertext blobs (existing `enc` attribute) for the two inserted records?

### _Generate the immutable version item and reEncrypt for the active item_

In this case we would GenerateDataKeyWithoutPlaintext to generate the immutable version, and then reEncrypt this data key to generate the active version.

The selection of what version is active is a privileged operation.

The downside here, is that KMS does not offer a way to verify that these two data keys are the same without decrypting them.
But because we are handling the creation here, this is correct by construction.
This becomes a problem for a caller not involved in the creation process.

### Use the same ciphertext blob and not authenticate the fact that this version is active

This has some nice features but the biggest argument against this is that it makes changing what version is active into a non-privileged operation.
Anyone with access to the table can now take any version of this branch key and make that version active by updating the record correctly.
It is nice that we would no longer require reEncrypt.

## How do customers enforce correct access to branch keys?

### Encryption context policy control

KMS already has extensive hooks for policy control over encryption context.
By extending this feature to branch keys there is a clear policy hook for decrypting the branch key on the KMS side.
By extending these encryption context values to the use of this branch key we create a strong control for customer about how their branch keys are accessed, used, and reasoned about.

Let’s look at an extreme case of how this is useful.
Say we have a branch key id collision for some reason.
Perhaps it is because a customer has chosen to insert custom branch key ids, or they are replicating regionally isolated key stores together.

However it occurs the system is intending to have two distinct branch keys.
Given the changes above, any such collision will fail.
But what if this failure does not halt the system correctly?
For example, two different customers `A` and `B` want to encrypt data with branch key `foo`.
Unknown to `B` the creation of `foo`\_b failed and the `foo` branch key is for `A`.
What would happen is a request of `foo` would succeed for both and `B`’s data is incorrect.

By having encryption context associated with the branch the access of these keys can be controlled by this information.

A point of complexity here is that there are two policy evaluations that are relevant here.
First the evaluation to get access to the branch key and second on how the branch key is used.
In practical terms this is a KMS policy evaluation to unwrap the branch key.
This gets logged in cloudTrail and give visibility to how and where the branch key is being used.

The second is when the branch key is used to encrypt/decrypt.
When encrypting plaintext this is the requirements for the encryption context associated with the plaintext.
This second policy is out of scope for this document
and can be handled as an additional feature in the future.

## What kinds of encryption context conditions should exist for access to a branch key?

### _Only equality_

This is obviously the simplest.
AWS KMS is the party doing the evaluation.

### Local policy evaluation

Having access to the branch key be done by the keystore is too late.
The secret has already been unwrapped.

## How is this encryption context for access to the branch key added to a branch key?

### _In the create branch key function, add an optional parameter for encryption context_

This value MUST be immutable to the branch key.
Changing these values changes the authorization for previously encrypted messages.

### Also add on new version

This complicates things greatly.
It is more flexible and could still be immutable but merging these two sets could be tricky to reason about.
This is a feature that can be added later.

## How should the encryption context for access be serialized into the keystore item?

### _Prefix the custom values_

This complicates writing policies for these custom values slightly.
Customers need to know these strings.
But this disambiguates several ideas.
First it means that any custom value can not collide with an existing value.
Even if the customer selects to use our custom prefix this is not a problem in our case since we are not using a match but a strict “starts with”.
Second it means that on the KMS side it is very clear when a policy evaluation applies to a branch key vs a specific KMS data key.
This even works if the customer is using our prefix, since branch keys would duplicate this value and the KMS policy evaluation is strict equality.
Third the prefix can also be used to distinguish equality from starts with.
This further makes the KMS policy evaluation clearer.

### Prefix the standard values

This makes the values going to KMS very clean.
But it also means that we can’t tell as clearly which CloudTrail logs are for branch keys, by these custom values.
It is clearly possible to find by checking for the standard values, but customers will likely only check things they know about.
Also this make less reservation in the customer names since it is harder for them to collide with the standard names, but what if the customer wants to use our standard prefix?

### Store the custom values in a single attribute

This makes writing a policy based on these values overly complicated.
Since there is no way to have granular control of the values.
A customer may want to require several custom key/values.
But different roles have difference accesses.
KMS policy control is only on strict equality so combining all this information make it easy for our development, but more complicated for customers to use.

## How is the encryption context for access propagated to versions?

### _Authenticate the values from the current active key_

We do not want to give the versioning process access to the key, so decrypt is out.
But we already need reEncrypt to create the active entry.
We could reEncrypt the existing entry to authenticate the existing encryption context.
This means performing an additional correctness step, but this process is not in the hot path and does not occur frequently.
If this reEncrypt does not change the encryption context of the new blob, then the scope of policy protecting the branch keys can not be escaped.
This means that additional policy control could be written to constrain this process so that it can only reEncrypt from a version item to an active item and from active item to the same active item.
But this policy is not required to function, it is just an exercise in least privilege.

The idea here would be

1. Get the current item (if no active item exists fail)
2. ReEncrypt this item to encryption context identical to the existing encryption context.
3. All encryption context is now authenticated
4. Construct a `GenerateDataKeyWithoutPlaintext` request with the now authenticated additional values
5. ReEncrypt this new version so that a new Active item can be inserted
6. Insert both new items as a transaction

### Get the values from an unauthenticated meta entry

No, having this data be unauthenticated, but then add it to an authenticated process is crazy.

### Just use the existing values on the active entry without authentication

No, this is less dangerous than just using unauthenticated data, but it creates an authentication race. Having a way to inject unauthenticated data into an authenticated process is a bad idea.

### Get the values from an authenticated meta entry

The authentication could be done with decrypt, but this would give decrypt to the create/rotation process.
This could be constrained with policy, but this is not correct by construction.
If a customer does not apply this policy then the process is more privileged.
If it uses the same reEncrypt as above, then what additional value does this provide?
It also creates a new value that is not a branch key that is now floating around.

## How can we let customers control the branch key id?

### _Require that encryption context MUST also be provided when creating with a defined branch key id_

Since the customer is providing the id value, they know what the value is and can therefore construct the requirement around it.
Even if these values are literally the same, this adds values.
Since then the unsafe condition is when the customer system has duplicates.
Since creating a duplicate will fail, if they continue from there we have to assume some level of shared responsibility.
Since a customer _could_ just use an existing branch key guid.

### An optional branch key id on create

Simplify given the option is possible.
This makes the feature “easy to use” but it is not “hard to misuse”.

### The option is still not safe

If duplicates are created in a single key store there is a problem.
Even GUIDs do not 1000% remove this possibility, however the new structure does.
Even a duplicate GUID would fail in a single key store.
Given this, a very poor id selection would also fail.
This leads us to our second problem, distributed duplicates.
If the entire system is somehow split brain and a duplicate exists on both sides.
Simple GUIDs does not save this case. There needs to be further information.
And the encryption context constraints provide these controls.

Finally, several customers have asked for this.
With at least one taking on implementing this as custom work.
It is better for us to provide the safest API we can and document the sharp edge.
As opposed to trying to not support this or talk about it.

# One-Way Doors

- API changes live forever.
- Changing the database format renders existing keystores invalid

# Security Considerations

This creates a mechanism
to enforce access to branch keys.
Explicit encryption context verification
is more secure than implicit encryption context verification.

# Impact/Other considerations

Explaining this subtlety of encryption context
is going to increase the complexity of our documentation.
We SHOULD build additional workshops
to help customers explore how to build
their encryption context.
