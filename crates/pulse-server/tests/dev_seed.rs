mod common;

use axum::http::{Method, StatusCode};
use common::{api, campuses_of, count_rows, fresh_state};
use pulse_protocol::{
    AuthSession, DEV_SEED_CAMPUS_SLUG, DEV_SEED_PASSWORD, DEV_SEED_PEOPLE, SignInRequest, api_path,
};
use pulse_server::seed::seed_dev_campus;

#[tokio::test]
async fn seeding_twice_yields_one_demo_campus_with_every_person() {
    let state = fresh_state().await;
    let first = seed_dev_campus(&state.db).await.expect("first seed");
    let second = seed_dev_campus(&state.db).await.expect("second seed");
    assert_eq!(first, second);
    assert_eq!(
        count_rows(&state, "campus_members", first).await,
        i64::try_from(DEV_SEED_PEOPLE.len()).expect("people count")
    );
    for (email, _, role) in DEV_SEED_PEOPLE {
        let request = SignInRequest {
            email: email.to_owned(),
            password: DEV_SEED_PASSWORD.to_owned(),
        };
        let (status, body) = api(
            &state,
            Method::POST,
            api_path::AUTH_TOKENS,
            None,
            Some(&request),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let session: AuthSession = serde_json::from_value(body).expect("auth session");
        let campuses = campuses_of(&state, &session.token).await.campuses;
        assert_eq!(campuses.len(), 1);
        assert_eq!(campuses[0].slug, DEV_SEED_CAMPUS_SLUG);
        assert_eq!(campuses[0].role, role);
    }
}
