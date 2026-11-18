#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::clone_on_copy,
    clippy::let_unit_value,
    clippy::deref_addrof,
    clippy::explicit_auto_deref,
    clippy::borrow_deref_ref,
    clippy::uninlined_format_args,
    clippy::needless_borrow,
    mismatched_lifetime_syntaxes
)]

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::{Lifetimeable, Lockable, transform_result_dco};
use flutter_rust_bridge::{Handler, IntoIntoDart};
use pulse_protocol::validation::Validate;

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.13.0";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = 2029780457;

flutter_rust_bridge::frb_generated_default_handler!();

fn wire__pulse_protocol__ids__account_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "account_id_as_uuid",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::ids::AccountId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::AccountId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__account_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "account_id_generate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::AccountId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_error_code_from_http_status_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "api_error_code_from_http_status",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_status = <u16>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(
                        pulse_protocol::api::ApiErrorCode::from_http_status(api_status),
                    )?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_error_code_http_status_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "api_error_code_http_status",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::api::ApiErrorCode>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::api::ApiErrorCode::http_status(api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_error_conflict_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "api_error_conflict",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_field = <pulse_protocol::api::InputField>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::api::ApiError::conflict(api_field))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_error_invalid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "api_error_invalid",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_field = <pulse_protocol::api::InputField>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::api::ApiError::invalid(api_field))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_error_new_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "api_error_new",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_code = <pulse_protocol::api::ApiErrorCode>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::api::ApiError::new(api_code))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__campus_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_id_as_uuid",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::ids::CampusId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::CampusId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__campus_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_id_generate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::CampusId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_rooms_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_rooms",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_campus_id = <pulse_protocol::ids::CampusId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::api::api_path::campus_rooms(api_campus_id))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__client_snapshot_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "client_snapshot",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(crate::api::client::client_snapshot().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__create_campus_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "create_campus",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_request =
                <pulse_protocol::campus::CreateCampusRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::create_campus(api_request).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__campus__create_campus_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "create_campus_request_validate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that =
                <pulse_protocol::campus::CreateCampusRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok =
                        pulse_protocol::campus::CreateCampusRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__campus__create_room_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "create_room_request_validate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that =
                <pulse_protocol::campus::CreateRoomRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::campus::CreateRoomRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_email_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_email",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_raw = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::normalize_email(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__refresh_campuses_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "refresh_campuses",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(crate::api::client::refresh_campuses().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__rbac__role_as_str_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "role_as_str",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::rbac::Role::as_str(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__rbac__role_permits_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "role_permits",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_role = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            let api_permission = <pulse_protocol::rbac::Permission>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::rbac::role_permits(api_role, api_permission))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__room_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "room_id_as_uuid",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::RoomId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__room_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "room_id_generate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::RoomId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__select_campus_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "select_campus",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_campus_id = <pulse_protocol::ids::CampusId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::select_campus(api_campus_id).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__session_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "session_id_as_uuid",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::ids::SessionId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::SessionId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__session_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "session_id_generate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::SessionId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__sign_in_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "sign_in",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_request = <pulse_protocol::auth::SignInRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::sign_in(api_request).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__auth__sign_in_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "sign_in_request_validate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::auth::SignInRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::auth::SignInRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__sign_out_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "sign_out",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(crate::api::client::sign_out().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__sign_up_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "sign_up",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_request = <pulse_protocol::auth::SignUpRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::sign_up(api_request).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__auth__sign_up_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "sign_up_request_validate",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <pulse_protocol::auth::SignUpRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::auth::SignUpRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__validate_display_name_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_display_name",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_raw = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_display_name(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__validate_password_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_password",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_raw = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_password(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__validate_slug_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_slug",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_raw = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_slug(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}

#[allow(clippy::unnecessary_literal_unwrap)]
const _: fn() = || {
    {
        let AccountId_ = None::<pulse_protocol::ids::AccountId>.unwrap();
        let _: uuid::Uuid = AccountId_.0;
    }
    {
        let AccountProfile = None::<pulse_protocol::auth::AccountProfile>.unwrap();
        let _: pulse_protocol::ids::AccountId = AccountProfile.account_id;
        let _: String = AccountProfile.email;
        let _: String = AccountProfile.display_name;
    }
    {
        let ApiError = None::<pulse_protocol::api::ApiError>.unwrap();
        let _: pulse_protocol::api::ApiErrorCode = ApiError.code;
        let _: Option<pulse_protocol::api::InputField> = ApiError.field;
    }
    {
        let CampusId_ = None::<pulse_protocol::ids::CampusId>.unwrap();
        let _: uuid::Uuid = CampusId_.0;
    }
    {
        let CampusSummary = None::<pulse_protocol::campus::CampusSummary>.unwrap();
        let _: pulse_protocol::ids::CampusId = CampusSummary.campus_id;
        let _: String = CampusSummary.slug;
        let _: String = CampusSummary.display_name;
        let _: pulse_protocol::rbac::Role = CampusSummary.role;
    }
    {
        let ClientSnapshot = None::<pulse_protocol::snapshot::ClientSnapshot>.unwrap();
        let _: pulse_protocol::snapshot::ConnectionSnapshot = ClientSnapshot.network;
        let _: pulse_protocol::snapshot::SessionSnapshot = ClientSnapshot.session;
    }
    {
        let ConnectionSnapshot = None::<pulse_protocol::snapshot::ConnectionSnapshot>.unwrap();
        let _: pulse_protocol::version::ProtocolVersion = ConnectionSnapshot.protocol_version;
        let _: pulse_protocol::snapshot::ConnectionPhase = ConnectionSnapshot.connection;
        let _: Option<u32> = ConnectionSnapshot.retry_in_ms;
        let _: Option<pulse_protocol::ids::CampusId> = ConnectionSnapshot.campus_id;
        let _: Option<pulse_protocol::ids::SessionId> = ConnectionSnapshot.session_id;
        let _: u32 = ConnectionSnapshot.attempt;
        let _: Option<pulse_protocol::frames::VersionMismatch> = ConnectionSnapshot.rejection;
        let _: Option<pulse_protocol::frames::HandshakeRejection> =
            ConnectionSnapshot.handshake_rejection;
    }
    {
        let CreateCampusRequest = None::<pulse_protocol::campus::CreateCampusRequest>.unwrap();
        let _: String = CreateCampusRequest.slug;
        let _: String = CreateCampusRequest.display_name;
    }
    {
        let CreateRoomRequest = None::<pulse_protocol::campus::CreateRoomRequest>.unwrap();
        let _: String = CreateRoomRequest.slug;
    }
    {
        let ProtocolVersion_ = None::<pulse_protocol::version::ProtocolVersion>.unwrap();
        let _: u16 = ProtocolVersion_.0;
    }
    {
        let Room = None::<pulse_protocol::campus::Room>.unwrap();
        let _: pulse_protocol::ids::RoomId = Room.room_id;
        let _: pulse_protocol::ids::CampusId = Room.campus_id;
        let _: String = Room.slug;
    }
    {
        let RoomId_ = None::<pulse_protocol::ids::RoomId>.unwrap();
        let _: uuid::Uuid = RoomId_.0;
    }
    {
        let SessionId_ = None::<pulse_protocol::ids::SessionId>.unwrap();
        let _: uuid::Uuid = SessionId_.0;
    }
    {
        let SessionSnapshot = None::<pulse_protocol::snapshot::SessionSnapshot>.unwrap();
        let _: pulse_protocol::snapshot::AuthPhase = SessionSnapshot.auth;
        let _: Option<pulse_protocol::auth::AccountProfile> = SessionSnapshot.account;
        let _: Vec<pulse_protocol::campus::CampusSummary> = SessionSnapshot.campuses;
        let _: Option<pulse_protocol::ids::CampusId> = SessionSnapshot.active_campus_id;
        let _: Vec<pulse_protocol::campus::Room> = SessionSnapshot.rooms;
        let _: bool = SessionSnapshot.busy;
        let _: Option<pulse_protocol::api::ApiError> = SessionSnapshot.error;
    }
    {
        let SignInRequest = None::<pulse_protocol::auth::SignInRequest>.unwrap();
        let _: String = SignInRequest.email;
        let _: String = SignInRequest.password;
    }
    {
        let SignUpRequest = None::<pulse_protocol::auth::SignUpRequest>.unwrap();
        let _: String = SignUpRequest.email;
        let _: String = SignUpRequest.display_name;
        let _: String = SignUpRequest.password;
    }
    {
        let VersionMismatch = None::<pulse_protocol::frames::VersionMismatch>.unwrap();
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.supported;
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.received;
    }
};

impl SseDecode for String {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for uuid::Uuid {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return uuid::Uuid::from_slice(&inner).expect("fail to decode uuid");
    }
}

impl SseDecode for pulse_protocol::ids::AccountId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::AccountId(var_field0);
    }
}

impl SseDecode for pulse_protocol::auth::AccountProfile {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_accountId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        return pulse_protocol::auth::AccountProfile {
            account_id: var_accountId,
            email: var_email,
            display_name: var_displayName,
        };
    }
}

