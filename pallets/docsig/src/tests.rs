use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

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

//use crate::{mock::*, BuyOrders, Error, Event, Orders};

