#![cfg_attr(not(feature = "std"), no_std)]

pub use core::{str, str::FromStr};
/// Pallet to manage the state of the docusign
pub use pallet::*;
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
	// Countries data Storage
	#[pallet::storage]
	#[pallet::getter(fn get_country)]
	pub(super) type Countries<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Counties data storage
	#[pallet::storage]
	#[pallet::getter(fn get_county)]
	pub(super) type Counties<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Districts data storage
	#[pallet::storage]
	#[pallet::getter(fn get_district)]
	pub(super) type Districts<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Precints data storage
	#[pallet::storage]
	#[pallet::getter(fn get_precint)]
	pub(super) type Precints<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Tellers data storage
	#[pallet::storage]
	#[pallet::getter(fn get_teller)]
	pub(super) type Tellers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Voting Session data storage
	#[pallet::storage]
	#[pallet::getter(fn get_session)]
	pub(super) type Sessions<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// we use a safe crypto hashing by blake2_128
	// Candidatesdata storage
	#[pallet::storage]
	#[pallet::getter(fn get_candidate)]
	pub(super) type Candidates<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// The administrators account allowed to create new passports one single account is allowed (+
	// sudo) a type of access is in the mapping for future usage, the passport writing access level
	// is 1
	#[pallet::storage]
	#[pallet::getter(fn get_adminaccount)]
	pub(super) type Administrators<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		u32,
		Vec<u8>,
		ValueQuery,
	>;

	// Votes Storage
	#[pallet::storage]
	#[pallet::getter(fn get_votes)]
	pub(super) type Votes<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, u32>,
			NMapKey<Blake2_128Concat, u32>,
			NMapKey<Blake2_128Concat, u32>,
			NMapKey<Blake2_128Concat, u32>,
		),
		u32,
		ValueQuery,
	>;
	// Events definitions
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		AdministratorCreated(T::AccountId, u32, Vec<u8>), // new administrator
		AdministratorDestroyed(T::AccountId, u32), // destroyed administrator
		CountryCreated(u32, Vec<u8>),              // new country created
		CountryDestroyed(u32),                     // country has been destroyed
		CountyCreated(u32, u32, Vec<u8>),          // new county created
		CountyDestroyed(u32, u32),                 // country has been destroyed
		DistrictCreated(u32, u32, Vec<u8>),        // new district created
		DistrictDestroyed(u32, u32),               // district has been destroyed
		PrecintCreated(u32, u32, Vec<u8>),         // new Precint created
		PrecintDestroyed(u32, u32),                // Precint has been destroyed
		TellerCreated(T::AccountId, Vec<u8>),      // new Teller created
		TellerDestroyed(T::AccountId),             // Teller has been destroyed
		SessionCreated(u32, Vec<u8>),              // New voting session created
		SessionDestroyed(u32),                     // Session Destroyed
		CandidateCreated(u32, Vec<u8>),            // New candidate created
		CandidateDestroyed(u32),                   // Candidate destroyed
		VotesUpdated(u32, u32, u32, u32, u32),     // Votes updated
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Id cannot be zero
		IdCannotBeZero,
		///the signer is not the administrator account neither the superuser
		SignerHasNoAccess,
		/// Account already configured as administrator
		AdministratorAlreadyPresent,
		// Administrator account not found
		AdministratorNotFound,
		/// Note is too long max 128 chars
		NoteTooLong,
		/// Country name is too short
		CountryNameTooShort,
		/// Country name is too long
		CountryNameToolong,
		/// Country id is already present
		CountryAlreadyPresent,
		/// Country id has not been found
		CountryNotFound,
		/// County name is too short
		CountyNameTooShort,
		/// County name is too long
		CountyNameToolong,
		/// Country id is already present
		CountyAlreadyPresent,
		/// Country id has not been found
		CountyNotFound,
		/// District name is too short
		DistrictNameTooShort,
		/// District name is too long
		DistrictNameToolong,
		/// District id is already present
		DistrictAlreadyPresent,
		/// District id has not been found
		DistrictNotFound,
		/// Precint name is too short
		PrecintNameTooShort,
		/// Precint name is too long
		PrecintNameToolong,
		/// Precint id is already present
		PrecintAlreadyPresent,
		/// Precint id has not been found
		PrecintNotFound,
		/// Teller name is too short
		TellerNameTooShort,
		/// Precint name is too long
		TellerNameToolong,
		/// Precint id is already present
		TellerAlreadyPresent,
		/// Precint id has not been found
		TellerNotFound,
		/// Invalid json structure
		InvalidJson,
		/// Teller name is too short minimum 1 chars
		TellerNameIsTooShort,
		/// Teller name is too long max 64 chars
		TellerNameIsTooLong,
		/// The districtid is mandatory and is missing
		MissingDistrictId,
		/// Precint/District has not been found
		PrecintDistrictNotFound,
		/// Session id already stored
		SessionAlreadyPresent,
		/// Data field is too long (max 1024 chars)
		DataTooLong,
		/// Description is too short (min 1 char)
		SessionDescriptionIsTooShort,
		/// Description is too long (max 64 chars)
		SessionDescriptionIsTooLong,
		/// Date of start is wrong
		DateStartisWrong,
		/// Date of end is wrong
		DateEndisWrong,
		/// Session Id not found
		SessionIdNotFound,
		/// Candidate Id is already stored
		CandidateAlreadyPresent,
		/// Candidate name is too short(min. 1 char)
		CandidateNameIsTooShort,
		/// Candidate name is too long (max. 64 chars)
		CandidateNameIsTooLong,
		/// Url is not valid
		UrlNotValid,
		/// Candidate Id not found
		CandidateNotFound,
		/// Teller not valid for the precintid submitted
		TellerNotValidforPrecintid,
		/// Teller not valid for the destrictid submitted
		TellerNotValidforDistrictid,
		/// Teller not valid for the sessionid submitted
		TellerNotValidforSessionid,
		/// Wrong number of votes
		WrongVotesNumber,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new country
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_country(origin: OriginFor<T>, id: u32, name: Vec<u8>) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			//check name length
			ensure!(!name.is_empty(), Error::<T>::CountryNameTooShort);
			ensure!(name.len() < 64, Error::<T>::CountryNameToolong);
			ensure!(id > 0, Error::<T>::IdCannotBeZero);
			ensure!(!Countries::<T>::contains_key(id), Error::<T>::CountryAlreadyPresent);
			// Insert new country
			Countries::<T>::insert(id, name.clone());
			// Generate event
			Self::deposit_event(Event::CountryCreated(id, name));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Country code
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_country(origin: OriginFor<T>, id: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account as signer
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// verify the entry exists
			ensure!(Countries::<T>::contains_key(id), Error::<T>::CountryNotFound);
			// Remove Entry
			Countries::<T>::take(id);
			// Generate event
			Self::deposit_event(Event::CountryDestroyed(id));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new county
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_county(
			origin: OriginFor<T>,
			id: u32,
			idcountry: u32,
			name: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			//check name length
			ensure!(!name.is_empty(), Error::<T>::CountyNameTooShort);
			ensure!(name.len() < 64, Error::<T>::CountyNameToolong);
			ensure!(id > 0, Error::<T>::IdCannotBeZero);
			ensure!(Countries::<T>::contains_key(idcountry), Error::<T>::CountryNotFound);
			ensure!(!Counties::<T>::contains_key(id, idcountry), Error::<T>::CountyAlreadyPresent);
			// Insert new county
			Counties::<T>::insert(id, idcountry, name.clone());
			// Generate event
			Self::deposit_event(Event::CountyCreated(id, idcountry, name));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a County code
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_county(origin: OriginFor<T>, id: u32, countryid: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account as signer
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// verify the entry exists
			ensure!(Counties::<T>::contains_key(id, countryid), Error::<T>::CountyNotFound);
			// Remove Entry
			Counties::<T>::take(id, countryid);
			// Generate event
			Self::deposit_event(Event::CountyDestroyed(id, countryid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new district
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_district(
			origin: OriginFor<T>,
			id: u32,
			idcounty: u32,
			name: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			//check name length
			ensure!(!name.is_empty(), Error::<T>::DistrictNameTooShort);
			ensure!(name.len() < 64, Error::<T>::DistrictNameToolong);
			ensure!(id > 0, Error::<T>::IdCannotBeZero);
			ensure!(
				!Districts::<T>::contains_key(id, idcounty),
				Error::<T>::DistrictAlreadyPresent
			);
			// Insert new District
			Districts::<T>::insert(id, idcounty, name.clone());
			// Generate event
			Self::deposit_event(Event::DistrictCreated(id, idcounty, name));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Distric code
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_district(origin: OriginFor<T>, id: u32, countyid: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account as signer
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// verify the entry exists
			ensure!(Districts::<T>::contains_key(id, countyid), Error::<T>::DistrictNotFound);
			// Remove Entry
			Districts::<T>::take(id, countyid);
			// Generate event
			Self::deposit_event(Event::DistrictDestroyed(id, countyid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new precint
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_precint(
			origin: OriginFor<T>,
			id: u32,
			iddistrict: u32,
			name: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			//check name length
			ensure!(!name.is_empty(), Error::<T>::PrecintNameTooShort);
			ensure!(name.len() < 64, Error::<T>::PrecintNameToolong);
			ensure!(id > 0, Error::<T>::IdCannotBeZero);
			ensure!(
				!Precints::<T>::contains_key(id, iddistrict),
				Error::<T>::PrecintAlreadyPresent
			);
			// Insert new Precint
			Precints::<T>::insert(id, iddistrict, name.clone());
			// Generate event
			Self::deposit_event(Event::PrecintCreated(id, iddistrict, name));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Precint code
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_precint(origin: OriginFor<T>, id: u32, districtid: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account as signer
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// verify the entry exists
			ensure!(Precints::<T>::contains_key(id, districtid), Error::<T>::PrecintNotFound);
			// Remove Entry
			Precints::<T>::take(id, districtid);
			// Generate event
			Self::deposit_event(Event::DistrictDestroyed(id, districtid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Add Tellers Account with a json structure
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_teller(
			origin: OriginFor<T>,
			account: T::AccountId,
			tellerdata: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check the same account is not already present
			ensure!(!Tellers::<T>::contains_key(account.clone()), Error::<T>::TellerAlreadyPresent);
			// check size of tellerdata field
			ensure!(tellerdata.len() < 1024, Error::<T>::DataTooLong);
			// check json structure
			ensure!(json_check_validity(tellerdata.clone()), Error::<T>::InvalidJson);
			// check name
			let name = json_get_value(tellerdata.clone(), "name".as_bytes().to_vec());
			ensure!(!name.is_empty(), Error::<T>::TellerNameIsTooShort);
			ensure!(name.len() <= 64, Error::<T>::TellerNameIsTooLong);
			// check precintid/districtid
			let precintid = json_get_value(tellerdata.clone(), "precintid".as_bytes().to_vec());
			let precintidu32 = vecu8_to_u32(precintid);
			let districtid = json_get_value(tellerdata.clone(), "districtid".as_bytes().to_vec());
			let districtidu32 = vecu8_to_u32(districtid);
			ensure!(
				Precints::<T>::contains_key(precintidu32, districtidu32),
				Error::<T>::PrecintDistrictNotFound
			);
			// check  sessionid
			let sessionid =
				json_get_complexarray(tellerdata.clone(), "sessionid".as_bytes().to_vec());
			ensure!(!sessionid.is_empty(), Error::<T>::MissingDistrictId);
			let mut x = 0;
			loop {
				let c = json_get_arrayvalue(sessionid.clone(), x);
				if c.is_empty() {
					break
				}
				let cv = vecu8_to_u32(c);
				ensure!(Sessions::<T>::contains_key(cv), Error::<T>::SessionIdNotFound);
				x += 1;
			}
			// add Teller
			Tellers::<T>::insert(account.clone(), tellerdata.clone());
			// Generate event
			Self::deposit_event(Event::TellerCreated(account, tellerdata));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy/remove Teller Account
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_teller(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check is present for the same level
			ensure!(Tellers::<T>::contains_key(account.clone()), Error::<T>::TellerNotFound);
			// remove teller
			Tellers::<T>::take(account.clone());
			// Generate event
			Self::deposit_event(Event::TellerDestroyed(account));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Add Voting Session with a json structure
		// {
		//		id
		//		description
		//		countryid
		//		countyid
		//		districtid
		//		precintid
		//		datestart
		//		dateend
		// 		url
		// }
		// for example:
		// {"description":"Presidential Elections 2023","countryid":"1","datestart":"2023-10-10","dateend":"2023-10-12","url":"https://necliberia.org"}
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_session(origin: OriginFor<T>, id: u32, sessiondata: Vec<u8>) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check the same session is not already present
			ensure!(!Sessions::<T>::contains_key(id), Error::<T>::SessionAlreadyPresent);
			// check size of sessiondata field
			ensure!(sessiondata.len() < 1024, Error::<T>::DataTooLong);
			// check json structure
			ensure!(json_check_validity(sessiondata.clone()), Error::<T>::InvalidJson);
			// check name
			let description =
				json_get_value(sessiondata.clone(), "description".as_bytes().to_vec());
			ensure!(!description.is_empty(), Error::<T>::SessionDescriptionIsTooShort);
			ensure!(description.len() <= 64, Error::<T>::SessionDescriptionIsTooLong);
			// check precintid/districtid/countyid,countryid
			let precintid = json_get_value(sessiondata.clone(), "precintid".as_bytes().to_vec());
			let precintidu32 = vecu8_to_u32(precintid);
			let districtid = json_get_value(sessiondata.clone(), "districtid".as_bytes().to_vec());
			let districtidu32 = vecu8_to_u32(districtid);
			let countyid = json_get_value(sessiondata.clone(), "countyid".as_bytes().to_vec());
			let countyidu32 = vecu8_to_u32(countyid);
			let countryid = json_get_value(sessiondata.clone(), "countryid".as_bytes().to_vec());
			let countryidu32 = vecu8_to_u32(countryid);
			if countryidu32 > 0 {
				ensure!(Countries::<T>::contains_key(countryidu32), Error::<T>::CountryNotFound);
			}
			if countyidu32 > 0 {
				ensure!(
					Counties::<T>::contains_key(countyidu32, countryidu32),
					Error::<T>::CountyNotFound
				);
			}
			if districtidu32 > 0 {
				ensure!(
					Districts::<T>::contains_key(districtidu32, countyidu32),
					Error::<T>::DistrictNotFound
				);
			}
			if precintidu32 > 0 {
				ensure!(
					Precints::<T>::contains_key(precintidu32, districtidu32),
					Error::<T>::PrecintNotFound
				);
			}
			// check datestart
			let datestart = json_get_value(sessiondata.clone(), "datestart".as_bytes().to_vec());
			ensure!(datestart.len() == 10, Error::<T>::DateStartisWrong);
			// check dateend
			let dateend = json_get_value(sessiondata.clone(), "dateend".as_bytes().to_vec());
			ensure!(dateend.len() == 10, Error::<T>::DateEndisWrong);
			//check url
			let url = json_get_value(sessiondata.clone(), "url".as_bytes().to_vec());
			ensure!(validate_weburl(url), Error::<T>::UrlNotValid);
			// add Voting Session
			Sessions::<T>::insert(id, sessiondata.clone());
			// Generate event
			Self::deposit_event(Event::SessionCreated(id, sessiondata));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Destroy/remove voting session
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_session(origin: OriginFor<T>, id: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check the session is present
			ensure!(Sessions::<T>::contains_key(id), Error::<T>::SessionIdNotFound);
			// Remove Session
			Sessions::<T>::take(id);
			// Generate event
			Self::deposit_event(Event::SessionDestroyed(id));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Add candidate to a voting session
		// {
		//		id
		//		name
		//		url
		//		sessionid
		// }
		// for example:
		// {"name":"George Weah","url":"https://necliberia.org","sessionid":"1"}
		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_candidate(
			origin: OriginFor<T>,
			id: u32,
			candidatedata: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check the same candidate is not already present
			ensure!(!Candidates::<T>::contains_key(id), Error::<T>::CandidateAlreadyPresent);
			// check size of candidatedata field
			ensure!(candidatedata.len() < 1024, Error::<T>::DataTooLong);
			// check json structure
			ensure!(json_check_validity(candidatedata.clone()), Error::<T>::InvalidJson);
			// check name
			let name = json_get_value(candidatedata.clone(), "name".as_bytes().to_vec());
			ensure!(!name.is_empty(), Error::<T>::CandidateNameIsTooShort);
			ensure!(name.len() <= 64, Error::<T>::CandidateNameIsTooLong);
			//check sessionid
			let sessionid = json_get_value(candidatedata.clone(), "sessionid".as_bytes().to_vec());
			let sessionidu32 = vecu8_to_u32(sessionid);
			ensure!(Sessions::<T>::contains_key(sessionidu32), Error::<T>::SessionIdNotFound);
			//check url
			let url = json_get_value(candidatedata.clone(), "url".as_bytes().to_vec());
			ensure!(validate_weburl(url), Error::<T>::UrlNotValid);
			// add Voting Session
			Candidates::<T>::insert(id, candidatedata.clone());
			// Generate event
			Self::deposit_event(Event::CandidateCreated(id, candidatedata));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy/remove Candidate Data
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_candidate(origin: OriginFor<T>, id: u32) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check for a valid administrator account
			ensure!(Administrators::<T>::contains_key(&sender, 1), Error::<T>::SignerHasNoAccess);
			// check candidate id
			ensure!(Candidates::<T>::contains_key(id), Error::<T>::CandidateNotFound);
			// remove candidate
			Candidates::<T>::take(id);
			// Generate event
			Self::deposit_event(Event::CandidateDestroyed(id));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Add Admin Account
		#[pallet::call_index(15)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn new_admin(
			origin: OriginFor<T>,
			account: T::AccountId,
			id: u32,
			note: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check the same account is not already poresent for the same level
			ensure!(
				!Administrators::<T>::contains_key(account.clone(), id),
				Error::<T>::AdministratorAlreadyPresent
			);
			// check size of note field
			ensure!(note.len() < 128, Error::<T>::NoteTooLong);
			// add administrator
			Administrators::<T>::insert(account.clone(), id, note.clone());
			// Generate event
			Self::deposit_event(Event::AdministratorCreated(account, id, note));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Destroy/remove Admin Account
		#[pallet::call_index(16)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_admin(
			origin: OriginFor<T>,
			account: T::AccountId,
			id: u32,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check if it is present
			ensure!(
				Administrators::<T>::contains_key(account.clone(), id),
				Error::<T>::AdministratorNotFound
			);
			// add administrator
			Administrators::<T>::take(account.clone(), id);
			// Generate event
			Self::deposit_event(Event::AdministratorDestroyed(account, id));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Update Votes from tellers call
		#[pallet::call_index(20)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn update_votes(
			origin: OriginFor<T>,
			candidateid: u32,
			sessionid: u32,
			districtid: u32,
			precintid: u32,
			votes: u32,
		) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// check the sender is a teller
			ensure!(!Tellers::<T>::contains_key(sender.clone()), Error::<T>::TellerNotFound);
			// read teller json data
			let tellerdata = Tellers::<T>::get(sender.clone());
			let precintidj = json_get_value(tellerdata.clone(), "precintid".as_bytes().to_vec());
			let precintidu32 = vecu8_to_u32(precintidj);
			let districtidj = json_get_value(tellerdata.clone(), "districtid".as_bytes().to_vec());
			let districtidu32 = vecu8_to_u32(districtidj);
			ensure!(precintidu32 == precintid, Error::<T>::TellerNotValidforPrecintid);
			ensure!(districtidu32 == districtid, Error::<T>::TellerNotValidforDistrictid);
			// check  sessionid
			let sessionidj =
				json_get_complexarray(tellerdata.clone(), "sessionid".as_bytes().to_vec());
			let mut x = 0;
			let mut flag = false;
			loop {
				let c = json_get_arrayvalue(sessionidj.clone(), x);
				if c.is_empty() {
					break
				}
				let cv = vecu8_to_u32(c);
				if cv == sessionid {
					flag = true;
					break
				}
				x += 1;
			}
			ensure!(flag, Error::<T>::TellerNotValidforSessionid);
			// check candidate id
			ensure!(!Candidates::<T>::contains_key(candidateid), Error::<T>::CandidateNotFound);
			// check number of votes
			ensure!(votes > 0, Error::<T>::WrongVotesNumber);
			// build the key
			let keyarg = &(candidateid, sessionid, districtid, precintid);
			//remove votes if present
			if Votes::<T>::contains_key(*keyarg) {
				Votes::<T>::take(*keyarg);
			}
			//store new votes
			Votes::<T>::insert(*keyarg, votes);
			// Generate event
			Self::deposit_event(Event::VotesUpdated(
				candidateid,
				sessionid,
				districtid,
				precintid,
				votes,
			));
			// Return a successful DispatchResult
			Ok(())
		}
	}
}
//*************************************************************************************
//*** functions blocks
//*************************************************************************************
// function to validate a json string for no/std. It does not allocate of memory
fn json_check_validity(j: Vec<u8>) -> bool {
	// minimum lenght of 2
	if j.len() < 2 {
		return false
	}
	// checks star/end with {}
	let first = *j.first().unwrap();
	let last = *j.last().unwrap();

	if first == b'{' && last != b'}' {
		return false
	}
	// checks start/end with []
	if first == b'[' && last != b']' {
		return false
	}
	// check that the start is { or [
	if first != b'{' && first != b'[' {
		return false
	}
	//checks that end is } or ]
	if last != b'}' && last != b']' {
		return false
	}
	//checks " opening/closing and : as separator between name and values
	let mut s: bool = true;
	let mut d: bool = true;
	let mut pg: bool = true;
	let mut ps: bool = true;
	let mut bp = b' ';
	for b in j {
		if b == b'[' && s {
			ps = false;
		}
		if b == b']' && s && !ps {
			ps = true;
		} else if b == b']' && s && ps {
			ps = false;
		}
		if b == b'{' && s {
			pg = false;
		}
		if b == b'}' && s && !pg {
			pg = true;
		} else if b == b'}' && s && pg {
			pg = false;
		}
		if b == b'"' && s && bp != b'\\' {
			s = false;
			bp = b;
			d = false;
			continue
		}
		if b == b':' && s {
			d = true;
			bp = b;
			continue
		}
		if b == b'"' && !s && bp != b'\\' {
			s = true;
			bp = b;
			d = true;
			continue
		}
		bp = b;
	}
	//fields are not closed properly
	if !s {
		return false
	}
	//fields are not closed properly
	if !d {
		return false
	}
	//fields are not closed properly
	if !ps {
		return false
	}
	// every ok returns true
	true
}
// function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the
// records is not present
fn json_get_arrayvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
	let mut result = Vec::new();
	let mut op = true;
	let mut cn = 0;
	let mut lb = b' ';
	for b in ar {
		if b == b',' && op {
			cn += 1;
			continue
		}
		if b == b'[' && op && lb != b'\\' {
			continue
		}
		if b == b']' && op && lb != b'\\' {
			continue
		}
		if b == b'"' && op && lb != b'\\' {
			continue
		}
		if b == b'"' && op && lb != b'\\' {
			op = false;
		}
		if b == b'"' && !op && lb != b'\\' {
			op = true;
		}
		// field found
		if cn == p {
			result.push(b);
		}
		lb = b;
	}
	result
}

// function to get value of a field for Substrate runtime (no std library and no variable
// allocation)
fn json_get_value(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
	let mut result = Vec::new();
	let mut k = Vec::new();
	let keyl = key.len();
	let jl = j.len();
	k.push(b'"');
	for xk in 0..keyl {
		k.push(*key.get(xk).unwrap());
	}
	k.push(b'"');
	k.push(b':');
	let kl = k.len();
	for x in 0..jl {
		let mut m = 0;
		if x + kl > jl {
			break
		}
		for (xx, i) in (x..x + kl).enumerate() {
			if *j.get(i).unwrap() == *k.get(xx).unwrap() {
				m += 1;
			}
		}
		if m == kl {
			let mut lb = b' ';
			let mut op = true;
			let mut os = true;
			for i in x + kl..jl - 1 {
				let v = *j.get(i).unwrap();
				if v == b'[' && op && os {
					os = false;
				}
				if v == b'}' && op && !os {
					os = true;
				}
				if v == b':' && op {
					continue
				}
				if v == b'"' && op && lb != b'\\' {
					op = false;
					continue
				}
				if v == b'"' && !op && lb != b'\\' {
					break
				}
				if v == b'}' && op {
					break
				}
				if v == b']' && op {
					break
				}
				if v == b',' && op && os {
					break
				}
				result.push(v);
				lb = v;
			}
			break
		}
	}
	result
}
// function to get value of a field with a complex array like [{....},{.....}] for Substrate runtime
// (no std library and no variable allocation)
fn json_get_complexarray(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
	let mut result = Vec::new();
	let mut k = Vec::new();
	let keyl = key.len();
	let jl = j.len();
	k.push(b'"');
	for xk in 0..keyl {
		k.push(*key.get(xk).unwrap());
	}
	k.push(b'"');
	k.push(b':');
	let kl = k.len();
	for x in 0..jl {
		let mut m = 0;
		if x + kl > jl {
			break
		}
		for (xx, i) in (x..x + kl).enumerate() {
			if *j.get(i).unwrap() == *k.get(xx).unwrap() {
				m += 1;
			}
		}
		if m == kl {
			let mut os = true;
			for i in x + kl..jl - 1 {
				let v = *j.get(i).unwrap();
				if v == b'[' && os {
					os = false;
				}
				result.push(v);
				if v == b']' && !os {
					break
				}
			}
			break
		}
	}
	result
}
/// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
	let vslice = v.as_slice();
	let vstr = str::from_utf8(vslice).unwrap_or("0");
	let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
	vvalue
}
// function to validate an web url return true/false
fn validate_weburl(weburl: Vec<u8>) -> bool {
	let mut valid = false;
	let mut x = 0;
	let mut httpsflag = false;
	let mut httpflag = false;
	let mut startpoint = 0;
	let https: Vec<u8> = "https://".into();
	let http: Vec<u8> = "http://".into();
	let httpscomp: Vec<u8> = weburl[0..8].into();
	let httpcomp: Vec<u8> = weburl[0..7].into();
	if https == httpscomp {
		httpsflag = true;
	}
	if http == httpcomp {
		httpflag = true;
	}
	if !httpflag && !httpsflag {
		return false
	}
	if httpsflag {
		startpoint = 8;
	}
	if httpflag {
		startpoint = 7;
	}
	for cc in weburl {
		let c: char = cc.into();
		if x < startpoint {
			x += 1;
			continue
		}
		// check for allowed chars
		if (' '..='_').contains(&c) || ('a'..='~').contains(&c) {
			valid = true;
		} else {
			valid = false;
			break
		}
	}
	valid
}
