#![cfg_attr(not(feature = "std"), no_std)]

/// Pallet to manage the state of the docsig
pub use pallet::*;
pub use core::str;
pub use core::str::FromStr;
pub use scale_info::prelude::vec::Vec;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// we use a safe crypto hashing by blake2_128
	// Document data hash storage
	#[pallet::storage]
	#[pallet::getter(fn get_document)]
	pub(super) type Documents<T: Config> = StorageDoubleMap< _,Blake2_128Concat, T::AccountId,Blake2_128Concat, u32,Vec<u8>,ValueQuery>;

	// signature storage
    #[pallet::storage]
    #[pallet::getter(fn get_signature)]
	pub(super) type Signatures<T: Config> = StorageDoubleMap< _,Blake2_128Concat, T::AccountId,Blake2_128Concat, u32,Vec<u8>,ValueQuery>;

    // Blob (binary large objects) data is a multiple keys storage
	#[pallet::storage]
	#[pallet::getter(fn get_blob)]
	pub(super) type Blobs<T: Config> = StorageNMap<
    	_,
    	(
			NMapKey<Blake2_128Concat, T::AccountId>,
        	NMapKey<Blake2_128Concat, u32>,
        	NMapKey<Blake2_128Concat, u32>,
    	),
    	Vec<u8>,
    	ValueQuery,
	>;

	// public keys for encryption
    #[pallet::storage]
    #[pallet::getter(fn get_encryption_public_key)]
	pub(super) type EncryptionPublicKeys<T: Config> = StorageMap< _,Blake2_128Concat, T::AccountId,Vec<u8>,ValueQuery>;


	// Events definitions
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>  {
		/// Event documentation should end with an array that provides descriptive names for event
		DocumentCreated{
			account: T::AccountId, 
			documentid: u32,
			documenthash:Vec<u8>
		},     // New document has been created
        DocumentDestroyed{
			account: T::AccountId,
			documentid: u32
		},   // Document destroyed
        DocumentSigned{
			account: T::AccountId,
			documentid: u32,
			documenthash: Vec<u8>
		},   // Document signed
		NewBlobCreated{
			account: T::AccountId,
			documentid:u32,chunkid:u32
		}, 	 // new BLOB created
		BlobDestroyed{
			account: T::AccountId,
			documentid:u32,
			chunkid:u32
		},	//  A BLOB has been destroyed
		EncryptionPublicKeyStored {
			account: T::AccountId,
			publickey:Vec<u8>
		} // public key for encryption has been stored

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        /// Document address is too long, cannot be more than 128
        DocumentTooLong,
        /// Document address is too short, cannot be less than 32
        DocumentTooShort,
        /// Id cannot be zero
        IdCannotBeZero,
        /// Document not found on the blockchain
        DocumentNotFound,
        /// Document already present on the blockchain
        DocumentAlreadyPresent,
        /// Document has been already signed from the sender
        DocumentAlreadySigned,
        ///  hash is too short
        HashTooShort,
        ///  hash is too long
        HashTooLong,
		/// the data in the blob is too short it must be > 1
		BlobTooShort,
		/// the data in the blob cannot be more than 100K bytes, you should create multiple chunks for bigger blob
		BlobTooLong,
		/// the blob is already stored, you may need to increase the chunk id for additional data
		BlobAlreadyPresent,
		/// The blob is not present on chain
		BlobNotFound,
		/// The public key is too short
		PublicKeyTooShort,
		/// The public key is too long
		PublicKeyTooLong
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	#[pallet::call]
	impl<T: Config > Pallet<T> {
		
		  /// Create a new document to be signed
		  #[pallet::call_index(1)]
		  #[pallet::weight(T::WeightInfo::new_document())]
		  pub fn new_document(origin:OriginFor<T>, id: u32,document: Vec<u8>) -> DispatchResult {
				// check the request is signed
				let sender = ensure_signed(origin)?;
				//check document length
				ensure!(document.len() >= 32, Error::<T>::DocumentTooShort);
				ensure!(document.len() <= 128, Error::<T>::DocumentTooLong);
				ensure!(id>0,Error::<T>::IdCannotBeZero);
				ensure!(!Documents::<T>::contains_key(&sender,&id),Error::<T>::DocumentAlreadyPresent);
				// Insert new Document
				Documents::<T>::insert(sender.clone(),id.clone(),document.clone());
				// Generate event
				Self::deposit_event(Event::DocumentCreated{
					account:sender,
					documentid: id,
					documenthash: document
				});
				// Return a successful DispatchResult
				Ok(())
		  }
		  /// Destroy a Document
		  #[pallet::call_index(2)]
		  #[pallet::weight(T::WeightInfo::destroy_document())]
		  pub fn destroy_document(origin:OriginFor<T>,id:u32) -> DispatchResult {
				// check the request is signed
				let sender = ensure_signed(origin)?;
				// verify the document exists
				ensure!(Documents::<T>::contains_key(&sender,&id)==true, Error::<T>::DocumentNotFound);
				// Remove Document 
				Documents::<T>::take(sender.clone(),id.clone());
				// Generate event
				//it can leave orphans, anyway it's a decision of the super user
				Self::deposit_event(Event::DocumentDestroyed{
					account:sender,
					documentid: id
		  		});
				// Return a successful DispatchResult
				Ok(())
		  }
          #[pallet::call_index(3)]
		  #[pallet::weight(T::WeightInfo::sign_document())]
		  pub fn sign_document(origin:OriginFor<T>, id: u32,hash: Vec<u8>) -> DispatchResult {
				// check the request is signed
				let sender = ensure_signed(origin)?;
				//check  hash length
				ensure!(hash.len() <= 128, Error::<T>::HashTooLong);
				ensure!(hash.len() >= 32, Error::<T>::HashTooShort);
				ensure!(id>0,Error::<T>::IdCannotBeZero);
				ensure!(!Signatures::<T>::contains_key(&sender,&id),Error::<T>::DocumentAlreadySigned);
				// Insert Signature
				Signatures::<T>::insert(sender.clone(),id.clone(),hash.clone());
				// Generate event
				Self::deposit_event(Event::DocumentSigned{
					account:sender,
					documentid: id,
					documenthash: hash
				});
				// Return a successful DispatchResult
				Ok(())
		  }
		  #[pallet::call_index(4)]
		  #[pallet::weight(T::WeightInfo::sign_document())]
		  pub fn store_publickey(origin:OriginFor<T>,publickey: Vec<u8>) -> DispatchResult {
				// check the request is signed
				let sender = ensure_signed(origin)?;
				//check  public key length
				ensure!(publickey.len() <= 64, Error::<T>::PublicKeyTooLong);
				ensure!(publickey.len() >= 32, Error::<T>::PublicKeyTooShort);
				// remove the public key if exists already
				if EncryptionPublicKeys::<T>::contains_key(&sender) {
					EncryptionPublicKeys::<T>::take(sender.clone());	
				}
				//store the public key
				EncryptionPublicKeys::<T>::insert(sender.clone(),publickey.clone());	
				// Generate event
				Self::deposit_event(Event::EncryptionPublicKeyStored{
					account:sender,
					publickey: publickey
				});
				// Return a successful DispatchResult
				Ok(())
		  }
		  /// Store a BLOB (binary large object) eventually in multiple chunks of 100K bytes
		  /// the first chunk start from 0 and should increase by 1.
		  #[pallet::call_index(5)]
		  #[pallet::weight(T::WeightInfo::new_blob())]
		  pub fn new_blob(origin:OriginFor<T>, account: T::AccountId,id: u32,chunkid: u32,blob: Vec<u8>) -> DispatchResult {
				// check the request is signed
				let _sender = ensure_signed(origin)?;
				//check blob length
				ensure!(blob.len() > 1, Error::<T>::BlobTooShort);
				//the underlying rocksdb has a limit of 3 GB for the value 
				// the standard max block size in substrate is 4.5 MB, the max for parachain is 2 MB.
				// 100K should be a reasonable amount for single blob chunk
				ensure!(blob.len() <= 100000, Error::<T>::BlobTooLong); 
				// check id that cannot be <1
				ensure!(id>0,Error::<T>::IdCannotBeZero);
				// build the tuple to query the nmap
				let keyarg=&(account.clone(),id.clone(),chunkid.clone());
				//check that the same blob is not already stored
				ensure!(!Blobs::<T>::contains_key(keyarg.clone()),Error::<T>::BlobAlreadyPresent);
				// Insert the new BLOB chunk (it may be the only one if the file is smaller than 100K)
				Blobs::<T>::insert(keyarg,blob);
				// Generate event for the new Blob
				Self::deposit_event(Event::NewBlobCreated{
					account: account,
				    documentid: id,
					chunkid: chunkid
		  		});
				// Return a successful DispatchResult
				Ok(())
		  }
		  
		  /// Destroy a Blob, only the orginal creator can remove it and upon condition is not yet signed
		  #[pallet::call_index(6)]
		  #[pallet::weight(T::WeightInfo::destroy_blob())]
		  pub fn destroy_blob(origin:OriginFor<T>,account: T::AccountId,id:u32,chunkid:u32) -> DispatchResult {
				// check the request is signed
				let sender = ensure_signed(origin)?;
				//check that the matching document is not yet signed
				ensure!(!Signatures::<T>::contains_key(&sender,&id),Error::<T>::DocumentAlreadySigned);
				// build the tuple to query the nmap
				let keyarg=&(sender.clone(),id.clone(),chunkid.clone());
				// verify the blob exists and belong to the signer
				ensure!(Blobs::<T>::contains_key(keyarg.clone()),Error::<T>::BlobNotFound);
				// Remove the blob
				Blobs::<T>::take(keyarg);
				// Generate event
				Self::deposit_event(Event::BlobDestroyed{
					account:account,
					documentid: id,
					chunkid: chunkid
				});
				// Return a successful DispatchResult
				Ok(())
		  }
		  

	}
	
}