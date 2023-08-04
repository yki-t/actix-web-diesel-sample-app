table! {
    access_logs (id) {
        id -> BigInt,
        ip -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        query -> Nullable<Varchar>,
        user_agent -> Nullable<Varchar>,
        identifier -> Nullable<Varchar>,
        duration -> Nullable<Integer>,
        status_code -> Nullable<Integer>,
        others -> Nullable<Text>,
        requested_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> BigInt,
        typ -> Nullable<Integer>,
        level -> Nullable<Integer>,
        message -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Nullable<BigInt>,
        type_id -> Nullable<Integer>,
        username -> Varchar,
        password -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        remarks -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    sessions (id) {
        id -> Nullable<BigInt>,
        user_id -> Nullable<BigInt>,
        token -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    boards (id) {
        id -> Nullable<BigInt>,
        title -> Varchar,
        junre -> Varchar,
        base_url -> Varchar,
        url -> Nullable<Varchar>,

        remarks -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    threads (id) {
        id -> Nullable<BigInt>,
        board_id -> Nullable<BigInt>,

        title -> Varchar,
        url -> Varchar,
        series_name -> Nullable<Varchar>,
        series_number -> Nullable<Integer>,

        reses -> Integer,
        prev_id -> Nullable<BigInt>,
        next_id -> Nullable<BigInt>,
        momentum -> Nullable<BigInt>,

        remarks -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    reses (id) {
        id -> Nullable<BigInt>,
        thread_id -> Nullable<BigInt>,
        no -> Integer,
        username -> Varchar,
        uid -> Nullable<Varchar>,
        posted_at -> Nullable<Timestamp>,

        res_to_id -> Nullable<BigInt>,
        message -> Nullable<Varchar>,

        remarks -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(sessions -> users (user_id));
joinable!(threads -> boards (board_id));
joinable!(reses -> threads (thread_id));

allow_tables_to_appear_in_same_query!(users, sessions);
allow_tables_to_appear_in_same_query!(boards, threads, reses);

/*
// ref) https://github.com/diesel-rs/diesel/issues/2370#issuecomment-616678443
macro_rules! allow_group_by {
    ($($col: ty),+) => {
        allow_group_by!(($($col),+,); $($col),+);
    };
    ($group_by:ty; $($col_for: ty),+) => {
        $(
            impl
                ::diesel::expression::ValidGrouping<$group_by> for $col_for
            {
                type IsAggregate = ::diesel::expression::is_aggregate::Yes;
            }
        )+
    };
}
allow_group_by!(
    accounts::bank_name,
    accounts::branch_name,
    balances::date,
    balances::created_at
);
*/