impl SseDecode for pulse_protocol::api::ApiError {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_code = <pulse_protocol::api::ApiErrorCode>::sse_decode(deserializer);
        let mut var_field = <Option<pulse_protocol::api::InputField>>::sse_decode(deserializer);
        return pulse_protocol::api::ApiError {
            code: var_code,
            field: var_field,
        };
    }
}

impl SseDecode for pulse_protocol::api::ApiErrorCode {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::api::ApiErrorCode::InvalidInput,
            1 => pulse_protocol::api::ApiErrorCode::Unauthenticated,
            2 => pulse_protocol::api::ApiErrorCode::Forbidden,
            3 => pulse_protocol::api::ApiErrorCode::NotFound,
            4 => pulse_protocol::api::ApiErrorCode::Conflict,
            5 => pulse_protocol::api::ApiErrorCode::Unavailable,
            6 => pulse_protocol::api::ApiErrorCode::Internal,
            _ => unreachable!("Invalid variant for ApiErrorCode: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::AuthPhase {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::AuthPhase::SignedOut,
            1 => pulse_protocol::snapshot::AuthPhase::Authenticating,
            2 => pulse_protocol::snapshot::AuthPhase::SignedIn,
            _ => unreachable!("Invalid variant for AuthPhase: {}", inner),
        };
    }
}

