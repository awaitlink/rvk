api_category!("secure"; methods {
    add_app_event,
    check_token,
    get_app_balance,
    get_transactions_history,
    get_user_level,
    send_notification,
    set_counter,
    set_user_level
});

api_method!(get_sms_history, "secure.getSMSHistory");
api_method!(send_sms_notification, "secure.sendSMSNotification");
