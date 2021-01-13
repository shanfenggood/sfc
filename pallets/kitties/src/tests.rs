use crate::{Error, mock::*};
use crate::{mock::*};
// use frame_support::{assert_ok, assert_noop};
use super::*;

use frame_system::{EventRecord, Phase};
use frame_system::ensure_signed;


#[test]
fn owned_kitties_can_append_value(){
    new_test_ext().execute_with(||{
        run_to_block(10);
        let origin = Origin::signed(1);
        //第五题
       assert_eq!(KittiesModule::create(origin, ),Ok(()));
        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: TestEvent::simple_event(Event::<Test>::Created(1u64, 0)),
                topics: vec![],
            }]
        );
    });
}






// #[test]
// fn testdoubleMap(){
//     new_test_ext().execute_with(||{
//         run_to_block(10);
        // assert_eq!(KittiesModule::create(Origin::signed(1), ),Ok(()));
        //
        // OwnerKittyList::<Test>::get(Origin::signed(1),1);
    //
    // });
// }

//
// fn create_claim_works(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//         assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
//
//         assert_eq!(Poes::<Test>::get(&claim),(1,frame_system::Module::<Test>::block_number()));
//     })
// }
// #[test]
// fn create_claim_failed_when_toolong(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1,2,3,4,5];
//         assert_noop!(
//             PoeModule::create_claim(Origin::signed(1),claim.clone()),
//              Error::<Test>::ClaimTooLong
//         );
//     }
//     );
// }
//
// #[test]
// fn create_claim_failed_when_already_exist(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//         let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
//
//         assert_noop!(
//             PoeModule::create_claim(Origin::signed(1),claim.clone()),
//              Error::<Test>::ProofAlreadyExist
//         );
//
//
//     }
//     );
// }
//
// #[test]
// fn revoke_claim_works(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//         let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
//
//         assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
//     })
// }
//
// #[test]
// fn revoke_claim_failed_when_claim_is_not_exist(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//
//         assert_noop!(
//             PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
//             Error::<Test>::ClaimNotExist
//         );
//     })
// }
//
// #[test]
// fn transfer_claim_works(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//         let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
//         assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(), 2));
//         assert_eq!(Poes::<Test>::get(&claim),(2,frame_system::Module::<Test>::block_number()));
//
//     })
// }
// #[test]
// fn transfer_claim_works_not_claim_owner(){
//     new_test_ext().execute_with(||{
//         let claim = vec![0,1];
//         let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
//         assert_noop!(
//             PoeModule::transfer_claim(Origin::signed(2),claim.clone(), 1),
//             Error::<Test>::NotClaimOwner
//             );
//     })
// }
//