impl SseDecode for bool {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

impl SseDecode for pulse_protocol::ids::CampusId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::CampusId(var_field0);
    }
}

impl SseDecode for pulse_protocol::campus::CampusSummary {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_campusId = <pulse_protocol::ids::CampusId>::sse_decode(deserializer);
        let mut var_slug = <String>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_role = <pulse_protocol::rbac::Role>::sse_decode(deserializer);
        return pulse_protocol::campus::CampusSummary {
            campus_id: var_campusId,
            slug: var_slug,
            display_name: var_displayName,
            role: var_role,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ClientSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_network =
            <pulse_protocol::snapshot::ConnectionSnapshot>::sse_decode(deserializer);
        let mut var_session = <pulse_protocol::snapshot::SessionSnapshot>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ClientSnapshot {
            network: var_network,
            session: var_session,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ConnectionPhase {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::ConnectionPhase::Disconnected,
            1 => pulse_protocol::snapshot::ConnectionPhase::Connecting,
            2 => pulse_protocol::snapshot::ConnectionPhase::Handshaking,
            3 => pulse_protocol::snapshot::ConnectionPhase::Connected,
            4 => pulse_protocol::snapshot::ConnectionPhase::Backoff,
            _ => unreachable!("Invalid variant for ConnectionPhase: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ConnectionSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_protocolVersion =
            <pulse_protocol::version::ProtocolVersion>::sse_decode(deserializer);
        let mut var_connection =
            <pulse_protocol::snapshot::ConnectionPhase>::sse_decode(deserializer);
        let mut var_retryInMs = <Option<u32>>::sse_decode(deserializer);
        let mut var_campusId = <Option<pulse_protocol::ids::CampusId>>::sse_decode(deserializer);
        let mut var_sessionId = <Option<pulse_protocol::ids::SessionId>>::sse_decode(deserializer);
        let mut var_attempt = <u32>::sse_decode(deserializer);
        let mut var_rejection =
            <Option<pulse_protocol::frames::VersionMismatch>>::sse_decode(deserializer);
        let mut var_handshakeRejection =
            <Option<pulse_protocol::frames::HandshakeRejection>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ConnectionSnapshot {
            protocol_version: var_protocolVersion,
            connection: var_connection,
            retry_in_ms: var_retryInMs,
            campus_id: var_campusId,
            session_id: var_sessionId,
            attempt: var_attempt,
            rejection: var_rejection,
            handshake_rejection: var_handshakeRejection,
        };
    }
}

impl SseDecode for pulse_protocol::campus::CreateCampusRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_slug = <String>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        return pulse_protocol::campus::CreateCampusRequest {
            slug: var_slug,
            display_name: var_displayName,
        };
    }
}

impl SseDecode for pulse_protocol::campus::CreateRoomRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_slug = <String>::sse_decode(deserializer);
        return pulse_protocol::campus::CreateRoomRequest { slug: var_slug };
    }
}

