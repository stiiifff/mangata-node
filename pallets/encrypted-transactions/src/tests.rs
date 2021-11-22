// Copyright (C) 2020 Mangata team
#![allow(non_snake_case)]

use super::*;
use crate::mock::*;
use frame_support::assert_err;
use sp_runtime::traits::BlakeTwo256;

#[test]
fn W_submit_doubly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let builder: u128 = 2;

        let call = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(1, 1, 1),
        ))];

        let doubly_encrypted_call = call.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            1,
            2,
            3,
        )
        .unwrap();
        let expected_number_of_tx_in_queue = 1;
        assert_eq!(
            EncryptedTX::doubly_encrypted_queue(&builder).len(),
            expected_number_of_tx_in_queue
        );

        let identifier = EncryptedTX::doubly_encrypted_queue(&2)[0];

        let txn_registry_details = TxnRegistryDetails {
            doubly_encrypted_call: doubly_encrypted_call,
            user: 1,
            nonce: 1,
            weight: 1,
            builder: 2,
            executor: 3,
            singly_encrypted_call: None,
            decrypted_call: None,
        };

        assert_eq!(
            EncryptedTX::txn_registry(identifier),
            Some(txn_registry_details)
        );
        let doubly_encrypted_event =
            TestEvent::encrypted(Event::<Test>::DoublyEncryptedTxnSubmitted(1, 1, identifier));

        assert!(System::events()
            .iter()
            .any(|record| record.event == doubly_encrypted_event));
    });
}

#[test]
fn NW_submit_doubly_BalanceTooLow() {
    new_test_ext().execute_with(|| {
        let doubly_encrypted_call = vec![1, 2, 3];
        assert_err!(
            EncryptedTX::submit_doubly_encrypted_transaction(
                Origin::signed(1),
                doubly_encrypted_call,
                1,
                1,
                2,
                3,
            ),
            Error::<Test>::BalanceTooLowForFee,
        );
    });
}

#[test]
fn NW_submit_doubly_transaction_too_long() {
    new_test_ext().execute_with(|| {
        let maxLength = <Test as Trait>::DoublyEncryptedCallMaxLength::get();

        let doubly_encrypted_call = vec![0; maxLength as usize + 1];

        EncryptedTX::create_new_token(&1, 1000000000000000);

        assert_err!(
            EncryptedTX::submit_doubly_encrypted_transaction(
                Origin::signed(1),
                doubly_encrypted_call,
                1,
                1,
                2,
                3,
            ),
            Error::<Test>::DoublyEncryptedCallMaxLengthExceeded,
        );
    });
}

#[test]
#[ignore]
fn NW_submit_doubly_same_transaction_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let call = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(1, 1, 1),
        ))];

        let doubly_encrypted_call = call.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            1,
            2,
            3,
        )
        .unwrap();

        assert_err!(
            EncryptedTX::submit_doubly_encrypted_transaction(
                Origin::signed(1),
                doubly_encrypted_call,
                1,
                1,
                2,
                3,
            ),
            Error::<Test>::TransactionAlreadyInQueue,
        );

        let expected_number_of_tx_in_queue = 1;
        assert_eq!(
            EncryptedTX::doubly_encrypted_queue(&2).len(),
            expected_number_of_tx_in_queue
        );
    });
}

#[test]
fn W_submit_singly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let callForDoubly = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(1, 1, 1),
        ))];

        let callForSingly = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(1, 1, 2),
        ))];

        let doubly_encrypted_call = callForDoubly.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            1,
            2,
            3,
        )
        .unwrap();

        let identifier = EncryptedTX::doubly_encrypted_queue(&2)[0];

        let singly_encrypted_call = callForSingly.encode();

        let mut expected_number_of_tx_in_doubly_queue = 1;
        assert_eq!(
            EncryptedTX::doubly_encrypted_queue(&2).len(),
            expected_number_of_tx_in_doubly_queue
        );

        EncryptedTX::submit_singly_encrypted_transaction(
            Origin::none(),
            identifier,
            singly_encrypted_call.clone(),
        )
        .unwrap();

        expected_number_of_tx_in_doubly_queue = 0;
        let expected_number_of_tx_in_singly_queue = 1;

        assert_eq!(
            EncryptedTX::doubly_encrypted_queue(&2).len(),
            expected_number_of_tx_in_doubly_queue
        );
        assert_eq!(
            EncryptedTX::singly_encrypted_queue(&3).len(),
            expected_number_of_tx_in_singly_queue
        );
        assert_eq!(EncryptedTX::singly_encrypted_queue(&3)[0], identifier);
        let singly_encrypted_event =
            TestEvent::encrypted(Event::<Test>::SinglyEncryptedTxnSubmitted(1, 1, identifier));

        assert!(System::events()
            .iter()
            .any(|record| record.event == singly_encrypted_event));

        let txn_registry_details = TxnRegistryDetails {
            doubly_encrypted_call: doubly_encrypted_call,
            user: 1,
            nonce: 1,
            weight: 1,
            builder: 2,
            executor: 3,
            singly_encrypted_call: Some(singly_encrypted_call),
            decrypted_call: None,
        };
        assert_eq!(
            EncryptedTX::txn_registry(identifier),
            Some(txn_registry_details)
        );
    });
}

