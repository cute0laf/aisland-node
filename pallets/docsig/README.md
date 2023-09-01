# Pallet Docsig

This pallet is used from Docsig Dapp to store documents and signatures.  
You can test the [live pallet Docsig](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Ftestnet.aisland.io#/extrinsics) on Aisland's testnet.  
Here the list of the exstrinsics available and their parameters:  
  
## Store Public Key
The users are supposed to publish their own public key used for encryption. The function is:  
- docSig.storePublickey(publickey)  
where public key is the ECDSA public key in hex format (64 bytes)

## Store Documents (hash only)
The first user signing a document will sign its hash (64 bytes in hex), calling:  
- docSig.newDocument(id, document)  
where "id" is the unique document id, and "document" is the has of the file  

## Store Binary Document
If the user select "blockchain" for storage, the document data will be fully stored on blockchain calling the function:
- docSig.newBlob(id, chunckid, document)  
where "id" is the unique document id, "chunkid" is number starting from 1 and increase by 1 for each chunk of the file.
The single chunk cannot be bigger >1MB, you may store multiple chunks for bigger files.
"document" is the content of the file encoded in base64.

## Delete Binary Document
The document can be deleted from the owner, calling the function:  
- docSig.newBlob(id, chunckid)
where "id" is the unique document id, "chunkid" is number starting from 1 and increase by 1 for each chunk of the file.

## Delete Document (hash)
You can delete a document calling the function:  
- docSig.destroyDocument(id)  
Where "id" is the unique id of the document to delete, only the owner can delete it.

## Sign Document
The signatures after the the first one, have to call the function:  
- docSig.signDocument(id,hash)
Where "id" is the unique id of the document and "hash" is the hash of the file signed.

# Unit Tests
change folder to: pallet/docsig and run:  
```
cargo test
```
if everything is correct you should see something like this:  
```
  Compiling sp-runtime v7.0.0 (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling sp-version v5.0.0 (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling sp-inherents v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling sp-staking v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling sp-api v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling frame-support v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling frame-system v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.42#ff24c60a)
   Compiling pallet-docsig v0.1.1 (/Users/samuelelandi_1/src/aisland/aisland-node/pallets/docsig)
    Finished test [unoptimized + debuginfo] target(s) in 14.42s
     Running unittests src/lib.rs (/Users/samuelelandi_1/src/aisland/aisland-node/target/debug/deps/pallet_docsig-58519acb322b524f)

running 5 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::test_publickey ... ok
test tests::test_signature ... ok
test tests::test_documents ... ok
test tests::test_blob ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```