impl SseDecode for pulse_protocol::frames::HandshakeRejection {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::frames::HandshakeRejection::Unauthenticated,
            1 => pulse_protocol::frames::HandshakeRejection::CampusUnavailable,
            _ => unreachable!("Invalid variant for HandshakeRejection: {}", inner),
        };
    }
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for pulse_protocol::api::InputField {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::api::InputField::Email,
            1 => pulse_protocol::api::InputField::DisplayName,
            2 => pulse_protocol::api::InputField::Password,
            3 => pulse_protocol::api::InputField::Slug,
            _ => unreachable!("Invalid variant for InputField: {}", inner),
        };
    }
}

impl SseDecode for Vec<pulse_protocol::campus::CampusSummary> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::campus::CampusSummary>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<u8> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::campus::Room> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::campus::Room>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Option<pulse_protocol::auth::AccountProfile> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::auth::AccountProfile>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::api::ApiError> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::api::ApiError>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::CampusId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::CampusId>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::frames::HandshakeRejection> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::frames::HandshakeRejection>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::api::InputField> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::api::InputField>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::SessionId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::SessionId>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<u32> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<u32>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::frames::VersionMismatch> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::frames::VersionMismatch>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for pulse_protocol::rbac::Permission {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::rbac::Permission::ViewCampus,
            1 => pulse_protocol::rbac::Permission::ViewRooms,
            2 => pulse_protocol::rbac::Permission::CreateRoom,
            3 => pulse_protocol::rbac::Permission::ManageMembers,
            4 => pulse_protocol::rbac::Permission::ManageCampus,
            _ => unreachable!("Invalid variant for Permission: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::version::ProtocolVersion {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <u16>::sse_decode(deserializer);
        return pulse_protocol::version::ProtocolVersion(var_field0);
    }
}

impl SseDecode for pulse_protocol::rbac::Role {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::rbac::Role::Owner,
            1 => pulse_protocol::rbac::Role::Admin,
            2 => pulse_protocol::rbac::Role::Member,
            3 => pulse_protocol::rbac::Role::Guest,
            _ => unreachable!("Invalid variant for Role: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::campus::Room {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_campusId = <pulse_protocol::ids::CampusId>::sse_decode(deserializer);
        let mut var_slug = <String>::sse_decode(deserializer);
        return pulse_protocol::campus::Room {
            room_id: var_roomId,
            campus_id: var_campusId,
            slug: var_slug,
        };
    }
}

impl SseDecode for pulse_protocol::ids::RoomId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::RoomId(var_field0);
    }
}

impl SseDecode for pulse_protocol::ids::SessionId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::SessionId(var_field0);
    }
}

impl SseDecode for pulse_protocol::snapshot::SessionSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_auth = <pulse_protocol::snapshot::AuthPhase>::sse_decode(deserializer);
        let mut var_account =
            <Option<pulse_protocol::auth::AccountProfile>>::sse_decode(deserializer);
        let mut var_campuses =
            <Vec<pulse_protocol::campus::CampusSummary>>::sse_decode(deserializer);
        let mut var_activeCampusId =
            <Option<pulse_protocol::ids::CampusId>>::sse_decode(deserializer);
        let mut var_rooms = <Vec<pulse_protocol::campus::Room>>::sse_decode(deserializer);
        let mut var_busy = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::SessionSnapshot {
            auth: var_auth,
            account: var_account,
            campuses: var_campuses,
            active_campus_id: var_activeCampusId,
            rooms: var_rooms,
            busy: var_busy,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::auth::SignInRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_password = <String>::sse_decode(deserializer);
        return pulse_protocol::auth::SignInRequest {
            email: var_email,
            password: var_password,
        };
    }
}

