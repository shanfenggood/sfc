#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode,Decode,Codec};
use frame_support::{Parameter,decl_storage,decl_module,decl_error,decl_event,ensure,StorageValue,StorageMap,traits::Randomness,
                    traits::{Get, Currency, ReservableCurrency}
                    };
use sp_io::hashing::blake2_128;

use sp_std::vec::Vec;
use frame_system::ensure_signed;
use sp_runtime::{
    DispatchError,
    traits::{
        Bounded,
        Member,AtLeast32BitUnsigned
    }
};

#[derive(Encode,Decode)]
pub struct Kitty(pub [u8;16]);

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//第六题
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;


pub trait Trait: frame_system::Trait{

    type Event: From<Event<Self>>+Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;

    //第二题
    type KittyIndex: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + Bounded ;


    type KittyReserve: Get<BalanceOf<Self>>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


decl_storage!{
    trait Store for Module<T:Trait> as Kitties{
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count):T::KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex=>Option<T::AccountId>;

        //第三题：存储帐号与kitty 关系
        pub OwnerKittyList get(fn kitty_list): double_map hasher(blake2_128_concat) T::AccountId ,hasher(blake2_128_concat) T::KittyIndex =>T::KittyIndex;

        //第四题
        pub ChildrenList get(fn children_list): map hasher(blake2_128_concat) T::KittyIndex =>Vec<T::KittyIndex>;
        pub ParentMap get(fn parent_map): map hasher(blake2_128_concat) T::KittyIndex =>(T::KittyIndex,T::KittyIndex);

        pub BreedList get(fn breed_list): double_map hasher(blake2_128_concat) T::KittyIndex ,hasher(blake2_128_concat) T::KittyIndex =>T::KittyIndex;


    }
}
decl_error!{
    pub enum Error for Module<T:Trait>{
        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
        NotKittyOwner,
        MoneyNotEnough
    }
}

decl_event!{
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, KittyIndex = <T as Trait>::KittyIndex {

        Created(AccountId,KittyIndex),
        Transferred(AccountId,AccountId,KittyIndex),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		type Error = Error<T>;
		fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin){

            let sender = ensure_signed(origin)?;
            T::Currency::reserve(&sender, T::KittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough )?;

            let kitty_id = Self::next_kitty_id()?;

            let dna = Self::random_value(&sender);

            let kitty = Kitty(dna);
            Self::insert_kitty(&sender,kitty_id,kitty);

            OwnerKittyList::<T>::insert(&sender,kitty_id,kitty_id);

            Self::deposit_event(RawEvent::Created(sender,kitty_id));

        }

        #[weight = 0]
        pub fn transfer(origin,to:T::AccountId,kitty_id:T::KittyIndex){
             let sender = ensure_signed(origin)?;
             //判断是不是owner
             //第一题
             let owner = KittyOwners::<T>::get(kitty_id);
            match owner{
                Some(v) => ensure!(sender==v,Error::<T>::NotKittyOwner),
                None =>return Err(Error::<T>::InvalidKittyId.into())
            }

            OwnerKittyList::<T>::remove(&sender,kitty_id);
            OwnerKittyList::<T>::insert(&to,kitty_id,kitty_id);

             <KittyOwners<T>>::insert(kitty_id,to.clone());
             Self::deposit_event(RawEvent::Transferred(sender,to,kitty_id))

        }

        #[weight = 0]
        pub fn breed(origin,kitty_id_1:T::KittyIndex,kitty_id_2:T::KittyIndex){
            let sender = ensure_signed(origin)?;
            T::Currency::reserve(&sender, T::KittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough )?;

            let new_kitty_id = Self::do_breed(&sender,kitty_id_1,kitty_id_2)?;

            //加入孩子列表
            let mut baby_vec;
            if ChildrenList::<T>::contains_key(&kitty_id_1){
                baby_vec = ChildrenList::<T>::get(&kitty_id_1);
            }else{
                baby_vec = Vec::<T::KittyIndex>::new();
            }
            baby_vec.push(new_kitty_id);
            ChildrenList::<T>::insert(&kitty_id_1,baby_vec);

            if ChildrenList::<T>::contains_key(&kitty_id_2){
                baby_vec = ChildrenList::<T>::get(&kitty_id_2);
            }else{
                baby_vec = Vec::<T::KittyIndex>::new();
            }
            baby_vec.push(new_kitty_id);
            ChildrenList::<T>::insert(&kitty_id_2,baby_vec);

            //建议Parent关系
            ParentMap::<T>::insert(new_kitty_id,(kitty_id_1,kitty_id_2));

            //建立breed关系
            BreedList::<T>::insert(kitty_id_1,kitty_id_2,kitty_id_2);
            BreedList::<T>::insert(kitty_id_2,kitty_id_1,kitty_id_1);

            OwnerKittyList::<T>::insert(&sender,new_kitty_id,new_kitty_id);
            Self::deposit_event(RawEvent::Created(sender,new_kitty_id));

        }
    }
}

fn combine_dna(dna1:u8,dna2:u8,selector:u8)->u8{
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> where {
    fn next_kitty_id()-> sp_std::result::Result<T::KittyIndex,DispatchError> {

        let kitty_id = Self::kitties_count();
        if kitty_id == T::KittyIndex::max_value(){
            return Err(Error::<T>::KittiesCountOverflow.into());
        }
        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) ->[u8;16]{

        (T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index()
        ).using_encoded(blake2_128)

    }

    fn insert_kitty(owner: &T::AccountId, kitty_id:T::KittyIndex,kitty:Kitty){
        Kitties::<T>::insert(kitty_id,kitty);
        KittiesCount::<T>::put(kitty_id+1.into());
        <KittyOwners<T>>::insert(kitty_id,owner);
    }

    fn do_breed(sender: &T::AccountId,  kitty_id_1:T::KittyIndex,kitty_id_2:T::KittyIndex  )->sp_std::result::Result<T::KittyIndex,DispatchError>{
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2,Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;

        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8;16];

        for i in 0..kitty1_dna.len(){
            new_dna[i] = combine_dna(kitty1_dna[i],kitty2_dna[i],selector[i]);
        }

        Self::insert_kitty(sender,kitty_id,Kitty(new_dna));
        Ok(kitty_id)
    }

}


