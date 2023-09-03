#![cfg_attr(not(feature = "std"), no_std)]

pub use core::{str, str::FromStr};
/// Pallet to manage the state of the market place
pub use pallet::*;
use scale_info::prelude::vec;
pub use scale_info::prelude::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use bs58;
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
	pub type Balance = u128;

	// we use a safe crypto hashing by blake2_128
	// Seller data storage
	#[pallet::storage]
	#[pallet::getter(fn get_seller)]
	pub(super) type Sellers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_iso_country)]
	pub(super) type IsoCountries<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_products_department)]
	pub(super) type ProductDepartments<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_products_category)]
	pub(super) type ProductCategories<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_currency)]
	pub(super) type Currencies<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_products_color)]
	pub(super) type ProductColors<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_products_size)]
	pub(super) type ProductSizes<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_product)]
	pub(super) type Products<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_shipper)]
	pub(super) type Shippers<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_shipper_rate)]
	pub(super) type ShippingRates<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_iso_dialcode)]
	pub(super) type IsoDialcode<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_login_data)]
	pub(super) type LoginData<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_email_account)]
	pub(super) type EmailAccount<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn get_encrypted_seed)]
	pub(super) type EmailEncryptedSeed<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_product_model)]
	pub(super) type ProductModels<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_brand)]
	pub(super) type Brands<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_manufacturer)]
	pub(super) type Manufacturers<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<u8>, ValueQuery>;

	// Events definitions
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		MarketPlaceIsoCountryCreated(Vec<u8>, Vec<u8>), // New Iso contry code has been created
		MarketPlaceIsoCountryDestroyed(Vec<u8>), // Iso contry code has been destroyed
		MarketPlaceDepartmentCreated(u32, Vec<u8>), // New department created
		MarketPlaceDepartmentDestroyed(u32),     // Department has been destroyed/removed
		MarketPlaceCategoryCreated(u32, u32, Vec<u8>), // New producct category has been created
		MarketPlaceCategoryDestroyed(u32, u32),  // Product category has been destroyed
		MarketPlaceSellerCreated(T::AccountId, Vec<u8>), // New seller has been created
		MarketPlaceSellerDestroyed(T::AccountId), // Seller destroyed
		MarketPlaceProductUpdated(u32, Vec<u8>), // A product has been created or updated
		MarketPlaceIsoDialCodeCreated(Vec<u8>, Vec<u8>), // New country dial code has been created
		MarketPlaceIsoDialCodeDestroyed(Vec<u8>), // A country dial code has been destroyed
		MarketPlaceCurrencyCodeCreated(Vec<u8>, Vec<u8>), // A new currency has been created
		MarketPlaceCurrencyDestroyed(Vec<u8>),   // A currency has been destroyed
		MarketPlaceColorCreated(u32, Vec<u8>),   // A new color has been created
		MarketPlaceColorDestroyed(u32),          // A color has been removed
		MarketPlaceSizeCreated(u32, Vec<u8>),    // A new size table has been created
		MarketPlaceSizeDestroyed(u32),           // A size table has been removed
		MarketPlaceLoginDataCreated(Vec<u8>, Vec<u8>, T::AccountId), // A new login data has been created
		MarketPlaceLoginDataDestroyed(Vec<u8>),  // A login data has been destroyed
		MarketPlaceLoginPwdChanged(Vec<u8>, Vec<u8>), // password changed
		MarketPlaceManufacturerCreated(u32, Vec<u8>), // A new manufacturer has been created
		MarketPlaceManufacturerDestroyed(u32),   // A manufacturer has been removed
		MarketPlaceShipperCreated(u32, Vec<u8>), // A new shipper has been created
		MarketPlaceShipperDestroyed(u32),        // A shipper has been removed
		MarketShippingRateCreated(u32, Vec<u8>), // A new shipping rate has been created
		MarketShippingRateDestroyed(u32),        // A shipping rate has been removed
		MarketPlaceBrandCreated(u32, Vec<u8>),   // A new brand has been created
		MarketPlaceBrandDestroyed(u32),          // A brand has been removed
		MarketPlaceProductModelCreated(u32, Vec<u8>), // a new product model has been created
		MarketPlaceProductModelDestroyed(u32),   // a product model has been removed
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Uid cannot be zero
		UidCannotBeZero,
		/// Configuration data is too short
		ConfigurationTooShort,
		/// Configuration data is too long
		ConfigurationTooLong,
		/// Seller is already present
		SellerAlreadyPresent,
		/// Invalid json sintax
		InvalidJson,
		/// Department Description is too short, it should be > 3 bytes
		DepartmentDescriptionTooShort,
		// Department Description is too long, it should be < 128 bytes
		DepartmentDescriptionTooLong,
		/// Department Id cannot be equale to zero
		DepartmentUidCannotBeZero,
		/// Department is already present on chain
		DepartmentAlreadyPresent,
		/// Department not found on chain
		DepartmentNotFound,
		/// Category ID cannot be equal to zero
		CategoryUidCannotBeZero,
		/// Category Description is too short
		CategoryDescriptionTooShort,
		/// Category Description is too long
		CategoryDescriptionTooLong,
		/// Category has not been found
		CategoryNotFound,
		/// Product category is already present on chain
		ProductCategoryAlreadyPresent,
		/// Product category not found on chain
		ProductCategoryNotFound,
		/// The country code is wrong, it must be long 2 bytes
		WrongLengthCountryCode,
		/// The country name is too short, it must be >=3
		CountryNameTooShort,
		/// Country code already present on chain
		CountryCodeAlreadyPresent,
		/// Country code not found on chain
		CountryCodeNotFound,
		/// International Dial code is too short it must be at the least 2 bytes
		DialcodeTooShort,
		/// Seller type can be 1 for Company, 2 for Professional, 3 for Private
		SellerTypeInvalid,
		/// Seller name is too short, it must be at least 5 bytes
		SellerNameTooShort,
		/// The Sellet city is too short, it mut be at the least 5 bytes
		SellerCityTooShort,
		/// The Sellet city is too short, it mut be at the least 5 bytes
		SellerCityTooLong,
		/// The seller address is too long, maximum 128 bytes
		SellerAddressTooLong,
		/// The seller zip code is too long, maximum 12 bytes
		SellerZipCodeTooLong,
		/// Po Box is too long, maximum 64 bytes
		SellerPoBoxTooLong,
		/// Seller certification description is too short, must be > 3 bytes
		SellerCertificationDescriptionTooShort,
		/// Seller certification description is too long, must be <= 64 bytes
		SellerCertificationDescriptionTooLong,
		/// Seller certificate verification is too short
		SellerCertificateVerificationTooShort,
		/// Seller certificate verification is too long
		SellerCertificateVerificationTooLong,
		/// Seller info email is wrong
		SellerInfoEmailIsWrong,
		/// Seller support email is wrong
		SellerSupportEmailIsWrong,
		/// Phone description is too short, it should be at the least 4 bytes
		SellerPhoneDescriptionTooShort,
		/// Phone description is too long, maximum 64 bytes
		SellerPhoneDescriptionTooLong,
		/// Phone number is too short at the least > 3 bytes
		SellerPhoneNumberTooShort,
		/// Phone number is too long, maximum 21 bytes
		SellerPhoneNumberTooLong,
		/// Categories of product/service sold from seller is missing
		SellerCategoriesMissing,
		/// Included countries for shipment are missing at the least "countries":[], should be set
		SellercountriesMissing,
		/// the inout fiels is not set for the the country, it should be 0 for included, 1 for
		/// excluded country with default worldwide
		IncludedExcludedCountryValueIsMissing,
		/// The latitude of the center point for the shipment area, is missing
		ShipmentAreaCenterLatitudeIsMissing,
		/// The longitude of the center point for the shipment area, is missing
		ShipmentAreaCenterLongitudeIsMissing,
		/// The latitude of the border point for the shipment area, is missing
		ShipmentAreaBorderLatitudeIsMissing,
		/// The longitude of the border point for the shipment area, is missing
		ShipmentAreaBorderLongituteIsMissing,
		/// Seller Social Url is wrong
		SellerSocialUrlIsWrong,
		/// Seller web site is wrong
		SellerWebsiteUrlIsWrong,
		/// Seller the url for certificate verification is wrong
		SellerCertificationUrlIsWrong,
		/// Seller phone number is wrong
		SellerPhoneNumberIsWrong,
		/// Seller data has not been found on chain
		SellerDataNotFound,
		/// Seller Default language is wrong
		SellerDefaultLanguageIsWrong,
		/// Seller default unit of measurement is wrong
		SellerDefaultUnitMeasurementIsWrong,
		/// Default return policy in days cannot be more than 10 years
		DefaultReturnPolicyIsExcessive,
		/// Currency code should be between 2 and 5 bytes
		WrongLengthCurrencyCode,
		/// Currency name is too short, at the last 3 bytes are required
		CurrencyNameTooShort,
		/// Currency name is too long, maximum allowed are 32 bytes
		CurrencyNameTooLong,
		/// Currency category can be "c" for crypto currency like Bitcoin or "f" for fiat/national
		/// currency like USD
		CurrencyCategoryIswrong,
		/// Blockchain name is too short, minimum 3 bytes
		BlockchainNameTooShort,
		/// Blockchain name is too long, maximum 32 bytes
		BlockchainNameTooLong,
		/// Currency code is already present
		CurrencyCodeAlreadyPresent,
		/// Currency code has not been found
		CurrencyCodeNotFound,
		/// Product Description is too short, minimum 10 bytes
		ProductDescriptionTooShort,
		/// Product Description is too long, maximum 64 bytes
		ProductDescriptionTooLong,
		/// Product Long Description is too short, minimum 64 bytes
		ProductLongDescriptionTooShort,
		/// Product Long Description is too long, maximum 4096 bytes
		ProductLongDescriptionTooLong,
		/// Product price must be > zero
		ProductPriceCannotBeZero,
		/// Specification must be >0 and < 8192
		SpecificationsIsdInvalid,
		/// Media files cannot be empty, high quality description is required.
		MediaCannotBeEmpty,
		/// Media Description is wrong, cannot be empty
		MediaDescriptionIswrong,
		/// Media filename is wrong, cannot be empty
		MediaFileNameIsWrong,
		/// Media Ipfs Address is wrong
		MediaIpfsAddressIsWrong,
		/// Color Uid cannot be zero
		ColorUidCannotBeZero,
		/// Color Description is too short
		ColorDescriptionTooShort,
		/// Color Description is too long
		ColorDescriptionTooLong,
		/// Color already present with the same uid
		ColorAlreadyPresent,
		/// Color not found
		ColorNotFound,
		/// Size Uid cannot be Zero
		SizeUidCannotBeZero,
		/// Size info cannot be > 8192 bytes
		SizeInfoTooLong,
		/// Size already present with the same UID
		SizeAlreadyPresent,
		/// Size has not been found
		SizeNotFound,
		/// Size code is missing
		SizeCodeIsMissing,
		/// Size description is missing
		SizeDescriptionIsMissing,
		/// Size area is missing
		SizeAreaIsMissing,
		/// Manufacturer Id cannot be empty
		ManufacturerUidCannotBeZero,
		/// Manufacturer name must be minimum 4 bytes
		ManufacturerNameIsTooShort,
		/// Manufacturer name can be maximum 64 bytes
		ManufacturerNameIsTooLong,
		/// Manufacturer website must be minimum 4 bytes
		ManufacturerWebsiteIsTooShort,
		/// Manufacturer name can be maximum 128 bytes
		ManufacturerWebsiteIsTooLong,
		/// Manufacturer is already present
		ManufacturerAlreadyPresent,
		/// Manufacturer has not been found
		ManufacturerNotFound,
		/// Brand Id cannot be empty
		BrandUidCannotBeZero,
		/// Brand name must be minimum 4 bytes
		BrandNameIsTooShort,
		/// Brand name can be maximum 64 bytes
		BrandNameIsTooLong,
		/// Manufacturer is already present
		BrandAlreadyPresent,
		/// Brand has not been found
		BrandNotFound,
		/// Model id cannot be empty or zero
		ModelUidCannotBeZero,
		/// Model name must be minimum 3 bytes
		ModelNameIsTooShort,
		/// Model name must be maximum 32 bytes
		ModelNameIsTooLong,
		/// Model is already present
		ModelAlreadyPresent,
		/// Model has not been found
		ModelNotfound,
		/// Shipper Id cannot be zero/empty
		ShipperUidCannotBeZero,
		/// Shipper name must be longer than 3 bytes
		ShipperNameIsTooShort,
		/// Shipper name cannot be longer than 64 bytes
		ShipperNameIsTooLong,
		/// Shipper website must be longer than 4 bytes
		ShipperWebsiteIsTooShort,
		/// Shipper website cannot be longer than 64 bytes
		ShipperWebsiteIsTooLong,
		/// Shipper is already present
		ShipperAlreadyPresent,
		/// Country of origin is not present on chain
		OriginCountryNotPresent,
		/// Destination country is not present on chain
		DestinationCountryNotPresent,
		/// Json field is too long
		JsonIsTooLong,
		/// Shipper has not been found
		ShipperNotFound,
		/// Shipping rate ID cannot be zero/empty
		ShippingRateUidCannotBeZero,
		/// Shipper id is missing
		ShipperIdIsMissing,
		/// From kg field is missing
		FromKgIsMissing,
		/// To kg field is missing
		ToKgIsMissing,
		/// Shipping rate cannot be zero
		ShippingRateCannotbeZero,
		/// Shipping rate cannot be found
		ShippingRatesNotFound,
		/// Dimension lenght size is wrong
		DimensionWrongLength,
		/// Dimension wide size is wrong
		DimensionWrongWide,
		/// Dimension height size is wrong
		DimensionWrongHeight,
		/// Dimension weight size is wrong
		DimensionWrongWeight,
		/// UPC code is missing or is wrong
		UniversalProductCodeIsWrong,
		/// Center Latitude of the area is missing
		CenterLatitudeIsMissing,
		/// Center Longitude of the area is missing
		CenterLongitudeIsMissing,
		/// Border Latitude of the area is missing
		BorderLatitudeIsMissing,
		/// Center Longitude of the area is missing
		BorderLongitudeIsMissing,
		/// Invalid Api Url, should be an https or http address
		InvalidApiUrl,
		/// Language code is wrong
		LanguageCodeIsWrong,
		/// Wrong lenght for the Encrypted Email
		WrongLengthEmailHash,
		/// Wrong lenght for the Encrypted Password
		WrongLengthEncryptedPassword,
		/// Email hash is already present on chain
		EmailHashAlreadyPresent,
		// Email hash has not been found on chain
		EmailHashNotFound,
		/// Signer of transaction is not authorized to execute it
		SignerIsNotAuthorized,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		/// Create a new Iso country code and name
		pub fn create_iso_country(
			origin: OriginFor<T>,
			countrycode: Vec<u8>,
			countryname: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// check country code length == 2
			ensure!(countrycode.len() == 2, Error::<T>::WrongLengthCountryCode);
			// check country name length  >= 3
			ensure!(countryname.len() >= 3, Error::<T>::CountryNameTooShort);
			// check the country is not alreay present on chain
			ensure!(
				!IsoCountries::<T>::contains_key(&countrycode),
				Error::<T>::CountryCodeAlreadyPresent
			);
			//let project = Projects::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;
			// store the Iso Country Code and Name
			IsoCountries::<T>::insert(countrycode.clone(), countryname.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceIsoCountryCreated(countrycode, countryname));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy an Iso country code and name
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_iso_country(origin: OriginFor<T>, countrycode: Vec<u8>) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// verify the country code exists
			ensure!(IsoCountries::<T>::contains_key(&countrycode), Error::<T>::CountryCodeNotFound);
			// Remove country code
			IsoCountries::<T>::take(countrycode.clone());
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceIsoCountryDestroyed(countrycode));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new product department
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_product_department(
			origin: OriginFor<T>,
			uid: u32,
			description: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::DepartmentUidCannotBeZero);
			//check description length
			ensure!(description.len() > 3, Error::<T>::DepartmentDescriptionTooShort);
			ensure!(description.len() < 128, Error::<T>::DepartmentDescriptionTooLong);
			// check the department is not alreay present on chain
			ensure!(
				!ProductDepartments::<T>::contains_key(uid),
				Error::<T>::DepartmentAlreadyPresent
			);
			// store the department
			ProductDepartments::<T>::insert(uid, description.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceDepartmentCreated(uid, description));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a product department
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_product_department(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the department exists
			ensure!(ProductDepartments::<T>::contains_key(uid), Error::<T>::DepartmentNotFound);
			// Remove department
			ProductDepartments::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceDepartmentDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new product category
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_product_category(
			origin: OriginFor<T>,
			uiddepartment: u32,
			uidcategory: u32,
			description: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// check uid department >0
			ensure!(uiddepartment > 0, Error::<T>::DepartmentUidCannotBeZero);
			// check uid category >0
			ensure!(uidcategory > 0, Error::<T>::CategoryUidCannotBeZero);
			//check description length
			ensure!(description.len() > 3, Error::<T>::CategoryDescriptionTooShort);
			ensure!(description.len() < 128, Error::<T>::CategoryDescriptionTooLong);
			// check the department is  alreay present on chain
			ensure!(
				ProductDepartments::<T>::contains_key(uiddepartment),
				Error::<T>::DepartmentNotFound
			);
			// check the department/category is not alreay present on chain
			ensure!(
				!ProductCategories::<T>::contains_key(uiddepartment, uidcategory),
				Error::<T>::ProductCategoryAlreadyPresent
			);
			// store the department
			ProductCategories::<T>::insert(uiddepartment, uidcategory, description.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceCategoryCreated(
				uiddepartment,
				uidcategory,
				description,
			));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a product category
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_product_category(
			origin: OriginFor<T>,
			uiddepartment: u32,
			uidcategory: u32,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// verify the department/category exists
			ensure!(
				ProductCategories::<T>::contains_key(uiddepartment, uidcategory),
				Error::<T>::ProductCategoryNotFound
			);
			// Remove department
			ProductCategories::<T>::take(uiddepartment, uidcategory);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceCategoryDestroyed(uiddepartment, uidcategory));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new seller
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_update_seller(
			origin: OriginFor<T>,
			configuration: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let mut sender = ensure_signed(origin)?;
			let originalsigner = sender.clone();
			//check configuration length
			ensure!(configuration.len() > 12, Error::<T>::ConfigurationTooShort);
			ensure!(configuration.len() < 8192, Error::<T>::ConfigurationTooLong);
			// check json validity
			ensure!(json_check_validity(configuration.clone()), Error::<T>::InvalidJson);
			// checking seller type 1= Company, 2= Freelancer, 3= Individual, 4 == Government Agency
			// 4 == NGO
			let sellertype =
				json_get_value(configuration.clone(), "sellertype".as_bytes().to_vec());
			let sellertypeu32 = vecu8_to_u32(sellertype);
			ensure!(
				sellertypeu32 == 1 ||
					sellertypeu32 == 2 || sellertypeu32 == 3 ||
					sellertypeu32 == 4 || sellertypeu32 == 5,
				Error::<T>::SellerTypeInvalid
			);
			// checking company name or name/surname
			let sellername = json_get_value(configuration.clone(), "name".as_bytes().to_vec());
			ensure!(sellername.len() > 5, Error::<T>::SellerNameTooShort);
			// address we check for maximum lenght of 128 bytes
			let selleraddress =
				json_get_value(configuration.clone(), "address".as_bytes().to_vec());
			ensure!(selleraddress.len() < 128, Error::<T>::SellerAddressTooLong);
			// zip code we check for maximum lenght of 12 bytes
			let sellerzip = json_get_value(configuration.clone(), "zip".as_bytes().to_vec());
			ensure!(sellerzip.len() < 13, Error::<T>::SellerZipCodeTooLong);
			// checking the city minimum 3 bytes
			let sellerpobox = json_get_value(configuration.clone(), "pobox".as_bytes().to_vec());
			ensure!(sellerpobox.len() < 64, Error::<T>::SellerPoBoxTooLong);
			// checking the city minimum 3 bytes
			let sellercity = json_get_value(configuration.clone(), "city".as_bytes().to_vec());
			ensure!(sellercity.len() > 5, Error::<T>::SellerCityTooShort);
			ensure!(sellercity.len() < 64, Error::<T>::SellerCityTooLong);
			// checking websites
			let websites =
				json_get_complexarray(configuration.clone(), "websites".as_bytes().to_vec());
			if !websites.is_empty() {
				let mut x = 0;
				loop {
					let w = json_get_recordvalue(websites.clone(), x);
					if w.is_empty() {
						break
					}
					let weburl = json_get_value(w.clone(), "weburl".as_bytes().to_vec());
					ensure!(aisland_validate_weburl(weburl), Error::<T>::SellerWebsiteUrlIsWrong);
					x += 1;
				}
			}
			// checking social url
			let socialurls =
				json_get_complexarray(configuration.clone(), "socialurls".as_bytes().to_vec());
			if !socialurls.is_empty() {
				let mut x = 0;
				loop {
					let w = json_get_recordvalue(socialurls.clone(), x);
					if w.is_empty() {
						break
					}
					let socialurl = json_get_value(w.clone(), "socialurl".as_bytes().to_vec());
					ensure!(aisland_validate_weburl(socialurl), Error::<T>::SellerSocialUrlIsWrong);
					x += 1;
				}
			}
			// checking certifications
			let certifications =
				json_get_complexarray(configuration.clone(), "certifications".as_bytes().to_vec());
			if !certifications.is_empty() {
				let mut x = 0;
				loop {
					let w = json_get_recordvalue(certifications.clone(), x);
					if w.is_empty() {
						break
					}
					let certificationdescription =
						json_get_value(configuration.clone(), "description".as_bytes().to_vec());
					let certificateverificationurl = json_get_value(
						configuration.clone(),
						"verificationurl".as_bytes().to_vec(),
					);
					ensure!(
						certificationdescription.len() > 3,
						Error::<T>::SellerCertificationDescriptionTooShort
					);
					ensure!(
						certificationdescription.len() <= 64,
						Error::<T>::SellerCertificationDescriptionTooLong
					);
					ensure!(
						certificateverificationurl.len() > 3,
						Error::<T>::SellerCertificateVerificationTooShort
					);
					ensure!(
						certificateverificationurl.len() <= 64,
						Error::<T>::SellerCertificateVerificationTooLong
					);
					ensure!(
						aisland_validate_weburl(certificateverificationurl.clone()),
						Error::<T>::SellerCertificationUrlIsWrong
					);
					x += 1;
				}
			}
			// checking emailinfo
			let emailinfo = json_get_value(configuration.clone(), "emailinfo".as_bytes().to_vec());
			ensure!(emailinfo.len() > 5, Error::<T>::SellerInfoEmailIsWrong);
			ensure!(aisland_validate_email(emailinfo.clone()), Error::<T>::SellerInfoEmailIsWrong);
			// checking email support
			let emailsupport =
				json_get_value(configuration.clone(), "emailsupport".as_bytes().to_vec());
			ensure!(emailsupport.len() > 5, Error::<T>::SellerSupportEmailIsWrong);
			ensure!(
				aisland_validate_email(emailsupport.clone()),
				Error::<T>::SellerSupportEmailIsWrong
			);

			// checking phone numbers
			let phones = json_get_complexarray(configuration.clone(), "phones".as_bytes().to_vec());
			if !phones.is_empty() {
				let mut x = 0;
				loop {
					let w = json_get_recordvalue(phones.clone(), x);
					if w.is_empty() {
						break
					}
					let phonedescription = json_get_value(
						configuration.clone(),
						"phonedescription".as_bytes().to_vec(),
					);
					let phonenumber =
						json_get_value(configuration.clone(), "phonebumber".as_bytes().to_vec());
					ensure!(phonedescription.len() > 3, Error::<T>::SellerPhoneDescriptionTooShort);
					ensure!(
						phonedescription.len() <= 64,
						Error::<T>::SellerPhoneDescriptionTooLong
					);
					ensure!(phonenumber.len() > 3, Error::<T>::SellerPhoneNumberTooShort);
					ensure!(phonenumber.len() <= 23, Error::<T>::SellerPhoneNumberTooLong);
					ensure!(
						aisland_validate_phonenumber(phonenumber),
						Error::<T>::SellerPhoneNumberIsWrong
					);
					x += 1;
				}
			}
			// checking categories of products/services with the department
			let categories =
				json_get_complexarray(configuration.clone(), "categories".as_bytes().to_vec());
			ensure!(!categories.is_empty(), Error::<T>::SellerCategoriesMissing);
			let mut x = 0;
			let mut nc = 0;
			loop {
				let c = json_get_recordvalue(categories.clone(), x);
				if c.is_empty() {
					break
				}
				let category =
					json_get_value(configuration.clone(), "category".as_bytes().to_vec());
				let department =
					json_get_value(configuration.clone(), "department".as_bytes().to_vec());
				let categoryu32 = vecu8_to_u32(category);
				let departmentu32 = vecu8_to_u32(department);
				ensure!(
					ProductCategories::<T>::contains_key(categoryu32, departmentu32),
					Error::<T>::ProductCategoryNotFound
				);
				x += 1;
				nc += 1;
			}
			// check that we have at least one valid product category
			ensure!(nc > 0, Error::<T>::SellerCategoriesMissing);
			// checking included countries of shipment, if not set means worldwide less the excluded
			// countries
			let countries =
				json_get_complexarray(configuration.clone(), "countries".as_bytes().to_vec());
			ensure!(!countries.is_empty(), Error::<T>::SellercountriesMissing);
			let mut x = 0;
			loop {
				let c = json_get_recordvalue(countries.clone(), x);
				if c.is_empty() {
					break
				}
				let country = json_get_value(configuration.clone(), "country".as_bytes().to_vec());
				let inout = json_get_value(configuration.clone(), "inout".as_bytes().to_vec());
				let inoutv = vecu8_to_u32(inout);
				ensure!(IsoCountries::<T>::contains_key(country), Error::<T>::CountryCodeNotFound);
				ensure!(
					inoutv == 0 || inoutv == 1,
					Error::<T>::IncludedExcludedCountryValueIsMissing
				);
				x += 1;
			}
			// check that we have at least one valid country
			ensure!(x > 0, Error::<T>::SellerCategoriesMissing);
			// delivery area can be delimited by GPS coordinates where a first point is the center
			// of a circle and second point is the border of the same circle this is useful if a
			// service/product can be delivered only around a certain place
			let shipmentarea =
				json_get_complexarray(configuration.clone(), "shipmentarea".as_bytes().to_vec());
			if !shipmentarea.is_empty() {
				let centerlatitude =
					json_get_value(shipmentarea.clone(), "centerlatitude".as_bytes().to_vec());
				let centerlongitude =
					json_get_value(shipmentarea.clone(), "centerlongitude".as_bytes().to_vec());
				let borderlatitude =
					json_get_value(shipmentarea.clone(), "borderlatitude".as_bytes().to_vec());
				let borderlongitude =
					json_get_value(shipmentarea.clone(), "borderlongitude".as_bytes().to_vec());
				ensure!(
					!centerlatitude.is_empty(),
					Error::<T>::ShipmentAreaCenterLatitudeIsMissing
				);
				ensure!(
					!centerlongitude.is_empty(),
					Error::<T>::ShipmentAreaCenterLongitudeIsMissing
				);
				ensure!(
					!borderlatitude.is_empty(),
					Error::<T>::ShipmentAreaBorderLatitudeIsMissing
				);
				ensure!(
					!borderlongitude.is_empty(),
					Error::<T>::ShipmentAreaBorderLongituteIsMissing
				);
			}
			// check for optional default language
			let defaultlanguage =
				json_get_value(configuration.clone(), "defaultlanguage".as_bytes().to_vec());
			if !defaultlanguage.is_empty() {
				ensure!(
					aisland_validate_languagecode(defaultlanguage),
					Error::<T>::SellerDefaultLanguageIsWrong
				);
			}
			// check for optional default unit of measurement
			let defaultunitmeasurement =
				json_get_value(configuration.clone(), "defaultunitmeasurement".as_bytes().to_vec());
			if !defaultunitmeasurement.is_empty() {
				ensure!(
					aisland_validate_unitmeasurement(defaultunitmeasurement),
					Error::<T>::SellerDefaultUnitMeasurementIsWrong
				);
			}
			// check for default return policy in days
			let defaultreturnpolicy =
				json_get_value(configuration.clone(), "defaultreturnpolicy".as_bytes().to_vec());
			if !defaultreturnpolicy.is_empty() {
				let drp = vecu8_to_u32(defaultreturnpolicy);
				ensure!(drp < 3650, Error::<T>::DefaultReturnPolicyIsExcessive);
			}
			// check for optional seller account (when the signer acts as a proxy for gasless
			// transactions)
			let selleraccount =
				json_get_value(configuration.clone(), "selleraccount".as_bytes().to_vec());
			if !selleraccount.is_empty() {
				let selleraccountv = bs58::decode(selleraccount).into_vec().unwrap();
				let selleraccountid = T::AccountId::decode(&mut &selleraccountv[1..33]).unwrap();
				sender = selleraccountid;
			}
			//store seller on chain
			if !Sellers::<T>::contains_key(&sender) {
				// Insert new seller
				Sellers::<T>::insert(sender.clone(), configuration.clone());
			} else {
				// check for proxy account before updating
				if originalsigner != sender {
					let settings = Sellers::<T>::get(sender.clone());
					let proxyaccount =
						json_get_value(settings.clone(), "proxyaccount".as_bytes().to_vec());
					ensure!(!proxyaccount.is_empty(), Error::<T>::SignerIsNotAuthorized);
					let proxyaccountv = bs58::decode(proxyaccount).into_vec().unwrap();
					let proxyaccountid = T::AccountId::decode(&mut &proxyaccountv[1..33]).unwrap();
					ensure!(proxyaccountid == originalsigner, Error::<T>::SignerIsNotAuthorized);
				}
				// Replace Seller Data
				Sellers::<T>::take(sender.clone());
				Sellers::<T>::insert(sender.clone(), configuration.clone());
			}
			// Generate event
			Self::deposit_event(Event::MarketPlaceSellerCreated(sender, configuration));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Seller
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_seller(origin: OriginFor<T>) -> DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			// verify the seller exists
			ensure!(Sellers::<T>::contains_key(&sender), Error::<T>::SellerDataNotFound);
			// Remove Seller
			Sellers::<T>::take(sender.clone());
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceSellerDestroyed(sender));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create/update a Product
		/// Example:
		/// {"description":"xxxx","longdescription","xxxx","price":1000,"currencycode","USDC"}
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_update_product(
			origin: OriginFor<T>,
			uid: u32,
			configuration: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let _sender = ensure_signed(origin)?;
			//check configuration length
			ensure!(configuration.len() > 12, Error::<T>::ConfigurationTooShort);
			ensure!(configuration.len() < 65536, Error::<T>::ConfigurationTooLong);
			// check json validity
			ensure!(json_check_validity(configuration.clone()), Error::<T>::InvalidJson);
			// check for mandatory short description
			let description =
				json_get_value(configuration.clone(), "description".as_bytes().to_vec());
			ensure!(description.len() >= 10, Error::<T>::ProductDescriptionTooShort);
			ensure!(description.len() <= 64, Error::<T>::ProductDescriptionTooLong);
			// check for mandatory long description
			let longdescription =
				json_get_value(configuration.clone(), "longdescription".as_bytes().to_vec());
			ensure!(longdescription.len() >= 64, Error::<T>::ProductLongDescriptionTooShort);
			ensure!(longdescription.len() <= 8192, Error::<T>::ProductLongDescriptionTooLong);
			// check for price >0
			let price = json_get_value(configuration.clone(), "price".as_bytes().to_vec());
			let pricevalue = vecu8_to_u128(price);
			ensure!(pricevalue > 0, Error::<T>::ProductPriceCannotBeZero);
			// check for mandatory currency code
			let currencycode =
				json_get_value(configuration.clone(), "currency".as_bytes().to_vec());
			ensure!(Currencies::<T>::contains_key(&currencycode), Error::<T>::CurrencyCodeNotFound);
			// check for specifications
			let specifications =
				json_get_value(configuration.clone(), "specifications".as_bytes().to_vec());
			ensure!(
				(specifications.is_empty() || specifications.len() >= 8192),
				Error::<T>::SpecificationsIsdInvalid
			);
			// Media is an array of photos,videos and document being part of the product
			// documentation the structure can be
			// [{"description":"xxxxx","filename":"xxxxxxx"},"ipfs":"xxxxxxx",("color":xx)},{..}]
			let media = json_get_complexarray(configuration.clone(), "media".as_bytes().to_vec());
			ensure!(media.len() > 10, Error::<T>::MediaCannotBeEmpty);
			let mut x = 0;
			loop {
				let jr = json_get_recordvalue(media.clone(), x);
				if jr.is_empty() {
					break
				}
				let description = json_get_value(jr.clone(), "description".as_bytes().to_vec());
				ensure!(!description.is_empty(), Error::<T>::MediaDescriptionIswrong);

				let filename = json_get_value(jr.clone(), "filename".as_bytes().to_vec());
				ensure!(!filename.is_empty(), Error::<T>::MediaFileNameIsWrong);

				let ipfs = json_get_value(jr.clone(), "ipfs".as_bytes().to_vec());
				ensure!(ipfs.len() >= 32, Error::<T>::MediaIpfsAddressIsWrong);
				let color = json_get_value(jr.clone(), "color".as_bytes().to_vec());
				if !color.is_empty() {
					let colorvalue = vecu8_to_u32(color);
					ensure!(
						ProductColors::<T>::contains_key(colorvalue),
						Error::<T>::ColorNotFound
					);
				}
				x += 1;
			}
			// check colors if enabled
			let colors = json_get_complexarray(configuration.clone(), "colors".as_bytes().to_vec());
			if !colors.is_empty() {
				x = 0;
				loop {
					let c = json_get_arrayvalue(colors.clone(), x);
					if c.is_empty() {
						break
					}
					let cv = vecu8_to_u32(c);
					ensure!(ProductColors::<T>::contains_key(cv), Error::<T>::ColorNotFound);
					x += 1;
				}
			}
			// check size if enabled
			let sizes = json_get_complexarray(configuration.clone(), "sizes".as_bytes().to_vec());
			if !sizes.is_empty() {
				x = 0;
				loop {
					let sz = json_get_arrayvalue(sizes.clone(), x);
					if sz.is_empty() {
						break
					}
					let szv = vecu8_to_u32(sz);
					ensure!(ProductSizes::<T>::contains_key(szv), Error::<T>::ColorNotFound);
					x += 1;
				}
			}
			// check dimension (optional)
			let dimension =
				json_get_complexarray(configuration.clone(), "dimension".as_bytes().to_vec());
			if !dimension.is_empty() {
				x = 0;
				loop {
					let jr = json_get_recordvalue(dimension.clone(), x);
					if !jr.is_empty() {
						break
					}
					// check for length
					let l = json_get_value(jr.clone(), "length".as_bytes().to_vec());
					let v = vecu8_to_u32(l);
					ensure!(v > 0, Error::<T>::DimensionWrongLength);
					// check for wide
					let w = json_get_value(jr.clone(), "wide".as_bytes().to_vec());
					let v = vecu8_to_u32(w);
					ensure!(v > 0, Error::<T>::DimensionWrongWide);
					// check for height
					let h = json_get_value(jr.clone(), "height".as_bytes().to_vec());
					let v = vecu8_to_u32(h);
					ensure!(v > 0, Error::<T>::DimensionWrongHeight);
					// check for Weight
					let w = json_get_value(jr.clone(), "weight".as_bytes().to_vec());
					let v = vecu8_to_u32(w);
					ensure!(v > 0, Error::<T>::DimensionWrongWeight);
					x += 1;
				}
			}
			// check UPC code (TODO - UPC check CRC validity)
			let u = json_get_value(configuration.clone(), "upc".as_bytes().to_vec());
			ensure!(u.len() > 7, Error::<T>::UniversalProductCodeIsWrong);
			// check for shipping countries (optional)
			let shippingcountries = json_get_complexarray(
				configuration.clone(),
				"shippingcountries".as_bytes().to_vec(),
			);
			if !shippingcountries.is_empty() {
				x = 0;
				loop {
					let countrycode = json_get_arrayvalue(shippingcountries.clone(), x);
					if countrycode.is_empty() {
						break
					}
					ensure!(
						!IsoCountries::<T>::contains_key(&countrycode),
						Error::<T>::CountryCodeNotFound
					);
					x += 1;
				}
			}
			// check for shipping area
			// TODO (check for GPS coordinates validity)
			let shippingarea =
				json_get_complexarray(configuration.clone(), "shippingarea".as_bytes().to_vec());
			if !shippingarea.is_empty() {
				x = 0;
				loop {
					let centerlatitude =
						json_get_value(shippingarea.clone(), "centerlatitude".as_bytes().to_vec());
					let centerlongitude =
						json_get_value(shippingarea.clone(), "centerlongitude".as_bytes().to_vec());
					let borderlatitude =
						json_get_value(shippingarea.clone(), "borderlatitude".as_bytes().to_vec());
					let borderlongitude =
						json_get_value(shippingarea.clone(), "borderlongitude".as_bytes().to_vec());
					ensure!(!centerlatitude.len() > 0, Error::<T>::CenterLatitudeIsMissing);
					ensure!(!centerlongitude.len() > 0, Error::<T>::CenterLongitudeIsMissing);
					ensure!(!borderlatitude.len() > 0, Error::<T>::BorderLatitudeIsMissing);
					ensure!(!borderlongitude.len() > 0, Error::<T>::BorderLongitudeIsMissing);
					x += 1;
				}
			}
			// check for the shippers (optional field)
			let shippers = json_get_value(configuration.clone(), "shippers".as_bytes().to_vec());
			if !shippers.is_empty() {
				x = 0;
				loop {
					let shipper = json_get_arrayvalue(shippers.clone(), x);
					let v = vecu8_to_u32(shipper);
					ensure!(Shippers::<T>::contains_key(v), Error::<T>::ShipperNotFound);
					x += 1;
				}
			}
			// check for the API (optional field)
			let apiavailability =
				json_get_value(configuration.clone(), "apiavailability".as_bytes().to_vec());
			if !apiavailability.is_empty() {
				ensure!(aisland_validate_weburl(apiavailability), Error::<T>::InvalidApiUrl);
			}
			// check for the language if any
			let language = json_get_value(configuration.clone(), "language".as_bytes().to_vec());
			if !language.is_empty() {
				ensure!(aisland_validate_languagecode(language), Error::<T>::LanguageCodeIsWrong);
			}
			// TODO check the products was created from the same signer
			if Products::<T>::contains_key(uid) {
				Products::<T>::take(uid);
			}
			Products::<T>::insert(uid, configuration.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceProductUpdated(uid, configuration));
			// Return a successful DispatchResult
			Ok(())
		}
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		/// Create a new Iso dial code and name
		pub fn create_dialcode_country(
			origin: OriginFor<T>,
			countrycode: Vec<u8>,
			dialcode: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// check country code length == 2
			ensure!(countrycode.len() == 2, Error::<T>::WrongLengthCountryCode);
			// check country name length  >= 3
			ensure!(dialcode.len() >= 2, Error::<T>::DialcodeTooShort);
			// check the dialcode is not alreay present on chain
			ensure!(
				!IsoDialcode::<T>::contains_key(&countrycode),
				Error::<T>::CountryCodeAlreadyPresent
			);
			// store the Iso Dial Code
			IsoDialcode::<T>::insert(countrycode.clone(), dialcode.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceIsoDialCodeCreated(countrycode, dialcode));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy an Iso country code and name
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_dialcode_country(
			origin: OriginFor<T>,
			countrycode: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// verify the country code exists
			ensure!(IsoDialcode::<T>::contains_key(&countrycode), Error::<T>::CountryCodeNotFound);
			// Remove country code
			IsoDialcode::<T>::take(countrycode.clone());
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceIsoDialCodeDestroyed(countrycode));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Currency code with name and other info in a json structure
		/// {"name":"Bitcoin","category":"c(rypto)/f(iat)","country":"countryisocode","blockchain":"
		/// Ethereum(...)","address":"xxxfor_crypto_currencyxxx"} for example:
		/// {"name":"Bitcoin","category":"c","country":"AE","blockchain":"Bitcoin","address":"not
		/// applicable"} {"name":"American Dollars","category":"f","country":"US","blockchain":"not
		/// applicable","address":"not applicable"}
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_currency(
			origin: OriginFor<T>,
			currencycode: Vec<u8>,
			info: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// check currency code length is between 3 and 5 bytes
			ensure!(
				(currencycode.len() >= 3 && currencycode.len() <= 5),
				Error::<T>::WrongLengthCurrencyCode
			);
			// check the info field is not longer 1024 bytes
			ensure!((info.len() <= 1024), Error::<T>::SizeInfoTooLong);
			// check for a valid json structure
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// check for name
			let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
			ensure!(name.len() >= 3, Error::<T>::CurrencyNameTooShort);
			ensure!(name.len() <= 32, Error::<T>::CurrencyNameTooLong);
			// check for type of currency (fiat/crypto)
			let category = json_get_value(info.clone(), "category".as_bytes().to_vec());
			let c: Vec<u8> = vec![b'c'];
			let f: Vec<u8> = vec![b'f'];

			ensure!((category == c || category == f), Error::<T>::CurrencyCategoryIswrong);
			// check for the country code in case of Fiat currency
			if category == f {
				let countrycode = json_get_value(info.clone(), "country".as_bytes().to_vec());
				ensure!(
					IsoCountries::<T>::contains_key(countrycode),
					Error::<T>::CountryCodeNotFound
				);
			}
			// check for the blockchain in case of Crypto currency
			if category == c {
				let blockchain = json_get_value(info.clone(), "blockchain".as_bytes().to_vec());
				ensure!(blockchain.len() >= 3, Error::<T>::BlockchainNameTooShort);
				ensure!(blockchain.len() <= 32, Error::<T>::BlockchainNameTooLong);
			}
			// check the currency is not alreay present on chain
			ensure!(
				!Currencies::<T>::contains_key(&currencycode),
				Error::<T>::CurrencyCodeAlreadyPresent
			);
			// store the Currency Code and info
			Currencies::<T>::insert(currencycode.clone(), info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceCurrencyCodeCreated(currencycode, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a currency
		#[pallet::call_index(15)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_currency(origin: OriginFor<T>, currencycode: Vec<u8>) -> DispatchResult {
			// check the request is signed from the Super User
			ensure_root(origin)?;
			// verify the currency code exists
			ensure!(Currencies::<T>::contains_key(&currencycode), Error::<T>::CurrencyCodeNotFound);
			// Remove currency code
			Currencies::<T>::take(currencycode.clone());
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceCurrencyDestroyed(currencycode));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new product Color
		#[pallet::call_index(16)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_product_color(
			origin: OriginFor<T>,
			uid: u32,
			description: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::ColorUidCannotBeZero);
			//check description length
			ensure!(description.len() >= 2, Error::<T>::ColorDescriptionTooShort);
			ensure!(description.len() < 32, Error::<T>::ColorDescriptionTooLong);
			// check the color is not alreay present on chain
			ensure!(!ProductColors::<T>::contains_key(uid), Error::<T>::ColorAlreadyPresent);
			// store the color
			ProductColors::<T>::insert(uid, description.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceColorCreated(uid, description));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a product color
		#[pallet::call_index(17)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_product_color(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the color exists
			ensure!(ProductColors::<T>::contains_key(uid), Error::<T>::ColorNotFound);
			// Remove color
			ProductColors::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceColorDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new product Size
		/// example json in info field: {"code":"XL","description":"Extra Large","area":"Europe"}
		#[pallet::call_index(18)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_product_size(
			origin: OriginFor<T>,
			uid: u32,
			info: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::SizeUidCannotBeZero);
			//check info length
			ensure!(info.len() < 8192, Error::<T>::SizeInfoTooLong);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// checking sizes structure that must have some fields defined
			let mut x = 0;
			loop {
				let sz = json_get_recordvalue(info.clone(), x);
				if sz.is_empty() {
					break
				}

				let code = json_get_value(info.clone(), "code".as_bytes().to_vec());
				ensure!(!code.is_empty(), Error::<T>::SizeCodeIsMissing);

				let description = json_get_value(info.clone(), "description".as_bytes().to_vec());
				ensure!(!description.is_empty(), Error::<T>::SizeDescriptionIsMissing);

				let area = json_get_value(info.clone(), "area".as_bytes().to_vec());
				ensure!(!area.is_empty(), Error::<T>::SizeAreaIsMissing);

				x += 1;
			}
			// check the size is not alreay present on chain
			ensure!(!ProductSizes::<T>::contains_key(uid), Error::<T>::SizeAlreadyPresent);
			// store the Size
			ProductSizes::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceSizeCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a product size
		#[pallet::call_index(19)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_product_size(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the size exists
			ensure!(ProductSizes::<T>::contains_key(uid), Error::<T>::SizeNotFound);
			// Remove size
			ProductSizes::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceSizeDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Manufacturer
		/// Example field info: {"name":"Samsung","website":"https://www.samsung.com"}
		#[pallet::call_index(20)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_manufacturer(
			origin: OriginFor<T>,
			uid: u32,
			info: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::ManufacturerUidCannotBeZero);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// check for name field
			let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
			ensure!(name.len() >= 4, Error::<T>::ManufacturerNameIsTooShort);
			ensure!(name.len() <= 64, Error::<T>::ManufacturerNameIsTooLong);
			// check for website field
			let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
			ensure!(website.len() >= 4, Error::<T>::ManufacturerWebsiteIsTooShort);
			ensure!(website.len() <= 64, Error::<T>::ManufacturerWebsiteIsTooLong);
			// check the manufacturer is not alreay present on chain
			ensure!(!Manufacturers::<T>::contains_key(uid), Error::<T>::ManufacturerAlreadyPresent);
			// store the manufacturer
			Manufacturers::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceManufacturerCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a manufacturer
		#[pallet::call_index(21)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_manufacturer(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the manufacturer exists
			ensure!(Manufacturers::<T>::contains_key(uid), Error::<T>::ManufacturerNotFound);
			// Remove manufacturer
			Manufacturers::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceManufacturerDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Shipper
		/// exmaple info field: {"name":"DHL","website":"www.dhl.com"}
		#[pallet::call_index(22)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_shipper(origin: OriginFor<T>, uid: u32, info: Vec<u8>) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::ShipperUidCannotBeZero);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			ensure!(info.len() <= 16384, Error::<T>::InvalidJson);
			// check for name field
			let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
			ensure!(name.len() >= 3, Error::<T>::ShipperNameIsTooShort);
			ensure!(name.len() <= 64, Error::<T>::ShipperNameIsTooLong);
			// check for info field
			ensure!(info.len() <= 32768, Error::<T>::JsonIsTooLong);
			// check for website field
			let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
			ensure!(website.len() >= 4, Error::<T>::ShipperWebsiteIsTooShort);
			ensure!(website.len() <= 64, Error::<T>::ShipperWebsiteIsTooLong);
			// check the Shipper is not alreay present on chain
			ensure!(!Shippers::<T>::contains_key(uid), Error::<T>::ShipperAlreadyPresent);
			// check for origincountries field (optional)
			let origincountries =
				json_get_complexarray(info.clone(), "origincountries".as_bytes().to_vec());
			if !origincountries.is_empty() {
				let mut x = 0;
				loop {
					let oc = json_get_arrayvalue(origincountries.clone(), x);
					if oc.is_empty() {
						break
					}
					ensure!(
						IsoCountries::<T>::contains_key(oc),
						Error::<T>::OriginCountryNotPresent
					);
					x += 1;
				}
			}
			// check for destinationcountries field (optional)
			let destinationcountries =
				json_get_complexarray(info.clone(), "destinationcountries".as_bytes().to_vec());
			if !destinationcountries.is_empty() {
				let mut x = 0;
				loop {
					let dc = json_get_arrayvalue(destinationcountries.clone(), x);
					if dc.is_empty() {
						break
					}
					ensure!(
						IsoCountries::<T>::contains_key(dc),
						Error::<T>::DestinationCountryNotPresent
					);
					x += 1;
				}
			}
			// store the shippers
			Shippers::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceShipperCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Shipper
		#[pallet::call_index(23)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_shipper(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the shipper exists
			ensure!(Shippers::<T>::contains_key(uid), Error::<T>::ShipperNotFound);
			// Remove shipper
			Shippers::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceShipperDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create new Shipping Rates
		/// example field info:
		/// {"shipperid":1,"origincountry":"AE","currency":"AED","rates":[{"destination":"LR","
		/// fromkg":0,"to":1,"rate":10},{"destination":"LR","fromkg":1,"tokg":2,"rate":15}]}
		#[pallet::call_index(24)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_shipping_rates(
			origin: OriginFor<T>,
			uid: u32,
			info: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::ShippingRateUidCannotBeZero);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// check for info field
			ensure!(info.len() <= 32768, Error::<T>::JsonIsTooLong);
			// check for shipperid field
			let shipperid = json_get_value(info.clone(), "shipperid".as_bytes().to_vec());
			let shipperidv = vecu8_to_u32(shipperid);
			ensure!(shipperidv > 0, Error::<T>::ShipperIdIsMissing);
			ensure!(Shippers::<T>::contains_key(shipperidv), Error::<T>::ShipperNotFound);
			// check for origincountry field
			let origincountry = json_get_value(info.clone(), "origincountry".as_bytes().to_vec());
			ensure!(
				IsoCountries::<T>::contains_key(origincountry.clone()),
				Error::<T>::OriginCountryNotPresent
			);
			// check for currency field
			let currency = json_get_value(info.clone(), "currency".as_bytes().to_vec());
			ensure!(Currencies::<T>::contains_key(currency), Error::<T>::CurrencyCodeNotFound);
			// check for rates
			let rates = json_get_complexarray(info.clone(), "rates".as_bytes().to_vec());
			if !rates.is_empty() {
				let mut x = 0;
				loop {
					let r = json_get_recordvalue(rates.clone(), x);
					if r.is_empty() {
						break
					}
					// check for destination
					let dc = json_get_value(r.clone(), "destination".as_bytes().to_vec());
					ensure!(
						IsoCountries::<T>::contains_key(dc),
						Error::<T>::OriginCountryNotPresent
					);
					// check for fromkg
					let fromkg = json_get_value(r.clone(), "fromkg".as_bytes().to_vec());
					ensure!(!fromkg.is_empty(), Error::<T>::FromKgIsMissing);
					// check for tokg
					let tokg = json_get_value(r.clone(), "tokg".as_bytes().to_vec());
					ensure!(!tokg.is_empty(), Error::<T>::ToKgIsMissing);
					// check for rates  (in the currency set)
					let rate = json_get_value(r.clone(), "rate".as_bytes().to_vec());
					let ratev = vecu8_to_u32(rate);
					ensure!(ratev > 0, Error::<T>::ShippingRateCannotbeZero);
					x += 1;
				}
			}
			// store the shipping rates
			ShippingRates::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketShippingRateCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy Shipping Rates
		#[pallet::call_index(25)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_shipping_rates(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the shipping rate exists
			ensure!(ShippingRates::<T>::contains_key(uid), Error::<T>::ShippingRatesNotFound);
			// Remove shipper
			Shippers::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceShipperDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Brand
		/// Example of info field: {"name":"Galaxy","manufacturer":7}
		#[pallet::call_index(26)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_brand(origin: OriginFor<T>, uid: u32, info: Vec<u8>) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::BrandUidCannotBeZero);
			ensure!(info.len() <= 1024, Error::<T>::JsonIsTooLong);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// check for name field
			let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
			ensure!(name.len() >= 4, Error::<T>::BrandNameIsTooShort);
			ensure!(name.len() <= 64, Error::<T>::BrandNameIsTooLong);
			// check for website field
			let manufacturer = json_get_value(info.clone(), "manufacturer".as_bytes().to_vec());
			let mv = vecu8_to_u32(manufacturer);
			ensure!(mv > 0, Error::<T>::ManufacturerNotFound);
			// check the Manufacturer is  present on chain
			ensure!(Manufacturers::<T>::contains_key(mv), Error::<T>::ManufacturerNotFound);
			// check the brand is not present on chain
			ensure!(!Brands::<T>::contains_key(uid), Error::<T>::BrandAlreadyPresent);
			// store the brand
			Brands::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceBrandCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a Brand
		#[pallet::call_index(27)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_brand(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the brand exists
			ensure!(Brands::<T>::contains_key(uid), Error::<T>::BrandNotFound);
			// Remove brand
			Brands::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceBrandDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new product model
		/// Example field info: {"name":"A1","brand":1}
		#[pallet::call_index(28)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_product_model(
			origin: OriginFor<T>,
			uid: u32,
			info: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from root
			ensure_root(origin)?;
			// check uid >0
			ensure!(uid > 0, Error::<T>::ModelUidCannotBeZero);
			// check valid json
			ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
			// check for name field
			let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
			ensure!(name.len() >= 2, Error::<T>::ModelNameIsTooShort);
			ensure!(name.len() <= 32, Error::<T>::ModelNameIsTooLong);
			// check for brand field
			let brand = json_get_value(info.clone(), "brand".as_bytes().to_vec());
			let bv = vecu8_to_u32(brand);
			ensure!(bv > 0, Error::<T>::BrandNotFound);
			// check the brand is  present on chain
			ensure!(Brands::<T>::contains_key(bv), Error::<T>::BrandNotFound);
			// check the model is not present on chain
			ensure!(!ProductModels::<T>::contains_key(uid), Error::<T>::ModelAlreadyPresent);
			// store the model
			ProductModels::<T>::insert(uid, info.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceProductModelCreated(uid, info));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Destroy a product model
		#[pallet::call_index(29)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_product_model(origin: OriginFor<T>, uid: u32) -> DispatchResult {
			// check the request is signed from Super User
			ensure_root(origin)?;
			// verify the model exists
			ensure!(ProductModels::<T>::contains_key(uid), Error::<T>::BrandNotFound);
			// Remove model
			ProductModels::<T>::take(uid);
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceProductModelDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Login Data
		#[pallet::call_index(30)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn create_login_data(
			origin: OriginFor<T>,
			emailhash: Vec<u8>,
			encryptedpwdhash: Vec<u8>,
			accountid: T::AccountId,
			encryptedseed: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed
			let _sender = ensure_signed(origin)?;
			// check Email hash length
			ensure!(emailhash.len() > 8, Error::<T>::WrongLengthEmailHash);
			// check Encrypted Password length
			ensure!(encryptedpwdhash.len() > 8, Error::<T>::WrongLengthEncryptedPassword);
			// check the email ahsh is not alreay present on chain
			ensure!(!LoginData::<T>::contains_key(&emailhash), Error::<T>::EmailHashAlreadyPresent);
			// store the Login Data
			LoginData::<T>::insert(emailhash.clone(), encryptedpwdhash.clone());
			// store the Account id
			EmailAccount::<T>::insert(emailhash.clone(), accountid.clone());
			// store the encrypted seed (double encryption layer)
			EmailEncryptedSeed::<T>::insert(emailhash.clone(), encryptedseed.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceLoginDataCreated(
				emailhash,
				encryptedpwdhash,
				accountid,
			));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Create a new Login Data
		#[pallet::call_index(31)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn change_pwd_login_data(
			origin: OriginFor<T>,
			emailhash: Vec<u8>,
			encryptedpwdhash: Vec<u8>,
		) -> DispatchResult {
			// check the request is signed from the same user who created it
			let sender = ensure_signed(origin)?;
			// check Email hash length
			ensure!(emailhash.len() > 8, Error::<T>::WrongLengthEmailHash);
			// check Encrypted Password length
			ensure!(encryptedpwdhash.len() > 8, Error::<T>::WrongLengthEncryptedPassword);
			// check the email hash is alreay present on chain
			ensure!(LoginData::<T>::contains_key(&emailhash), Error::<T>::EmailHashNotFound);
			// check the email account is present on chain
			ensure!(EmailAccount::<T>::contains_key(&emailhash), Error::<T>::EmailHashNotFound);
			//let accountidemail=EmailAccount::<T>::get(&emailhash).unw;
			let accountidemail = EmailAccount::<T>::get(emailhash.clone()).unwrap();
			// check that the signer is the creator of the original state
			ensure!(sender == accountidemail, Error::<T>::SignerIsNotAuthorized);
			// store the Login Data
			LoginData::<T>::take(&emailhash);
			LoginData::<T>::insert(emailhash.clone(), encryptedpwdhash.clone());
			// Generate event
			Self::deposit_event(Event::MarketPlaceLoginPwdChanged(emailhash, encryptedpwdhash));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Destroy a login data
		#[pallet::call_index(32)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_login_data(origin: OriginFor<T>, emailhash: Vec<u8>) -> DispatchResult {
			// check the request is signed from the same signer of the original writing
			let sender = ensure_signed(origin)?;
			// verify the login data exists
			ensure!(LoginData::<T>::contains_key(&emailhash), Error::<T>::EmailHashNotFound);
			ensure!(EmailAccount::<T>::contains_key(&emailhash), Error::<T>::EmailHashNotFound);
			let accountid = EmailAccount::<T>::get(emailhash.clone());
			ensure!(
				accountid == core::prelude::v1::Some(sender),
				Error::<T>::SignerIsNotAuthorized
			);
			// Remove email hash and accountid and encrypted seed
			LoginData::<T>::take(emailhash.clone());
			EmailAccount::<T>::take(emailhash.clone());
			EmailEncryptedSeed::<T>::take(emailhash.clone());
			// Generate event
			//it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(Event::MarketPlaceLoginDataDestroyed(emailhash));
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
	let first = *j.first().unwrap();
	if first != b'{' && first != b'[' {
		return false
	}
	//checks that end is } or ]
	let last = *j.last().unwrap();
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
// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec
// when the records is not present
fn json_get_recordvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
	let mut result = Vec::new();
	let mut op = true;
	let mut cn = 0;
	let mut lb = b' ';
	for v in ar {
		if v == b',' && op {
			cn += 1;
			continue
		}
		if v == b'[' && op && lb != b'\\' {
			continue
		}
		if v == b']' && op && lb != b'\\' {
			continue
		}
		if v == b'{' && op && lb != b'\\' {
			op = false;
		}
		if v == b'}' && !op && lb != b'\\' {
			op = true;
		}
		// field found
		if cn == p {
			result.push(v);
		}
		lb = v;
	}
	result
}
// function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the
// records is not present
fn json_get_arrayvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
	let mut result = Vec::new();
	let mut op = true;
	let mut cn = 0;
	let mut lb = b' ';
	for v in ar {
		if v == b',' && op {
			cn += 1;
			continue
		}
		if v == b'[' && op && lb != b'\\' {
			continue
		}
		if v == b']' && op && lb != b'\\' {
			continue
		}
		if v == b'"' && op && lb != b'\\' {
			continue
		}
		if v == b'"' && op && lb != b'\\' {
			op = false;
		}
		if v == b'"' && !op && lb != b'\\' {
			op = true;
		}
		// field found
		if cn == p {
			result.push(v);
		}
		lb = v;
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
// function to validate and email address, return true/false
fn aisland_validate_email(email: Vec<u8>) -> bool {
	let mut flagat = false;
	let mut valid = false;
	let mut phase = 1;
	let mut dotphase2 = false;
	for c in email {
		let ch: char = c.into();
		if ch == '@' {
			flagat = true;
			phase = 2;
			continue
		}
		// check for allowed char in the first part of the email address before @
		if phase == 1 {
			if ch.is_ascii_alphabetic() || ch == '-' || ch == '.' || ch == '_' {
				valid = true;
			} else {
				valid = false;
				break
			}
		}
		// check for allowed char in the second part of the email address before @
		if phase == 2 {
			if ch.is_ascii_alphabetic() || ch == '-' || ch == '.' {
				valid = true;
			} else {
				valid = false;
				break
			}
			if c == 46 {
				dotphase2 = true;
			}
		}
	}
	// return validity true/false
	if flagat && dotphase2 {
		valid
	} else {
		flagat
	}
}
// function to validate an web url return true/false
fn aisland_validate_weburl(weburl: Vec<u8>) -> bool {
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
	for c in weburl {
		let ch: char = c.into();
		if x < startpoint {
			x += 1;
			continue
		}
		// check for allowed chars
		if (' '..='_').contains(&ch) || ('a'..='~').contains(&ch) {
			valid = true;
		} else {
			valid = false;
			break
		}
	}
	valid
}
// function to validate a phone number
fn aisland_validate_phonenumber(phonenumber: Vec<u8>) -> bool {
	// check maximum lenght
	if phonenumber.len() > 23 {
		return false
	}
	// check admitted bytes
	let mut x = 0;
	for vv in phonenumber.clone() {
		let v: char = vv.into();
		if v.is_ascii_digit() || (v == '+' && x == 0) {
			x += 1;
		} else {
			return false
		}
	}
	// load international prefixes table
	let p = vec![
		"972", "93", "355", "213", "376", "244", "54", "374", "297", "61", "43", "994", "973",
		"880", "375", "32", "501", "229", "975", "387", "267", "55", "246", "359", "226", "257",
		"855", "237", "1", "238", "345", "236", "235", "56", "86", "61", "57", "269", "242", "682",
		"506", "385", "53", "537", "420", "45", "253", "593", "20", "503", "240", "291", "372",
		"251", "298", "679", "358", "33", "594", "689", "241", "220", "995", "49", "233", "350",
		"30", "299", "590", "502", "224", "245", "595", "509", "504", "36", "354", "91", "62",
		"964", "353", "972", "39", "81", "962", "254", "686", "965", "996", "371", "961", "266",
		"231", "423", "370", "352", "261", "265", "60", "960", "223", "356", "692", "596", "222",
		"230", "262", "52", "377", "976", "382", "1664", "212", "95", "264", "674", "977", "31",
		"599", "687", "64", "505", "227", "234", "683", "672", "47", "968", "92", "680", "507",
		"675", "595", "51", "63", "48", "351", "974", "40", "250", "685", "378", "966", "221",
		"381", "248", "232", "65", "421", "386", "677", "27", "500", "34", "94", "249", "597",
		"268", "46", "41", "992", "66", "228", "690", "676", "216", "90", "993", "688", "256",
		"380", "971", "44", "1", "598", "998", "678", "681", "967", "260", "263", "591", "673",
		"61", "243", "225", "500", "44", "379", "852", "98", "44", "44", "850", "82", "856", "218",
		"853", "389", "691", "373", "258", "970", "872", "262", "7", "590", "290", "590", "508",
		"239", "252", "47", "963", "886", "255", "670", "58", "84",
	];
	// normalis number
	let mut startpoint = 0;
	if phonenumber[0] == b'0' && phonenumber[1] == b'0' {
		startpoint = 2;
	}
	if phonenumber[0] == b'+' {
		startpoint = 1;
	}
	// create vec for comparison
	let pc3: Vec<u8> =
		vec![phonenumber[startpoint], phonenumber[startpoint + 1], phonenumber[startpoint + 2]];
	let pc2: Vec<u8> = vec![phonenumber[startpoint], phonenumber[startpoint + 1]];
	let pc1: Vec<u8> = vec![phonenumber[startpoint]];

	let valid = p.iter().any(|xp| {
		let bytes = Into::<Vec<u8>>::into(*xp);
		bytes == pc3 || bytes == pc2 || bytes == pc1
	});
	valid
}
// function to validate a language code
fn aisland_validate_languagecode(language: Vec<u8>) -> bool {
	// check maximum lenght
	if language.len() > 2 {
		return false
	}
	// load allowed language code
	let p = vec![
		"aa", "ab", "ae", "af", "ak", "am", "an", "ar", "as", "av", "ay", "az", "ba", "be", "bg",
		"bh", "bi", "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs", "cu", "cv",
		"cy", "da", "de", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff", "fi",
		"fj", "fo", "fr", "fy", "ga", "gd", "gl", "gn", "gu", "gv", "ha", "he", "hi", "ho", "hr",
		"ht", "hu", "hy", "hz", "ia", "id", "ie", "ig", "ii", "ik", "io", "is", "it", "iu", "ja",
		"jv", "ka", "kg", "ki", "kj", "kk", "kl", "km", "kn", "ko", "kr", "ks", "ku", "kv", "kw",
		"ky", "la", "lb", "lg", "li", "ln", "lo", "lt", "lu", "lv", "mg", "mh", "mi", "mk", "ml",
		"mn", "mr", "ms", "mt", "my", "na", "nb", "nd", "ne", "ng", "nl", "nn", "no", "nr", "nv",
		"ny", "oc", "oj", "om", "or", "os", "pa", "pi", "pl", "ps", "pt", "qu", "rm", "rn", "ro",
		"ru", "rw", "sa", "sc", "sd", "se", "sg", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr",
		"ss", "st", "su", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "tl", "tn", "to", "tr",
		"ts", "tt", "tw", "ty", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa", "wo", "xh", "yi",
		"yo", "za", "zh", "zu",
	];
	let mut valid = false;
	for xp in p {
		if language == Into::<Vec<u8>>::into(xp) {
			valid = true;
		}
	}
	valid
}
// function to validate the unit measurement system
fn aisland_validate_unitmeasurement(unitmeasurement: Vec<u8>) -> bool {
	// check maximum lenght
	if unitmeasurement.len() > 2 {
		return false
	}
	// load allowed language code
	let p: Vec<Vec<u8>> = vec!["ms".into(), "iu".into(), "us".into()];
	let mut valid = false;
	for xp in p {
		if xp == unitmeasurement {
			valid = true;
		}
	}
	valid
}
// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
	let vslice = v.as_slice();
	let vstr = str::from_utf8(vslice).unwrap_or("0");
	let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
	vvalue
}
// function to convert vec<u8> to u128
fn vecu8_to_u128(v: Vec<u8>) -> u128 {
	let vslice = v.as_slice();
	let vstr = str::from_utf8(vslice).unwrap_or("0");
	let vvalue: u128 = u128::from_str(vstr).unwrap_or(0);
	vvalue
}