impl SseDecode for pulse_protocol::auth::SignUpRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_password = <String>::sse_decode(deserializer);
        return pulse_protocol::auth::SignUpRequest {
            email: var_email,
            display_name: var_displayName,
            password: var_password,
        };
    }
}

impl SseDecode for u16 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u16::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u8 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

impl SseDecode for pulse_protocol::frames::VersionMismatch {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_supported =
            <pulse_protocol::version::ProtocolVersion>::sse_decode(deserializer);
        let mut var_received = <pulse_protocol::version::ProtocolVersion>::sse_decode(deserializer);
        return pulse_protocol::frames::VersionMismatch {
            supported: var_supported,
            received: var_received,
        };
    }
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    match func_id {
        2 => wire__pulse_protocol__ids__account_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        3 => wire__pulse_protocol__ids__account_id_generate_impl(port, ptr, rust_vec_len, data_len),
        4 => wire__pulse_protocol__api__api_error_code_from_http_status_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        5 => wire__pulse_protocol__api__api_error_code_http_status_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        6 => wire__pulse_protocol__api__api_error_conflict_impl(port, ptr, rust_vec_len, data_len),
        7 => wire__pulse_protocol__api__api_error_invalid_impl(port, ptr, rust_vec_len, data_len),
        8 => wire__pulse_protocol__api__api_error_new_impl(port, ptr, rust_vec_len, data_len),
        9 => wire__pulse_protocol__ids__campus_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        10 => wire__pulse_protocol__ids__campus_id_generate_impl(port, ptr, rust_vec_len, data_len),
        11 => wire__pulse_protocol__api__api_path__campus_rooms_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        12 => wire__crate__api__client__client_snapshot_impl(port, ptr, rust_vec_len, data_len),
        13 => wire__crate__api__client__create_campus_impl(port, ptr, rust_vec_len, data_len),
        14 => wire__pulse_protocol__campus__create_campus_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        15 => wire__pulse_protocol__campus__create_room_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        16 => wire__pulse_protocol__validation__normalize_email_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        17 => wire__crate__api__client__refresh_campuses_impl(port, ptr, rust_vec_len, data_len),
        18 => wire__pulse_protocol__rbac__role_as_str_impl(port, ptr, rust_vec_len, data_len),
        19 => wire__pulse_protocol__rbac__role_permits_impl(port, ptr, rust_vec_len, data_len),
        20 => wire__pulse_protocol__ids__room_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        21 => wire__pulse_protocol__ids__room_id_generate_impl(port, ptr, rust_vec_len, data_len),
        22 => wire__crate__api__client__select_campus_impl(port, ptr, rust_vec_len, data_len),
        23 => wire__pulse_protocol__ids__session_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        24 => {
            wire__pulse_protocol__ids__session_id_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        25 => wire__crate__api__client__sign_in_impl(port, ptr, rust_vec_len, data_len),
        26 => wire__pulse_protocol__auth__sign_in_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        27 => wire__crate__api__client__sign_out_impl(port, ptr, rust_vec_len, data_len),
        28 => wire__crate__api__client__sign_up_impl(port, ptr, rust_vec_len, data_len),
        29 => wire__pulse_protocol__auth__sign_up_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        30 => wire__pulse_protocol__validation__validate_display_name_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        31 => wire__pulse_protocol__validation__validate_password_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        32 => {
            wire__pulse_protocol__validation__validate_slug_impl(port, ptr, rust_vec_len, data_len)
        }
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    match func_id {
        _ => unreachable!(),
    }
}

impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::AccountId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::AccountId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::AccountId>>
    for pulse_protocol::ids::AccountId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::AccountId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::auth::AccountProfile> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.account_id.into_into_dart().into_dart(),
            self.0.email.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::auth::AccountProfile>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::auth::AccountProfile>>
    for pulse_protocol::auth::AccountProfile
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::auth::AccountProfile> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::api::ApiError> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.code.into_into_dart().into_dart(),
            self.0.field.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::api::ApiError>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::api::ApiError>>
    for pulse_protocol::api::ApiError
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::api::ApiError> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::api::ApiErrorCode> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::api::ApiErrorCode::InvalidInput => 0.into_dart(),
            pulse_protocol::api::ApiErrorCode::Unauthenticated => 1.into_dart(),
            pulse_protocol::api::ApiErrorCode::Forbidden => 2.into_dart(),
            pulse_protocol::api::ApiErrorCode::NotFound => 3.into_dart(),
            pulse_protocol::api::ApiErrorCode::Conflict => 4.into_dart(),
            pulse_protocol::api::ApiErrorCode::Unavailable => 5.into_dart(),
            pulse_protocol::api::ApiErrorCode::Internal => 6.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::api::ApiErrorCode>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::api::ApiErrorCode>>
    for pulse_protocol::api::ApiErrorCode
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::api::ApiErrorCode> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::AuthPhase> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::AuthPhase::SignedOut => 0.into_dart(),
            pulse_protocol::snapshot::AuthPhase::Authenticating => 1.into_dart(),
            pulse_protocol::snapshot::AuthPhase::SignedIn => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::AuthPhase>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::AuthPhase>>
    for pulse_protocol::snapshot::AuthPhase
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::AuthPhase> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::CampusId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::CampusId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::CampusId>>
    for pulse_protocol::ids::CampusId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::CampusId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::CampusSummary> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.slug.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.role.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::CampusSummary>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::CampusSummary>>
    for pulse_protocol::campus::CampusSummary
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::CampusSummary> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ClientSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.network.into_into_dart().into_dart(),
            self.0.session.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ClientSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ClientSnapshot>>
    for pulse_protocol::snapshot::ClientSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ClientSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ConnectionPhase> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::ConnectionPhase::Disconnected => 0.into_dart(),
            pulse_protocol::snapshot::ConnectionPhase::Connecting => 1.into_dart(),
            pulse_protocol::snapshot::ConnectionPhase::Handshaking => 2.into_dart(),
            pulse_protocol::snapshot::ConnectionPhase::Connected => 3.into_dart(),
            pulse_protocol::snapshot::ConnectionPhase::Backoff => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ConnectionPhase>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ConnectionPhase>>
    for pulse_protocol::snapshot::ConnectionPhase
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ConnectionPhase> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ConnectionSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.protocol_version.into_into_dart().into_dart(),
            self.0.connection.into_into_dart().into_dart(),
            self.0.retry_in_ms.into_into_dart().into_dart(),
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.session_id.into_into_dart().into_dart(),
            self.0.attempt.into_into_dart().into_dart(),
            self.0.rejection.into_into_dart().into_dart(),
            self.0.handshake_rejection.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ConnectionSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ConnectionSnapshot>>
    for pulse_protocol::snapshot::ConnectionSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ConnectionSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::CreateCampusRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.slug.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::CreateCampusRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::CreateCampusRequest>>
    for pulse_protocol::campus::CreateCampusRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::CreateCampusRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::CreateRoomRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.slug.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::CreateRoomRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::CreateRoomRequest>>
    for pulse_protocol::campus::CreateRoomRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::CreateRoomRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::frames::HandshakeRejection> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::frames::HandshakeRejection::Unauthenticated => 0.into_dart(),
            pulse_protocol::frames::HandshakeRejection::CampusUnavailable => 1.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::frames::HandshakeRejection>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::frames::HandshakeRejection>>
    for pulse_protocol::frames::HandshakeRejection
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::frames::HandshakeRejection> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::api::InputField> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::api::InputField::Email => 0.into_dart(),
            pulse_protocol::api::InputField::DisplayName => 1.into_dart(),
            pulse_protocol::api::InputField::Password => 2.into_dart(),
            pulse_protocol::api::InputField::Slug => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::api::InputField>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::api::InputField>>
    for pulse_protocol::api::InputField
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::api::InputField> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::rbac::Permission> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::rbac::Permission::ViewCampus => 0.into_dart(),
            pulse_protocol::rbac::Permission::ViewRooms => 1.into_dart(),
            pulse_protocol::rbac::Permission::CreateRoom => 2.into_dart(),
            pulse_protocol::rbac::Permission::ManageMembers => 3.into_dart(),
            pulse_protocol::rbac::Permission::ManageCampus => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::rbac::Permission>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::rbac::Permission>>
    for pulse_protocol::rbac::Permission
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::rbac::Permission> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::version::ProtocolVersion> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::version::ProtocolVersion>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::version::ProtocolVersion>>
    for pulse_protocol::version::ProtocolVersion
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::version::ProtocolVersion> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::rbac::Role> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::rbac::Role::Owner => 0.into_dart(),
            pulse_protocol::rbac::Role::Admin => 1.into_dart(),
            pulse_protocol::rbac::Role::Member => 2.into_dart(),
            pulse_protocol::rbac::Role::Guest => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::rbac::Role>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::rbac::Role>>
    for pulse_protocol::rbac::Role
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::rbac::Role> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::Room> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.slug.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::Room>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::Room>>
    for pulse_protocol::campus::Room
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::Room> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::RoomId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::RoomId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::RoomId>>
    for pulse_protocol::ids::RoomId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::RoomId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::SessionId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::SessionId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::SessionId>>
    for pulse_protocol::ids::SessionId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::SessionId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::SessionSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.auth.into_into_dart().into_dart(),
            self.0.account.into_into_dart().into_dart(),
            self.0.campuses.into_into_dart().into_dart(),
            self.0.active_campus_id.into_into_dart().into_dart(),
            self.0.rooms.into_into_dart().into_dart(),
            self.0.busy.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::SessionSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::SessionSnapshot>>
    for pulse_protocol::snapshot::SessionSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::SessionSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::auth::SignInRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.email.into_into_dart().into_dart(),
            self.0.password.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::auth::SignInRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::auth::SignInRequest>>
    for pulse_protocol::auth::SignInRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::auth::SignInRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::auth::SignUpRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.email.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.password.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::auth::SignUpRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::auth::SignUpRequest>>
    for pulse_protocol::auth::SignUpRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::auth::SignUpRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::frames::VersionMismatch> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.supported.into_into_dart().into_dart(),
            self.0.received.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::frames::VersionMismatch>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::frames::VersionMismatch>>
    for pulse_protocol::frames::VersionMismatch
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::frames::VersionMismatch> {
        self.into()
    }
}