#[test]
fn NW_submit_singly_TxnDoesNotExistsInRegistry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let call = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(1, 1, 1),
        ))];

        let doubly_encrypted_call = call.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            1,
            2,
            3,
        )
        .unwrap();

        let singly_encrypted_call = call.encode();

        let invalid_identifier = BlakeTwo256::hash_of(&1);

        assert_err!(
            EncryptedTX::submit_singly_encrypted_transaction(
                Origin::none(),
                invalid_identifier,
                singly_encrypted_call,
            ),
            Error::<Test>::TxnDoesNotExistsInRegistry,
        );
    });
}

#[test]
fn W_submit_decrypted() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let call = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(10, 0, 10),
        ))];
        let weight = call[0].get_dispatch_info().weight.into();
        let doubly_encrypted_call = call.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            weight,
            2,
            3,
        )
        .unwrap();

        let identifier = EncryptedTX::doubly_encrypted_queue(&2)[0];

        let singly_encrypted_call = call.encode();

        EncryptedTX::submit_singly_encrypted_transaction(
            Origin::none(),
            identifier,
            singly_encrypted_call.clone(),
        )
        .unwrap();

        EncryptedTX::submit_decrypted_transaction(
            Origin::none(),
            identifier,
            singly_encrypted_call.clone(),
            1,
        )
        .unwrap();

        let expected_number_of_tx_in_singly_queue = 0;

        assert_eq!(
            EncryptedTX::singly_encrypted_queue(&3).len(),
            expected_number_of_tx_in_singly_queue
        );

        let txn_registry_details = TxnRegistryDetails {
            doubly_encrypted_call: doubly_encrypted_call,
            user: 1,
            nonce: 1,
            weight: weight,
            builder: 2,
            executor: 3,
            singly_encrypted_call: Some(singly_encrypted_call.clone()),
            decrypted_call: Some(singly_encrypted_call),
        };

        assert_eq!(
            EncryptedTX::txn_registry(identifier),
            Some(txn_registry_details)
        );

        let decrypted_transaction_submitted_event = TestEvent::encrypted(
            Event::<Test>::DecryptedTransactionSubmitted(1, 1, identifier),
        );
        let calls_executed_event =
            TestEvent::encrypted(Event::<Test>::CallsExecuted(1, 1, identifier));

        assert!(System::events()
            .iter()
            .any(|record| record.event == decrypted_transaction_submitted_event));
        assert!(System::events()
            .iter()
            .any(|record| record.event == calls_executed_event));

        assert_eq!(EncryptedTX::balance(0, 10), 10);
    });
}

#[test]
fn NW_submit_decrypted_incorrect_call_weight() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let call = vec![Box::new(<mock::Test as Trait>::Call::Tokens(
            orml_tokens::Call::<Test>::transfer(10, 0, 10),
        ))];

        let doubly_encrypted_call = call.encode();
        EncryptedTX::create_new_token(&1, 1000000000000000);

        EncryptedTX::submit_doubly_encrypted_transaction(
            Origin::signed(1),
            doubly_encrypted_call.clone(),
            1,
            1,
            2,
            3,
        )
        .unwrap();

        let identifier = EncryptedTX::doubly_encrypted_queue(&2)[0];

        let singly_encrypted_call = call.encode();

        EncryptedTX::submit_singly_encrypted_transaction(
            Origin::none(),
            identifier,
            singly_encrypted_call.clone(),
        )
        .unwrap();

        assert_err!(
            EncryptedTX::submit_decrypted_transaction(
                Origin::none(),
                identifier,
                singly_encrypted_call.clone(),
                1
            ),
            Error::<Test>::IncorrectCallWeight,
        );
    });
}

//TODO TxnRecord

