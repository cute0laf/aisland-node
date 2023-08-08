use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
// test new/destroy documents
#[test]
fn test_documents(){
	new_test_ext().execute_with(||{
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let id:u32 = 1u32.into();
		let mut document = Vec::<u8>::new();
		// generate an hash 128 bytes
		for _n in 0..128{
			document.push(b'x');
		}
		// store a document hash
		assert_ok!(DocSig::new_document(RuntimeOrigin::signed(1), id.clone(),document.clone()));
		//check stored document
		assert_eq!(DocSig::get_document(1u64,1u32),document.clone());
		// check the event generated for new document
		assert_eq!(
			last_event(),
			Event::DocumentCreated{ 
				account:1u64,
				documentid:id.clone(),
				documenthash:document.clone()
			}.into()
		);

		// try to store again the same document, it should fail
		// should fail if the document is too long
		assert_noop!(
			DocSig::new_document(RuntimeOrigin::signed(1), id.clone(),document.clone()),
			Error::<Test>::DocumentAlreadyPresent
		);
		// increase the size of document to 129, should fail 
		let mut documentfail = Vec::<u8>::new();
		for _n in 0..129{
			documentfail.push(b'x');
		}
		// should fail if the document is too long
		assert_noop!(
			DocSig::new_document(RuntimeOrigin::signed(1), 2u32,documentfail),
			Error::<Test>::DocumentTooLong
		);
		// making an hash shorter of 32 bytes, it should fail
		let mut documentfails = Vec::<u8>::new();
		for _n in 0..30{
			documentfails.push(b'x');
		}
		// should fail if the document is too long
		assert_noop!(
			DocSig::new_document(RuntimeOrigin::signed(1), 3u32,documentfails),
			Error::<Test>::DocumentTooShort
		);
		// try to store a documentid with value 0, should fail
		// should fail if the document is too long
		assert_noop!(
			DocSig::new_document(RuntimeOrigin::signed(1), 0u32,document),
			Error::<Test>::IdCannotBeZero
		);
		// destroy document
		assert_ok!(DocSig::destroy_document(RuntimeOrigin::signed(1), id.clone()));
		// check the event generated for document destroyed
		assert_eq!(
			last_event(),
			Event::DocumentDestroyed{ 
				account:1u64,
				documentid:id.clone(),
			}.into()
		);
		// destroy document a not existing document should fail
		assert_noop!(
			DocSig::destroy_document(RuntimeOrigin::signed(1), id.clone()),
			Error::<Test>::DocumentNotFound
		);

	});

}
// test signatures
#[test]
fn test_signature(){
	new_test_ext().execute_with(||{
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let id:u32 = 1u32.into();
		let mut documenthash = Vec::<u8>::new();
		// generate an hash 128 bytes
		for _n in 0..32{
			documenthash.push(b'x');
		}
		// store a document hash
		assert_ok!(DocSig::new_document(RuntimeOrigin::signed(1), id.clone(),documenthash.clone()));
		//sign document
		assert_ok!(DocSig::sign_document(RuntimeOrigin::signed(1), id.clone(),documenthash.clone()));
		// check the event generated for new signature
		assert_eq!(
			last_event(),
			Event::DocumentSigned{ 
				account:1u64,
				documentid:id.clone(),
				documenthash:documenthash.clone()
			}.into()
		);
		// try to sign an already signed document, should fail.
		assert_noop!(
			DocSig::sign_document(RuntimeOrigin::signed(1), id.clone(),documenthash.clone()),
			Error::<Test>::DocumentAlreadySigned
		);
		// sign a short hash, it should fail
		let mut documenthashs = Vec::<u8>::new();
		for _ns in 0..29{
			documenthashs.push(b'x');
		}
		assert_noop!(
			DocSig::sign_document(RuntimeOrigin::signed(1), 2u32,documenthashs),
			Error::<Test>::HashTooShort
		);
		// sign a long hash, it should fail
		let mut documenthashl = Vec::<u8>::new();
		for _nl in 0..129{
			documenthashl.push(b'x');
		}
		assert_noop!(
			DocSig::sign_document(RuntimeOrigin::signed(1), 2u32,documenthashl),
			Error::<Test>::HashTooLong
		);
		// sign a document with id=0, it should fail
		assert_noop!(
			DocSig::sign_document(RuntimeOrigin::signed(1), 0u32,documenthash),
			Error::<Test>::IdCannotBeZero
		);
	});
}
// test blobs
#[test]
fn test_blob(){
	new_test_ext().execute_with(||{
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let id:u32 = 1u32.into();
		let chunkid:u32 = 0u32.into();
		let mut document = Vec::<u8>::new();
		// generate an a document 10000 bytes
		for _n in 0..100000{
			document.push(b'x');
		}
		// store a new blob
		assert_ok!(DocSig::new_blob(RuntimeOrigin::signed(1),1u64,id.clone(),chunkid.clone(),document.clone()));
		// check the event generated for new blob
		assert_eq!(
			last_event(),
			Event::NewBlobCreated{ 
				account:1u64,
				documentid:id.clone(),
				chunkid:chunkid
			}.into()
		);
		//try to store a blob with 100001 bytes, it should fail
		let mut documentl = Vec::<u8>::new();
		// generate an a document 1 byte
		for _n in 0..100001{
			documentl.push(b'x');
		}		
		assert_noop!(
			DocSig::new_blob(RuntimeOrigin::signed(1),1u64,id.clone(),chunkid.clone(),documentl.clone()),
			Error::<Test>::BlobTooLong
		);
		//try to store a blob with 1 byte only, it should fail
		let mut documents = Vec::<u8>::new();
		// generate an a document 1 byte
		documents.push(b'x');
		assert_noop!(
			DocSig::new_blob(RuntimeOrigin::signed(1),1u64,id.clone(),chunkid.clone(),documents.clone()),
			Error::<Test>::BlobTooShort
		);
		// try to store a document id =0, it should fail
		assert_noop!(
			DocSig::new_blob(RuntimeOrigin::signed(1),1u64,0u32,chunkid.clone(),document.clone()),
			Error::<Test>::IdCannotBeZero
		);

		// delete a blob
		assert_ok!(DocSig::destroy_blob(RuntimeOrigin::signed(1),1u64,id.clone(),chunkid.clone()));
		// check the event generated for destroy blob
		assert_eq!(
			last_event(),
			Event::BlobDestroyed{ 
				account:1u64,
				documentid:id.clone(),
				chunkid:chunkid
			}.into()
		);
		// try to destroy a not existing document , it should fail
		assert_noop!(
			DocSig::destroy_blob(RuntimeOrigin::signed(1),100u64,0u32,chunkid.clone()),
			Error::<Test>::BlobNotFound
		);
		// make a signature on the same document id
		let mut documenthash = Vec::<u8>::new();
		// generate an hash 128 bytes
		for _n in 0..32{
			documenthash.push(b'x');
		}
		// store a document hash
		assert_ok!(DocSig::new_document(RuntimeOrigin::signed(1), id.clone(),documenthash.clone()));
		//sign document
		assert_ok!(DocSig::sign_document(RuntimeOrigin::signed(1), id.clone(),documenthash.clone()));
		//store a blob
		assert_ok!(DocSig::new_blob(RuntimeOrigin::signed(1),1u64,id.clone(),chunkid.clone(),document.clone()));
		// try to destroy a signed blob it should fail
		assert_noop!(
			DocSig::destroy_blob(RuntimeOrigin::signed(1),1u64,1u32,chunkid.clone()),
			Error::<Test>::DocumentAlreadySigned
		);
	});
}


