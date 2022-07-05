#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult},
        traits::{Currency},
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
	use sp_std::{prelude::*, str};


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the WCN pallet.
        type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type MaxLoyaltyOrgOwned: Get<u32>;

    }

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	// Handles our pallet's currency abstraction
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding Loyalty information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct LoyaltyPoint<T: Config> {
		pub org_code: [u8; 3],
		pub amount: Option<BalanceOf<T>>
	}



    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Trying to transfer or buy a loyalty point from oneself.but 1 generic argument
		TransferToSelf,
		/// You are not the owner of this loyalty point.
		NotOwner,
		/// An account may only own `MaxLoyaltyOrgOwned` loyalty point.
		TooManyOwned,
		/// other
		Others,
		// Error returned when fetching http info
		HttpFetchingError,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new loyalty point was successfully created.
		Created { org_code: [u8; 3], owner: AccountOf<T> },
		/// A trasfer was successfully transferred.
		Transferred { org_code: [u8; 3], owner: AccountOf<T> },
    }

	/// Track the loyalty point owned by each account.
	#[pallet::storage]
	#[pallet::getter(fn loyalty_ownerd)]
	pub(super) type LoyaltyOwned<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<LoyaltyPoint<T>, T::MaxLoyaltyOrgOwned>,
		ValueQuery,
	>;



    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100_000)]
		pub fn deposit_loyalty(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			org_code: Vec<u8>
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			// Create new loyalty to storage by calling helpuse sp_runtime::traits::Saturating;er function
			Self::mint(&sender, Self::convert_vu8_array(&org_code), amount)?;

			Ok(())
		}

		#[pallet::weight(100_000)]
		pub fn withdraw_loyalty(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			org_code: Vec<u8>
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			// Decrease to account
			Self::decrease_loyalty(&sender, Self::convert_vu8_array(&org_code), amount)?;
			Ok(())
		}

		#[pallet::weight(100_000)]
		pub fn transfer_loyalty(
			from_account: OriginFor<T>,
			to: T::AccountId,
			org_code: Vec<u8>,
			amount: BalanceOf<T>
		) -> DispatchResult{
			let from = ensure_signed(from_account)?;
			// Verify the kitty is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);
			// FIXME check fromAccount has org_code
			// Call transfer funciton
			Self::transfer(&from, &to, Self::convert_vu8_array(&org_code),amount)?;
			Ok(())
		}


		#[pallet::weight(100_000)]
		pub fn swap_loyalty(
			from_account: OriginFor<T>,
			to: T::AccountId,
			from_org_code: Vec<u8>,
			to_org_code: Vec<u8>,
			from_amount: BalanceOf<T>,
			to_amount: BalanceOf<T>,
		) -> DispatchResult{
			let from = ensure_signed(from_account)?;
			// FIXME Make sure current balance is enough
			// Decrease to account
			Self::decrease_loyalty(&from, Self::convert_vu8_array(&from_org_code), from_amount)?;
			// Increase to account
			Self::increase_loyalty(&to, Self::convert_vu8_array(&from_org_code), from_amount)?;

			// Decrease to account
			Self::decrease_loyalty(&to, Self::convert_vu8_array(&to_org_code), to_amount)?;
			// Increase to account
			Self::increase_loyalty(&from, Self::convert_vu8_array(&to_org_code), to_amount)?;

			Ok(())
		}

    }


    impl<T: Config> Pallet<T> {
       // Generates and returns random org
		pub fn gen_random_org() -> [u8; 3] {
			let str = "WCN";
 			let mut array_tmp = [0u8; 3];
			array_tmp[..str.len()].copy_from_slice(str.as_bytes());
			array_tmp
		}

		pub fn convert_vu8_array(
			input_data: &Vec<u8>,
		) -> [u8; 3]{
			let mut array_tmp = [0u8; 3];
			array_tmp[..input_data.len()].copy_from_slice(input_data.as_slice());
			array_tmp
		}

		pub fn transfer(
			from: &T::AccountId,
			to: &T::AccountId,
			org_code: [u8; 3],
			amount: BalanceOf<T>,
		) -> Result<[u8; 3], DispatchError>{
			// Decrease to account
			Self::decrease_loyalty(&from, org_code, amount)?;
			// Increase to account
			Self::increase_loyalty(&to, org_code, amount)?;
			Ok(org_code)
		}

		pub fn increase_loyalty(
			account_id: &T::AccountId,
			org_code: [u8; 3],
			amount: BalanceOf<T>,
		) -> Result<[u8; 3], DispatchError>{
			let mut loyalty_point = LoyaltyPoint::<T> { org_code: org_code, amount: Some(amount) };
			// Check if exist
			let mut loyalty_point_vec_tmp = LoyaltyOwned::<T>::get(&account_id);
			if let Some(index) = loyalty_point_vec_tmp.iter().position(|id| id.org_code == org_code) {
				 let loyalty_point_tmp = loyalty_point_vec_tmp.get(index).unwrap();
				 if loyalty_point_tmp.org_code == org_code {
				 	let tmp_camount = loyalty_point_tmp.amount.unwrap();
					 use sp_runtime::traits::Saturating;
					 loyalty_point.amount = Some(amount.saturating_add(tmp_camount));
					 loyalty_point_vec_tmp.remove(index);
					 loyalty_point_vec_tmp.try_push(loyalty_point).map_err(|_| Error::<T>::Others)?;
					 LoyaltyOwned::<T>::insert(&account_id, loyalty_point_vec_tmp);
				 }
			}else{
			// Append loyalty to LoyaltyOwned
			LoyaltyOwned::<T>::try_append(&account_id, loyalty_point)
				.map_err(|_| Error::<T>::TooManyOwned)?; 
			}
			Ok(org_code)
		}

		pub fn decrease_loyalty(
			account_id: &T::AccountId,
			org_code: [u8; 3],
			amount: BalanceOf<T>,
		) -> Result<[u8; 3], DispatchError>{
			let mut loyalty_point = LoyaltyPoint::<T> { org_code: org_code, amount: Some(amount) };
			// Check if exist
			let mut loyalty_point_vec_tmp = LoyaltyOwned::<T>::get(&account_id);
			if let Some(index) = loyalty_point_vec_tmp.iter().position(|id| id.org_code == org_code) {
				 let loyalty_point_tmp = loyalty_point_vec_tmp.get(index).unwrap();
				 if loyalty_point_tmp.org_code == org_code {
				 	let tmp_camount = loyalty_point_tmp.amount.unwrap();
					 use sp_runtime::traits::Saturating;
					 loyalty_point.amount = Some(tmp_camount.saturating_sub(amount));
					 loyalty_point_vec_tmp.remove(index);
					 loyalty_point_vec_tmp.try_push(loyalty_point).map_err(|_| Error::<T>::Others)?;
					 LoyaltyOwned::<T>::insert(&account_id, loyalty_point_vec_tmp);
				 }
			}else{
			// Append loyalty to LoyaltyOwned
			LoyaltyOwned::<T>::try_append(&account_id, loyalty_point)
				.map_err(|_| Error::<T>::TooManyOwned)?;
			}
			Ok(org_code)
		}

		// Helper to mint a loyalty point
		pub fn mint(
			owner: &T::AccountId,
			org_code: [u8; 3],
			amount: BalanceOf<T>,
		) -> Result<[u8; 3], DispatchError> {
			Self::increase_loyalty(&owner, org_code, amount)?;

			// Deposit our "Created"
			Self::deposit_event(Event::Created { org_code: org_code, owner: owner.clone() });

			// Returns the org_code of the new loyalty if this succeeds
			Ok(org_code)
		}
    }
}