impl SseEncode for String {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for uuid::Uuid {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.as_bytes().to_vec(), serializer);
    }
}

impl SseEncode for pulse_protocol::ids::AccountId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::auth::AccountProfile {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.account_id, serializer);
        <String>::sse_encode(self.email, serializer);
        <String>::sse_encode(self.display_name, serializer);
    }
}

impl SseEncode for pulse_protocol::api::ApiError {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::api::ApiErrorCode>::sse_encode(self.code, serializer);
        <Option<pulse_protocol::api::InputField>>::sse_encode(self.field, serializer);
    }
}

impl SseEncode for pulse_protocol::api::ApiErrorCode {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::api::ApiErrorCode::InvalidInput => 0,
                pulse_protocol::api::ApiErrorCode::Unauthenticated => 1,
                pulse_protocol::api::ApiErrorCode::Forbidden => 2,
                pulse_protocol::api::ApiErrorCode::NotFound => 3,
                pulse_protocol::api::ApiErrorCode::Conflict => 4,
                pulse_protocol::api::ApiErrorCode::Unavailable => 5,
                pulse_protocol::api::ApiErrorCode::Internal => 6,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::AuthPhase {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::AuthPhase::SignedOut => 0,
                pulse_protocol::snapshot::AuthPhase::Authenticating => 1,
                pulse_protocol::snapshot::AuthPhase::SignedIn => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for bool {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

impl SseEncode for pulse_protocol::ids::CampusId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::campus::CampusSummary {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::CampusId>::sse_encode(self.campus_id, serializer);
        <String>::sse_encode(self.slug, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <pulse_protocol::rbac::Role>::sse_encode(self.role, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ClientSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::snapshot::ConnectionSnapshot>::sse_encode(self.network, serializer);
        <pulse_protocol::snapshot::SessionSnapshot>::sse_encode(self.session, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ConnectionPhase {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::ConnectionPhase::Disconnected => 0,
                pulse_protocol::snapshot::ConnectionPhase::Connecting => 1,
                pulse_protocol::snapshot::ConnectionPhase::Handshaking => 2,
                pulse_protocol::snapshot::ConnectionPhase::Connected => 3,
                pulse_protocol::snapshot::ConnectionPhase::Backoff => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::ConnectionSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::version::ProtocolVersion>::sse_encode(self.protocol_version, serializer);
        <pulse_protocol::snapshot::ConnectionPhase>::sse_encode(self.connection, serializer);
        <Option<u32>>::sse_encode(self.retry_in_ms, serializer);
        <Option<pulse_protocol::ids::CampusId>>::sse_encode(self.campus_id, serializer);
        <Option<pulse_protocol::ids::SessionId>>::sse_encode(self.session_id, serializer);
        <u32>::sse_encode(self.attempt, serializer);
        <Option<pulse_protocol::frames::VersionMismatch>>::sse_encode(self.rejection, serializer);
        <Option<pulse_protocol::frames::HandshakeRejection>>::sse_encode(
            self.handshake_rejection,
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::campus::CreateCampusRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.slug, serializer);
        <String>::sse_encode(self.display_name, serializer);
    }
}

impl SseEncode for pulse_protocol::campus::CreateRoomRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.slug, serializer);
    }
}

impl SseEncode for pulse_protocol::frames::HandshakeRejection {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::frames::HandshakeRejection::Unauthenticated => 0,
                pulse_protocol::frames::HandshakeRejection::CampusUnavailable => 1,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for pulse_protocol::api::InputField {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::api::InputField::Email => 0,
                pulse_protocol::api::InputField::DisplayName => 1,
                pulse_protocol::api::InputField::Password => 2,
                pulse_protocol::api::InputField::Slug => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for Vec<pulse_protocol::campus::CampusSummary> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::campus::CampusSummary>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<u8> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::campus::Room> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::campus::Room>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::auth::AccountProfile> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::auth::AccountProfile>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::api::ApiError> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::api::ApiError>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::CampusId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::CampusId>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::frames::HandshakeRejection> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::frames::HandshakeRejection>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::api::InputField> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::api::InputField>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::SessionId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::SessionId>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<u32> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <u32>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::frames::VersionMismatch> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::frames::VersionMismatch>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for pulse_protocol::rbac::Permission {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::rbac::Permission::ViewCampus => 0,
                pulse_protocol::rbac::Permission::ViewRooms => 1,
                pulse_protocol::rbac::Permission::CreateRoom => 2,
                pulse_protocol::rbac::Permission::ManageMembers => 3,
                pulse_protocol::rbac::Permission::ManageCampus => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::version::ProtocolVersion {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <u16>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::rbac::Role {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::rbac::Role::Owner => 0,
                pulse_protocol::rbac::Role::Admin => 1,
                pulse_protocol::rbac::Role::Member => 2,
                pulse_protocol::rbac::Role::Guest => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::campus::Room {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::CampusId>::sse_encode(self.campus_id, serializer);
        <String>::sse_encode(self.slug, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::RoomId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::SessionId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::SessionSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::snapshot::AuthPhase>::sse_encode(self.auth, serializer);
        <Option<pulse_protocol::auth::AccountProfile>>::sse_encode(self.account, serializer);
        <Vec<pulse_protocol::campus::CampusSummary>>::sse_encode(self.campuses, serializer);
        <Option<pulse_protocol::ids::CampusId>>::sse_encode(self.active_campus_id, serializer);
        <Vec<pulse_protocol::campus::Room>>::sse_encode(self.rooms, serializer);
        <bool>::sse_encode(self.busy, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::auth::SignInRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.email, serializer);
        <String>::sse_encode(self.password, serializer);
    }
}

impl SseEncode for pulse_protocol::auth::SignUpRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.email, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <String>::sse_encode(self.password, serializer);
    }
}

impl SseEncode for u16 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u16::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u8 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

impl SseEncode for pulse_protocol::frames::VersionMismatch {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::version::ProtocolVersion>::sse_encode(self.supported, serializer);
        <pulse_protocol::version::ProtocolVersion>::sse_encode(self.received, serializer);
    }
}

#[cfg(not(target_family = "wasm"))]
mod io {

    use super::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::{Lifetimeable, Lockable, transform_result_dco};
    use flutter_rust_bridge::{Handler, IntoIntoDart};
    use pulse_protocol::validation::Validate;

    flutter_rust_bridge::frb_generated_boilerplate_io!();
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
