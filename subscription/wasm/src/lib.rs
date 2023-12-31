// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    subscription
    (
        init => init
        addAcceptedPaymentTokens => add_accepted_payment_tokens
        deposit => deposit
        withdraw => withdraw
        getAcceptedFeesTokens => accepted_payment_tokens
        getUserDepositedPayments => user_deposited_payments
        getUserDepositedEgld => user_deposited_egld
        registerService => register_service
        subscribe => subscribe
        unsubscribe => unsubscribe
        getServiceInfo => service_info
        getPendingServices => pending_services
        getSubscribedUsers => subscribed_users
        addPair => add_pair
        removePair => remove_pair
        getSafePrice => get_safe_price
        subtractPayment => subtract_payment
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
