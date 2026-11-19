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
use pulse_protocol::steward::*;
use pulse_protocol::validation::Validate;

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.13.0";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -2057301126;

flutter_rust_bridge::frb_generated_default_handler!();

fn wire__pulse_protocol__steward__StreamWriter_abort_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_abort",
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
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, true,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref_mut()),
                            _ => unreachable!(),
                        }
                    }
                    let mut api_that_guard = api_that_guard.unwrap();
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::StreamWriter::abort(
                        &mut *api_that_guard,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__StreamWriter_begin_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_begin",
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
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, true,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref_mut()),
                            _ => unreachable!(),
                        }
                    }
                    let mut api_that_guard = api_that_guard.unwrap();
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::StreamWriter::begin(
                        &mut *api_that_guard,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__StreamWriter_delta_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_delta",
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
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
            >>::sse_decode(&mut deserializer);
            let api_text = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, true,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref_mut()),
                            _ => unreachable!(),
                        }
                    }
                    let mut api_that_guard = api_that_guard.unwrap();
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::StreamWriter::delta(
                        &mut *api_that_guard,
                        &api_text,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__StreamWriter_end_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_end",
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
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, true,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref_mut()),
                            _ => unreachable!(),
                        }
                    }
                    let mut api_that_guard = api_that_guard.unwrap();
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::StreamWriter::end(
                        &mut *api_that_guard,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__StreamWriter_new_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_new",
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
            let api_stream_id = <pulse_protocol::ids::StreamId>::sse_decode(&mut deserializer);
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_thread_root_id =
                <Option<pulse_protocol::ids::MessageId>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::StreamWriter::new(
                        api_stream_id,
                        api_room_id,
                        api_thread_root_id,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__StreamWriter_next_sequence_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "StreamWriter_next_sequence",
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
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = Ok::<_, ()>(
                        pulse_protocol::steward::StreamWriter::next_sequence(&*api_that_guard),
                    )?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
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
fn wire__pulse_protocol__member__add_member_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "add_member_request_validate",
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
                <pulse_protocol::member::AddMemberRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::member::AddMemberRequest::validate(&api_that)?;
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
fn wire__crate__api__client__archive_inbox_selection_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "archive_inbox_selection",
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
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::archive_inbox_selection().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__member__assignable_roles_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "assignable_roles",
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
            let api_actor = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::member::assignable_roles(api_actor))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__attach_file_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "attach_file",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_file_name = <String>::sse_decode(&mut deserializer);
            let api_bytes = <Vec<u8>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::attach_file(api_room_id, api_file_name, api_bytes)
                                .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__attachment_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "attachment_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::AttachmentId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::AttachmentId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__attachment_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "attachment_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::AttachmentId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__media__attachment_kind_as_str_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "attachment_kind_as_str",
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
            let api_that = <pulse_protocol::media::AttachmentKind>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::media::AttachmentKind::as_str(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__media__attachment_kind_from_content_type_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "attachment_kind_from_content_type",
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
            let api_content_type = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(
                        pulse_protocol::media::AttachmentKind::from_content_type(&api_content_type),
                    )?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__mention__body_segments_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "body_segments",
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
            let api_body = <String>::sse_decode(&mut deserializer);
            let api_mentions =
                <Vec<pulse_protocol::messaging::MentionRef>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::mention::body_segments(
                        &api_body,
                        &api_mentions,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_downloads_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_downloads",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::api::api_path::campus_downloads(
                        api_campus_id,
                    ))?;
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
fn wire__pulse_protocol__api__api_path__campus_invites_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_invites",
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
                        Ok::<_, ()>(pulse_protocol::api::api_path::campus_invites(api_campus_id))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_member_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_member",
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
            let api_account_id = <pulse_protocol::ids::AccountId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::api::api_path::campus_member(
                        api_campus_id,
                        api_account_id,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_members_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_members",
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
                        Ok::<_, ()>(pulse_protocol::api::api_path::campus_members(api_campus_id))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_notes_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_notes",
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
                        Ok::<_, ()>(pulse_protocol::api::api_path::campus_notes(api_campus_id))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__api__api_path__campus_room_uploads_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "campus_room_uploads",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::api::api_path::campus_room_uploads(
                            api_campus_id,
                            api_room_id,
                        ))?;
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
fn wire__crate__api__client__cancel_inbox_reply_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "cancel_inbox_reply",
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
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::cancel_inbox_reply().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__client_nonce_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "client_nonce_as_uuid",
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
            let api_that = <pulse_protocol::ids::ClientNonce>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::ClientNonce::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__client_nonce_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "client_nonce_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::ClientNonce::generate())?;
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
fn wire__crate__api__client__close_thread_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "close_thread",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::close_thread().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__close_vault_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "close_vault",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::close_vault().await)?;
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
fn wire__pulse_protocol__media__create_upload_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "create_upload_request_validate",
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
                <pulse_protocol::media::CreateUploadRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok =
                        pulse_protocol::media::CreateUploadRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__dev_seed__dev_accounts_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "dev_accounts",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::dev_seed::dev_accounts())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__discard_draft_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "discard_draft",
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
            let api_draft_id = <pulse_protocol::ids::DraftId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::discard_draft(api_draft_id).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__discard_message_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "discard_message",
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
            let api_client_nonce =
                <pulse_protocol::ids::ClientNonce>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::discard_message(api_client_nonce).await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__dismiss_reminder_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "dismiss_reminder",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::dismiss_reminder().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__draft_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "draft_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::DraftId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::DraftId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__draft_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "draft_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::DraftId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__link__extract_links_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "extract_links",
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
            let api_body = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::link::extract_links(&api_body))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__mention__extract_mentions_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "extract_mentions",
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
            let api_body = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::mention::extract_mentions(&api_body))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__media__guess_content_type_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "guess_content_type",
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
            let api_file_name = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::media::guess_content_type(&api_file_name);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__inbox_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "inbox_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::InboxSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__init_storage_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "init_storage",
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
            let api_data_dir = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        crate::api::client::init_storage(api_data_dir);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__campus__invite_code_render_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "invite_code_render",
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
            let api_that = <pulse_protocol::campus::InviteCode>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::campus::InviteCode::render(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__campus__join_campus_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "join_campus_request_validate",
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
                <pulse_protocol::campus::JoinCampusRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::campus::JoinCampusRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__load_more_vault_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "load_more_vault",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::load_more_vault().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__lounge_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "lounge_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::LoungeId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::LoungeId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__lounge_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "lounge_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::LoungeId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__lounge_media_state_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "lounge_media_state_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::LoungeMediaState::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__lounge_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "lounge_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::LoungeSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__member__member_manageable_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "member_manageable",
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
            let api_actor = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            let api_target = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::member::member_manageable(
                        api_actor, api_target,
                    ))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__mention__mention_token_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "mention_token",
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
            let api_account_id = <pulse_protocol::ids::AccountId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::mention::mention_token(api_account_id))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__message_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "message_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::MessageId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__message_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "message_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::MessageId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__move_inbox_selection_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "move_inbox_selection",
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
            let api_step = <pulse_protocol::snapshot::InboxStep>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::move_inbox_selection(api_step).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__move_search_selection_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "move_search_selection",
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
            let api_step = <pulse_protocol::snapshot::InboxStep>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::move_search_selection(api_step).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_attachment_ids_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_attachment_ids",
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
            let api_ids = <Vec<pulse_protocol::ids::AttachmentId>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::normalize_attachment_ids(&api_ids)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_content_type_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_content_type",
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
                    let output_ok = pulse_protocol::validation::normalize_content_type(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_display_name_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_display_name",
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
                    let output_ok = pulse_protocol::validation::normalize_display_name(&api_raw)?;
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
fn wire__pulse_protocol__validation__normalize_file_name_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_file_name",
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
                    let output_ok = pulse_protocol::validation::normalize_file_name(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_invite_code_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_invite_code",
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
                    let output_ok = pulse_protocol::validation::normalize_invite_code(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_message_body_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_message_body",
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
                    let output_ok = pulse_protocol::validation::normalize_message_body(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_message_content_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_message_content",
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
            let api_body = <String>::sse_decode(&mut deserializer);
            let api_attachment_ids =
                <Vec<pulse_protocol::ids::AttachmentId>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::normalize_message_content(
                        &api_body,
                        &api_attachment_ids,
                    )?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_note_members_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_note_members",
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
            let api_requested =
                <Vec<pulse_protocol::ids::AccountId>>::sse_decode(&mut deserializer);
            let api_starter = <pulse_protocol::ids::AccountId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::normalize_note_members(
                        &api_requested,
                        api_starter,
                    )?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_reaction_emoji_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_reaction_emoji",
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
                    let output_ok = pulse_protocol::validation::normalize_reaction_emoji(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_room_ids_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_room_ids",
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
            let api_room_ids = <Vec<pulse_protocol::ids::RoomId>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::normalize_room_ids(&api_room_ids)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_search_query_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_search_query",
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
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::validation::normalize_search_query(&api_raw))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__normalize_topic_title_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "normalize_topic_title",
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
                    let output_ok = pulse_protocol::validation::normalize_topic_title(&api_raw)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__note__note_member_key_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "note_member_key",
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
            let api_members = <Vec<pulse_protocol::ids::AccountId>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::note::note_member_key(&api_members))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__note__note_title_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "note_title",
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
            let api_note = <pulse_protocol::note::Note>::sse_decode(&mut deserializer);
            let api_viewer = <pulse_protocol::ids::AccountId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::note::note_title(&api_note, api_viewer))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__open_search_hit_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "open_search_hit",
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
            let api_message_id = <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::open_search_hit(api_message_id).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__open_search_selection_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "open_search_selection",
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
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::open_search_selection().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__open_thread_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "open_thread",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_root_message_id =
                <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::open_thread(api_room_id, api_root_message_id).await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__open_vault_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "open_vault",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_tab = <pulse_protocol::media::VaultTab>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::open_vault(api_room_id, api_tab).await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__shelf__picker_anchor_ms_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "picker_anchor_ms",
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
            let api_clock_ms = <u64>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::shelf::picker_anchor_ms(api_clock_ms))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__shelf__picker_latest_ms_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "picker_latest_ms",
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
            let api_clock_ms = <u64>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::shelf::picker_latest_ms(api_clock_ms))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__react_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "react",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_message_id = <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_emoji = <String>::sse_decode(&mut deserializer);
            let api_action =
                <pulse_protocol::reaction::ReactionAction>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::react(
                                api_room_id,
                                api_message_id,
                                api_emoji,
                                api_action,
                            )
                            .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__reaction__react_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "react_validate",
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
            let api_that = <pulse_protocol::reaction::React>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::reaction::React::validate(&api_that)?;
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
fn wire__pulse_protocol__shelf__reminder_preset_offset_ms_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "reminder_preset_offset_ms",
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
            let api_that = <pulse_protocol::shelf::ReminderPreset>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::shelf::ReminderPreset::offset_ms(api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__retry_draft_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "retry_draft",
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
            let api_draft_id = <pulse_protocol::ids::DraftId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::retry_draft(api_draft_id).await)?;
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
fn wire__pulse_protocol__member__role_assignable_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "role_assignable",
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
            let api_actor = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            let api_role = <pulse_protocol::rbac::Role>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::member::role_assignable(api_actor, api_role))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__rbac__role_is_steward_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "role_is_steward",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::rbac::Role::is_steward(api_that))?;
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
fn wire__crate__api__client__search_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "search",
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
            let api_query = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(crate::api::client::search(api_query).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__search_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "search_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::SearchSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__search__search_terms_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "search_terms",
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
            let api_text = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::search::search_terms(&api_text))?;
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
fn wire__crate__api__client__select_room_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "select_room",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok =
                            Ok::<_, ()>(crate::api::client::select_room(api_room_id).await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__send_message_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "send_message",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_body = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::send_message(api_room_id, api_body).await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__messaging__send_message_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "send_message_validate",
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
            let api_that = <pulse_protocol::messaging::SendMessage>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::messaging::SendMessage::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__send_reply_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "send_reply",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_root_message_id =
                <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_body = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::send_reply(
                                api_room_id,
                                api_root_message_id,
                                api_body,
                            )
                            .await,
                        )?;
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
fn wire__crate__api__client__set_pin_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "set_pin",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_message_id = <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_pinned = <bool>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::set_pin(api_room_id, api_message_id, api_pinned)
                                .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__set_thread_subscription_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "set_thread_subscription",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_root_message_id =
                <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_state =
                <pulse_protocol::thread::SubscriptionState>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::set_thread_subscription(
                                api_room_id,
                                api_root_message_id,
                                api_state,
                            )
                            .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__set_topic_title_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "set_topic_title",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_root_message_id =
                <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_topic_title = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::set_topic_title(
                                api_room_id,
                                api_root_message_id,
                                api_topic_title,
                            )
                            .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__thread__set_topic_title_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "set_topic_title_validate",
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
            let api_that = <pulse_protocol::thread::SetTopicTitle>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::thread::SetTopicTitle::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__shelf_item_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "shelf_item_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::ShelfItemId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::ShelfItemId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__shelf_item_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "shelf_item_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::ShelfItemId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__shelf_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "shelf_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::ShelfSnapshot::default())?;
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
fn wire__crate__api__client__start_inbox_reply_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "start_inbox_reply",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::start_inbox_reply().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__steward_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "steward_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::StewardSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__steward_tool_description_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "steward_tool_description",
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
            let api_that = <pulse_protocol::steward::StewardTool>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::steward::StewardTool::description(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__steward_tool_from_name_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "steward_tool_from_name",
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
            let api_name = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::steward::StewardTool::from_name(&api_name))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__steward_tool_input_schema_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "steward_tool_input_schema",
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
            let api_that = <pulse_protocol::steward::StewardTool>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::steward::StewardTool::input_schema(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__steward_tool_name_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "steward_tool_name",
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
            let api_that = <pulse_protocol::steward::StewardTool>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::steward::StewardTool::name(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__stream_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "stream_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::StreamId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::StreamId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__stream_id_client_nonce_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "stream_id_client_nonce",
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
            let api_that = <pulse_protocol::ids::StreamId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::StreamId::client_nonce(api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__stream_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "stream_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::StreamId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__thread__subscription_state_as_str_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "subscription_state_as_str",
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
                <pulse_protocol::thread::SubscriptionState>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::thread::SubscriptionState::as_str(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__snapshot__thread_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "thread_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::ThreadSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__toggle_inbox_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "toggle_inbox",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::toggle_inbox().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__toggle_search_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "toggle_search",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::toggle_search().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__crate__api__client__toggle_shelf_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "toggle_shelf",
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
                        let output_ok = Ok::<_, ()>(crate::api::client::toggle_shelf().await)?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__ids__tool_call_id_as_uuid_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tool_call_id_as_uuid",
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
            let api_that = <pulse_protocol::ids::ToolCallId>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::ids::ToolCallId::as_uuid(&api_that))?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__ids__tool_call_id_generate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tool_call_id_generate",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::ids::ToolCallId::generate())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__tool_descriptors_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tool_descriptors",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::steward::tool_descriptors())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__steward__tool_error_code_as_str_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tool_error_code_as_str",
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
            let api_that = <pulse_protocol::steward::ToolErrorCode>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        pulse_protocol::steward::ToolErrorCode::as_str(api_that);
                    })?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__member__update_member_request_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "update_member_request_validate",
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
                <pulse_protocol::member::UpdateMemberRequest>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok =
                        pulse_protocol::member::UpdateMemberRequest::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__update_shelf_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "update_shelf",
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
            let api_room_id = <pulse_protocol::ids::RoomId>::sse_decode(&mut deserializer);
            let api_message_id = <pulse_protocol::ids::MessageId>::sse_decode(&mut deserializer);
            let api_action = <pulse_protocol::shelf::ShelfAction>::sse_decode(&mut deserializer);
            let api_preset =
                <Option<pulse_protocol::shelf::ReminderPreset>>::sse_decode(&mut deserializer);
            let api_remind_at_ms = <Option<u64>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| async move {
                transform_result_sse::<_, ()>(
                    (move || async move {
                        let output_ok = Ok::<_, ()>(
                            crate::api::client::update_shelf(
                                api_room_id,
                                api_message_id,
                                api_action,
                                api_preset,
                                api_remind_at_ms,
                            )
                            .await,
                        )?;
                        std::result::Result::Ok(output_ok)
                    })()
                    .await,
                )
            }
        },
    )
}
fn wire__pulse_protocol__shelf__update_shelf_validate_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "update_shelf_validate",
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
            let api_that = <pulse_protocol::shelf::UpdateShelf>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::ApiError>((move || {
                    let output_ok = pulse_protocol::shelf::UpdateShelf::validate(&api_that)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__validate_assignable_role_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_assignable_role",
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
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_assignable_role(api_role)?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__validation__validate_byte_size_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_byte_size",
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
            let api_byte_size = <u64>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_byte_size(api_byte_size)?;
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
fn wire__pulse_protocol__validation__validate_remind_at_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "validate_remind_at",
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
            let api_remind_at_ms = <u64>::sse_decode(&mut deserializer);
            let api_now_ms = <u64>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, pulse_protocol::api::InputField>((move || {
                    let output_ok = pulse_protocol::validation::validate_remind_at(
                        api_remind_at_ms,
                        api_now_ms,
                    )?;
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
fn wire__pulse_protocol__snapshot__vault_snapshot_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "vault_snapshot_default",
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
                    let output_ok =
                        Ok::<_, ()>(pulse_protocol::snapshot::VaultSnapshot::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__pulse_protocol__media__vault_tab_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "vault_tab_default",
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
                    let output_ok = Ok::<_, ()>(pulse_protocol::media::VaultTab::default())?;
                    std::result::Result::Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__client__watch_snapshots_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "watch_snapshots",
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
            let api_sink = <StreamSink<
                pulse_protocol::snapshot::ClientSnapshot,
                flutter_rust_bridge::for_generated::SseCodec,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Ok::<_, ()>({
                        crate::api::client::watch_snapshots(api_sink);
                    })?;
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
        let AddMemberRequest = None::<pulse_protocol::member::AddMemberRequest>.unwrap();
        let _: String = AddMemberRequest.email;
        let _: pulse_protocol::rbac::Role = AddMemberRequest.role;
    }
    {
        let ApiError = None::<pulse_protocol::api::ApiError>.unwrap();
        let _: pulse_protocol::api::ApiErrorCode = ApiError.code;
        let _: Option<pulse_protocol::api::InputField> = ApiError.field;
    }
    {
        let AttachmentId_ = None::<pulse_protocol::ids::AttachmentId>.unwrap();
        let _: uuid::Uuid = AttachmentId_.0;
    }
    {
        let AttachmentView = None::<pulse_protocol::snapshot::AttachmentView>.unwrap();
        let _: pulse_protocol::ids::AttachmentId = AttachmentView.attachment_id;
        let _: String = AttachmentView.file_name;
        let _: String = AttachmentView.content_type;
        let _: pulse_protocol::media::AttachmentKind = AttachmentView.kind;
        let _: u64 = AttachmentView.byte_size;
        let _: Option<String> = AttachmentView.url;
    }
    {
        let AuthToken_ = None::<pulse_protocol::auth::AuthToken>.unwrap();
        let _: String = AuthToken_.0;
    }
    {
        let BodySegment = None::<pulse_protocol::mention::BodySegment>.unwrap();
        let _: pulse_protocol::mention::SegmentKind = BodySegment.kind;
        let _: String = BodySegment.text;
        let _: Option<pulse_protocol::ids::AccountId> = BodySegment.account_id;
        let _: Option<String> = BodySegment.url;
    }
    {
        let CampusId_ = None::<pulse_protocol::ids::CampusId>.unwrap();
        let _: uuid::Uuid = CampusId_.0;
    }
    {
        let CampusInvite = None::<pulse_protocol::campus::CampusInvite>.unwrap();
        let _: pulse_protocol::ids::CampusId = CampusInvite.campus_id;
        let _: String = CampusInvite.code;
        let _: u64 = CampusInvite.expires_at_ms;
    }
    {
        let CampusSummary = None::<pulse_protocol::campus::CampusSummary>.unwrap();
        let _: pulse_protocol::ids::CampusId = CampusSummary.campus_id;
        let _: String = CampusSummary.slug;
        let _: String = CampusSummary.display_name;
        let _: pulse_protocol::rbac::Role = CampusSummary.role;
    }
    {
        let ClientNonce_ = None::<pulse_protocol::ids::ClientNonce>.unwrap();
        let _: uuid::Uuid = ClientNonce_.0;
    }
    {
        let ClientSnapshot = None::<pulse_protocol::snapshot::ClientSnapshot>.unwrap();
        let _: u64 = ClientSnapshot.sequence;
        let _: pulse_protocol::snapshot::ConnectionSnapshot = ClientSnapshot.network;
        let _: pulse_protocol::snapshot::SessionSnapshot = ClientSnapshot.session;
        let _: pulse_protocol::snapshot::TimelineSnapshot = ClientSnapshot.timeline;
        let _: pulse_protocol::snapshot::ThreadSnapshot = ClientSnapshot.thread;
        let _: pulse_protocol::snapshot::InboxSnapshot = ClientSnapshot.inbox;
        let _: pulse_protocol::snapshot::SearchSnapshot = ClientSnapshot.search;
        let _: pulse_protocol::snapshot::VaultSnapshot = ClientSnapshot.vault;
        let _: pulse_protocol::snapshot::ShelfSnapshot = ClientSnapshot.shelf;
        let _: pulse_protocol::snapshot::LoungeSnapshot = ClientSnapshot.lounges;
        let _: pulse_protocol::snapshot::StewardSnapshot = ClientSnapshot.stewards;
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
        let CreateUploadRequest = None::<pulse_protocol::media::CreateUploadRequest>.unwrap();
        let _: String = CreateUploadRequest.file_name;
        let _: String = CreateUploadRequest.content_type;
        let _: u64 = CreateUploadRequest.byte_size;
    }
    {
        let DevAccount = None::<pulse_protocol::dev_seed::DevAccount>.unwrap();
        let _: String = DevAccount.email;
        let _: String = DevAccount.display_name;
        let _: pulse_protocol::rbac::Role = DevAccount.role;
    }
    {
        let DraftAttachmentView = None::<pulse_protocol::snapshot::DraftAttachmentView>.unwrap();
        let _: pulse_protocol::ids::DraftId = DraftAttachmentView.draft_id;
        let _: pulse_protocol::ids::RoomId = DraftAttachmentView.room_id;
        let _: String = DraftAttachmentView.file_name;
        let _: pulse_protocol::media::AttachmentKind = DraftAttachmentView.kind;
        let _: u64 = DraftAttachmentView.byte_size;
        let _: pulse_protocol::snapshot::UploadState = DraftAttachmentView.upload;
    }
    {
        let DraftId_ = None::<pulse_protocol::ids::DraftId>.unwrap();
        let _: uuid::Uuid = DraftId_.0;
    }
    {
        let HighlightSpan = None::<pulse_protocol::snapshot::HighlightSpan>.unwrap();
        let _: String = HighlightSpan.text;
        let _: bool = HighlightSpan.matched;
    }
    {
        let InboxItem = None::<pulse_protocol::snapshot::InboxItem>.unwrap();
        let _: pulse_protocol::snapshot::InboxItemKind = InboxItem.kind;
        let _: pulse_protocol::ids::RoomId = InboxItem.room_id;
        let _: pulse_protocol::ids::MessageId = InboxItem.root_message_id;
        let _: Option<String> = InboxItem.topic_title;
        let _: String = InboxItem.root_author_display_name;
        let _: String = InboxItem.root_preview;
        let _: Option<pulse_protocol::snapshot::ReplyPreview> = InboxItem.latest_reply;
        let _: u32 = InboxItem.reply_count;
        let _: u32 = InboxItem.unread_replies;
        let _: u32 = InboxItem.unread_mentions;
        let _: pulse_protocol::thread::SubscriptionState = InboxItem.subscription;
        let _: pulse_protocol::ids::MessageId = InboxItem.activity_message_id;
    }
    {
        let InboxSnapshot = None::<pulse_protocol::snapshot::InboxSnapshot>.unwrap();
        let _: bool = InboxSnapshot.open;
        let _: Vec<pulse_protocol::snapshot::InboxItem> = InboxSnapshot.items;
        let _: u32 = InboxSnapshot.unread_threads;
        let _: Option<pulse_protocol::ids::MessageId> = InboxSnapshot.selected_root_message_id;
        let _: bool = InboxSnapshot.replying;
    }
    {
        let InviteCode = None::<pulse_protocol::campus::InviteCode>.unwrap();
        let _: pulse_protocol::ids::CampusId = InviteCode.campus_id;
        let _: String = InviteCode.secret;
    }
    {
        let IssuedStewardToken = None::<pulse_protocol::snapshot::IssuedStewardToken>.unwrap();
        let _: pulse_protocol::ids::AccountId = IssuedStewardToken.steward_id;
        let _: String = IssuedStewardToken.display_name;
        let _: pulse_protocol::auth::AuthToken = IssuedStewardToken.token;
    }
    {
        let JoinCampusRequest = None::<pulse_protocol::campus::JoinCampusRequest>.unwrap();
        let _: String = JoinCampusRequest.code;
    }
    {
        let LoungeEntry = None::<pulse_protocol::snapshot::LoungeEntry>.unwrap();
        let _: pulse_protocol::ids::LoungeId = LoungeEntry.lounge_id;
        let _: pulse_protocol::lounge::LoungeScope = LoungeEntry.scope;
        let _: Option<pulse_protocol::ids::RoomId> = LoungeEntry.room_id;
        let _: Option<String> = LoungeEntry.room_slug;
        let _: Vec<pulse_protocol::snapshot::LoungeParticipantView> = LoungeEntry.participants;
        let _: bool = LoungeEntry.joined;
    }
    {
        let LoungeId_ = None::<pulse_protocol::ids::LoungeId>.unwrap();
        let _: uuid::Uuid = LoungeId_.0;
    }
    {
        let LoungeParticipantView =
            None::<pulse_protocol::snapshot::LoungeParticipantView>.unwrap();
        let _: pulse_protocol::ids::AccountId = LoungeParticipantView.account_id;
        let _: String = LoungeParticipantView.display_name;
        let _: bool = LoungeParticipantView.from_self;
        let _: bool = LoungeParticipantView.can_speak;
    }
    {
        let LoungeSnapshot = None::<pulse_protocol::snapshot::LoungeSnapshot>.unwrap();
        let _: Vec<pulse_protocol::snapshot::LoungeEntry> = LoungeSnapshot.lounges;
        let _: Option<pulse_protocol::ids::LoungeId> = LoungeSnapshot.joined_lounge_id;
        let _: bool = LoungeSnapshot.joining;
        let _: Option<pulse_protocol::lounge::LoungeTicket> = LoungeSnapshot.ticket;
        let _: pulse_protocol::snapshot::LoungeMediaState = LoungeSnapshot.media;
        let _: bool = LoungeSnapshot.can_join;
        let _: bool = LoungeSnapshot.can_speak;
        let _: Option<pulse_protocol::api::ApiError> = LoungeSnapshot.error;
    }
    {
        let LoungeTicket = None::<pulse_protocol::lounge::LoungeTicket>.unwrap();
        let _: pulse_protocol::ids::LoungeId = LoungeTicket.lounge_id;
        let _: String = LoungeTicket.livekit_url;
        let _: String = LoungeTicket.room_name;
        let _: String = LoungeTicket.token;
        let _: u64 = LoungeTicket.expires_at_ms;
    }
    {
        let MemberEntry = None::<pulse_protocol::snapshot::MemberEntry>.unwrap();
        let _: pulse_protocol::ids::AccountId = MemberEntry.account_id;
        let _: String = MemberEntry.display_name;
        let _: String = MemberEntry.email;
        let _: pulse_protocol::rbac::Role = MemberEntry.role;
        let _: bool = MemberEntry.manageable;
        let _: bool = MemberEntry.is_self;
    }
    {
        let MentionRef = None::<pulse_protocol::messaging::MentionRef>.unwrap();
        let _: pulse_protocol::ids::AccountId = MentionRef.account_id;
        let _: String = MentionRef.display_name;
    }
    {
        let MessageId_ = None::<pulse_protocol::ids::MessageId>.unwrap();
        let _: uuid::Uuid = MessageId_.0;
    }
    {
        let Note = None::<pulse_protocol::note::Note>.unwrap();
        let _: pulse_protocol::ids::CampusId = Note.campus_id;
        let _: pulse_protocol::ids::RoomId = Note.room_id;
        let _: Vec<pulse_protocol::note::NoteMember> = Note.members;
    }
    {
        let NoteEntry = None::<pulse_protocol::snapshot::NoteEntry>.unwrap();
        let _: pulse_protocol::ids::RoomId = NoteEntry.room_id;
        let _: String = NoteEntry.title;
        let _: u32 = NoteEntry.member_count;
        let _: bool = NoteEntry.group;
    }
    {
        let NoteMember = None::<pulse_protocol::note::NoteMember>.unwrap();
        let _: pulse_protocol::ids::AccountId = NoteMember.account_id;
        let _: String = NoteMember.display_name;
    }
    {
        let ProtocolVersion_ = None::<pulse_protocol::version::ProtocolVersion>.unwrap();
        let _: u16 = ProtocolVersion_.0;
    }
    {
        let React = None::<pulse_protocol::reaction::React>.unwrap();
        let _: pulse_protocol::ids::RoomId = React.room_id;
        let _: pulse_protocol::ids::MessageId = React.message_id;
        let _: String = React.emoji;
        let _: pulse_protocol::reaction::ReactionAction = React.action;
    }
    {
        let ReactionSummary = None::<pulse_protocol::snapshot::ReactionSummary>.unwrap();
        let _: String = ReactionSummary.emoji;
        let _: u32 = ReactionSummary.count;
        let _: bool = ReactionSummary.reacted_by_self;
    }
    {
        let ReplyPreview = None::<pulse_protocol::snapshot::ReplyPreview>.unwrap();
        let _: pulse_protocol::ids::MessageId = ReplyPreview.message_id;
        let _: pulse_protocol::ids::AccountId = ReplyPreview.author_id;
        let _: String = ReplyPreview.author_display_name;
        let _: String = ReplyPreview.preview;
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
        let SearchHit = None::<pulse_protocol::snapshot::SearchHit>.unwrap();
        let _: pulse_protocol::ids::RoomId = SearchHit.room_id;
        let _: pulse_protocol::ids::MessageId = SearchHit.message_id;
        let _: Option<pulse_protocol::ids::MessageId> = SearchHit.thread_root_id;
        let _: String = SearchHit.author_display_name;
        let _: Option<String> = SearchHit.topic_title;
        let _: Vec<pulse_protocol::snapshot::HighlightSpan> = SearchHit.snippet;
        let _: u32 = SearchHit.score;
    }
    {
        let SearchSnapshot = None::<pulse_protocol::snapshot::SearchSnapshot>.unwrap();
        let _: bool = SearchSnapshot.open;
        let _: String = SearchSnapshot.query;
        let _: Vec<pulse_protocol::snapshot::SearchHit> = SearchSnapshot.hits;
        let _: Option<u32> = SearchSnapshot.selected_index;
        let _: bool = SearchSnapshot.searching;
        let _: Option<pulse_protocol::api::ApiError> = SearchSnapshot.error;
    }
    {
        let SendMessage = None::<pulse_protocol::messaging::SendMessage>.unwrap();
        let _: pulse_protocol::ids::RoomId = SendMessage.room_id;
        let _: pulse_protocol::ids::ClientNonce = SendMessage.client_nonce;
        let _: String = SendMessage.body;
        let _: Option<pulse_protocol::ids::MessageId> = SendMessage.thread_root_id;
        let _: Vec<pulse_protocol::ids::AttachmentId> = SendMessage.attachment_ids;
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
        let _: Option<pulse_protocol::ids::RoomId> = SessionSnapshot.active_room_id;
        let _: Option<pulse_protocol::campus::CampusInvite> = SessionSnapshot.invite;
        let _: bool = SessionSnapshot.can_invite;
        let _: Vec<pulse_protocol::snapshot::MemberEntry> = SessionSnapshot.members;
        let _: Vec<pulse_protocol::rbac::Role> = SessionSnapshot.assignable_roles;
        let _: Vec<pulse_protocol::dev_seed::DevAccount> = SessionSnapshot.quick_accounts;
        let _: Vec<pulse_protocol::snapshot::NoteEntry> = SessionSnapshot.notes;
        let _: Vec<pulse_protocol::snapshot::MemberEntry> = SessionSnapshot.note_candidates;
        let _: bool = SessionSnapshot.busy;
        let _: Option<pulse_protocol::api::ApiError> = SessionSnapshot.error;
    }
    {
        let SetTopicTitle = None::<pulse_protocol::thread::SetTopicTitle>.unwrap();
        let _: pulse_protocol::ids::RoomId = SetTopicTitle.room_id;
        let _: pulse_protocol::ids::MessageId = SetTopicTitle.root_message_id;
        let _: String = SetTopicTitle.topic_title;
    }
    {
        let ShelfEntry = None::<pulse_protocol::snapshot::ShelfEntry>.unwrap();
        let _: Option<pulse_protocol::ids::ShelfItemId> = ShelfEntry.shelf_item_id;
        let _: pulse_protocol::ids::RoomId = ShelfEntry.room_id;
        let _: pulse_protocol::ids::MessageId = ShelfEntry.message_id;
        let _: String = ShelfEntry.author_display_name;
        let _: String = ShelfEntry.preview;
        let _: Option<u64> = ShelfEntry.remind_at_ms;
        let _: Option<u64> = ShelfEntry.done_at_ms;
        let _: pulse_protocol::shelf::ShelfBucket = ShelfEntry.bucket;
        let _: bool = ShelfEntry.pending;
    }
    {
        let ShelfItemId_ = None::<pulse_protocol::ids::ShelfItemId>.unwrap();
        let _: uuid::Uuid = ShelfItemId_.0;
    }
    {
        let ShelfSnapshot = None::<pulse_protocol::snapshot::ShelfSnapshot>.unwrap();
        let _: bool = ShelfSnapshot.open;
        let _: Vec<pulse_protocol::snapshot::ShelfEntry> = ShelfSnapshot.due;
        let _: Vec<pulse_protocol::snapshot::ShelfEntry> = ShelfSnapshot.upcoming;
        let _: Vec<pulse_protocol::snapshot::ShelfEntry> = ShelfSnapshot.done;
        let _: u32 = ShelfSnapshot.due_count;
        let _: Option<pulse_protocol::snapshot::ShelfEntry> = ShelfSnapshot.latest_reminder;
        let _: Vec<pulse_protocol::shelf::ReminderPreset> = ShelfSnapshot.presets;
        let _: Option<u64> = ShelfSnapshot.picker_anchor_ms;
        let _: Option<u64> = ShelfSnapshot.picker_latest_ms;
        let _: Option<pulse_protocol::api::ApiError> = ShelfSnapshot.error;
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
        let StewardEntry = None::<pulse_protocol::snapshot::StewardEntry>.unwrap();
        let _: pulse_protocol::ids::AccountId = StewardEntry.steward_id;
        let _: String = StewardEntry.display_name;
        let _: String = StewardEntry.mention_token;
        let _: Vec<pulse_protocol::ids::RoomId> = StewardEntry.room_ids;
        let _: bool = StewardEntry.revoked;
        let _: bool = StewardEntry.manageable;
    }
    {
        let StewardSnapshot = None::<pulse_protocol::snapshot::StewardSnapshot>.unwrap();
        let _: bool = StewardSnapshot.open;
        let _: Vec<pulse_protocol::snapshot::StewardEntry> = StewardSnapshot.stewards;
        let _: bool = StewardSnapshot.can_manage;
        let _: Option<pulse_protocol::snapshot::IssuedStewardToken> = StewardSnapshot.issued;
        let _: bool = StewardSnapshot.pending;
        let _: Option<pulse_protocol::api::ApiError> = StewardSnapshot.error;
    }
    {
        let StreamChunk = None::<pulse_protocol::steward::StreamChunk>.unwrap();
        let _: pulse_protocol::ids::StreamId = StreamChunk.stream_id;
        let _: pulse_protocol::ids::RoomId = StreamChunk.room_id;
        let _: Option<pulse_protocol::ids::MessageId> = StreamChunk.thread_root_id;
        let _: pulse_protocol::steward::StreamPhase = StreamChunk.phase;
        let _: u32 = StreamChunk.sequence;
        let _: String = StreamChunk.text;
    }
    {
        let StreamId_ = None::<pulse_protocol::ids::StreamId>.unwrap();
        let _: uuid::Uuid = StreamId_.0;
    }
    {
        let ThreadPreview = None::<pulse_protocol::snapshot::ThreadPreview>.unwrap();
        let _: Option<String> = ThreadPreview.topic_title;
        let _: u32 = ThreadPreview.reply_count;
        let _: Option<pulse_protocol::snapshot::ReplyPreview> = ThreadPreview.latest_reply;
        let _: bool = ThreadPreview.show_latest_reply;
        let _: Option<pulse_protocol::thread::SubscriptionState> = ThreadPreview.subscription;
        let _: u32 = ThreadPreview.unread_replies;
    }
    {
        let ThreadSnapshot = None::<pulse_protocol::snapshot::ThreadSnapshot>.unwrap();
        let _: Option<pulse_protocol::ids::RoomId> = ThreadSnapshot.room_id;
        let _: Option<pulse_protocol::ids::MessageId> = ThreadSnapshot.root_message_id;
        let _: Option<pulse_protocol::snapshot::TimelineEntry> = ThreadSnapshot.root;
        let _: Vec<pulse_protocol::snapshot::TimelineEntry> = ThreadSnapshot.replies;
        let _: Option<String> = ThreadSnapshot.topic_title;
        let _: Option<pulse_protocol::thread::SubscriptionState> = ThreadSnapshot.subscription;
        let _: bool = ThreadSnapshot.syncing;
        let _: Vec<pulse_protocol::snapshot::StewardEntry> = ThreadSnapshot.mentionable_stewards;
    }
    {
        let TimelineEntry = None::<pulse_protocol::snapshot::TimelineEntry>.unwrap();
        let _: pulse_protocol::ids::ClientNonce = TimelineEntry.client_nonce;
        let _: Option<pulse_protocol::ids::MessageId> = TimelineEntry.message_id;
        let _: pulse_protocol::ids::AccountId = TimelineEntry.author_id;
        let _: String = TimelineEntry.author_display_name;
        let _: String = TimelineEntry.author_mention_token;
        let _: bool = TimelineEntry.from_self;
        let _: String = TimelineEntry.body;
        let _: Vec<pulse_protocol::mention::BodySegment> = TimelineEntry.segments;
        let _: Option<pulse_protocol::ids::MessageId> = TimelineEntry.thread_root_id;
        let _: Vec<pulse_protocol::snapshot::ReactionSummary> = TimelineEntry.reactions;
        let _: Option<pulse_protocol::snapshot::ThreadPreview> = TimelineEntry.thread;
        let _: pulse_protocol::snapshot::DeliveryState = TimelineEntry.delivery;
        let _: Option<pulse_protocol::messaging::MessageRejection> = TimelineEntry.rejection;
        let _: Vec<pulse_protocol::snapshot::AttachmentView> = TimelineEntry.attachments;
        let _: Vec<pulse_protocol::snapshot::DraftAttachmentView> = TimelineEntry.pending_drafts;
        let _: bool = TimelineEntry.pinned;
        let _: bool = TimelineEntry.shelved;
        let _: bool = TimelineEntry.author_is_steward;
        let _: Option<pulse_protocol::snapshot::StreamState> = TimelineEntry.stream;
    }
    {
        let TimelineSnapshot = None::<pulse_protocol::snapshot::TimelineSnapshot>.unwrap();
        let _: Option<pulse_protocol::ids::RoomId> = TimelineSnapshot.room_id;
        let _: Vec<pulse_protocol::snapshot::TimelineEntry> = TimelineSnapshot.entries;
        let _: bool = TimelineSnapshot.syncing;
        let _: Option<pulse_protocol::api::ApiError> = TimelineSnapshot.error;
        let _: Vec<String> = TimelineSnapshot.reaction_palette;
        let _: Vec<pulse_protocol::snapshot::DraftAttachmentView> = TimelineSnapshot.drafts;
        let _: Option<pulse_protocol::ids::MessageId> = TimelineSnapshot.focus_message_id;
        let _: bool = TimelineSnapshot.can_pin;
        let _: bool = TimelineSnapshot.can_upload;
        let _: Vec<pulse_protocol::snapshot::StewardEntry> = TimelineSnapshot.mentionable_stewards;
    }
    {
        let ToolCallId_ = None::<pulse_protocol::ids::ToolCallId>.unwrap();
        let _: uuid::Uuid = ToolCallId_.0;
    }
    {
        let ToolDescriptor = None::<pulse_protocol::steward::ToolDescriptor>.unwrap();
        let _: String = ToolDescriptor.name;
        let _: String = ToolDescriptor.description;
        let _: String = ToolDescriptor.input_schema;
    }
    {
        let UpdateMemberRequest = None::<pulse_protocol::member::UpdateMemberRequest>.unwrap();
        let _: pulse_protocol::rbac::Role = UpdateMemberRequest.role;
    }
    {
        let UpdateShelf = None::<pulse_protocol::shelf::UpdateShelf>.unwrap();
        let _: pulse_protocol::ids::RoomId = UpdateShelf.room_id;
        let _: pulse_protocol::ids::MessageId = UpdateShelf.message_id;
        let _: pulse_protocol::shelf::ShelfAction = UpdateShelf.action;
        let _: Option<u64> = UpdateShelf.remind_at_ms;
    }
    {
        let VaultEntry = None::<pulse_protocol::snapshot::VaultEntry>.unwrap();
        let _: pulse_protocol::media::VaultTab = VaultEntry.tab;
        let _: pulse_protocol::ids::RoomId = VaultEntry.room_id;
        let _: pulse_protocol::ids::MessageId = VaultEntry.message_id;
        let _: String = VaultEntry.author_display_name;
        let _: String = VaultEntry.preview;
        let _: Option<pulse_protocol::snapshot::AttachmentView> = VaultEntry.attachment;
        let _: Option<String> = VaultEntry.link;
        let _: Option<u64> = VaultEntry.pinned_at_ms;
    }
    {
        let VaultSnapshot = None::<pulse_protocol::snapshot::VaultSnapshot>.unwrap();
        let _: bool = VaultSnapshot.open;
        let _: Option<pulse_protocol::ids::RoomId> = VaultSnapshot.room_id;
        let _: pulse_protocol::media::VaultTab = VaultSnapshot.tab;
        let _: Vec<pulse_protocol::snapshot::VaultEntry> = VaultSnapshot.entries;
        let _: bool = VaultSnapshot.loading;
        let _: bool = VaultSnapshot.has_more;
        let _: Option<pulse_protocol::api::ApiError> = VaultSnapshot.error;
    }
    {
        let VersionMismatch = None::<pulse_protocol::frames::VersionMismatch>.unwrap();
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.supported;
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.received;
    }
};

flutter_rust_bridge::frb_generated_moi_arc_impl_value!(
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>
);

impl SseDecode for flutter_rust_bridge::for_generated::anyhow::Error {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <String>::sse_decode(deserializer);
        return flutter_rust_bridge::for_generated::anyhow::anyhow!("{}", inner);
    }
}

impl SseDecode for StreamWriter {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <RustOpaqueMoi<
            flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>,
        >>::sse_decode(deserializer);
        return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(inner);
    }
}

impl SseDecode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>>
{
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return decode_rust_opaque_moi(inner);
    }
}

impl SseDecode
    for StreamSink<
        pulse_protocol::snapshot::ClientSnapshot,
        flutter_rust_bridge::for_generated::SseCodec,
    >
{
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <String>::sse_decode(deserializer);
        return StreamSink::deserialize(inner);
    }
}

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

impl SseDecode for pulse_protocol::member::AddMemberRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_role = <pulse_protocol::rbac::Role>::sse_decode(deserializer);
        return pulse_protocol::member::AddMemberRequest {
            email: var_email,
            role: var_role,
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

impl SseDecode for pulse_protocol::ids::AttachmentId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::AttachmentId(var_field0);
    }
}

impl SseDecode for pulse_protocol::media::AttachmentKind {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::media::AttachmentKind::Image,
            1 => pulse_protocol::media::AttachmentKind::Video,
            2 => pulse_protocol::media::AttachmentKind::Audio,
            3 => pulse_protocol::media::AttachmentKind::File,
            _ => unreachable!("Invalid variant for AttachmentKind: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::AttachmentView {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_attachmentId = <pulse_protocol::ids::AttachmentId>::sse_decode(deserializer);
        let mut var_fileName = <String>::sse_decode(deserializer);
        let mut var_contentType = <String>::sse_decode(deserializer);
        let mut var_kind = <pulse_protocol::media::AttachmentKind>::sse_decode(deserializer);
        let mut var_byteSize = <u64>::sse_decode(deserializer);
        let mut var_url = <Option<String>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::AttachmentView {
            attachment_id: var_attachmentId,
            file_name: var_fileName,
            content_type: var_contentType,
            kind: var_kind,
            byte_size: var_byteSize,
            url: var_url,
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

impl SseDecode for pulse_protocol::auth::AuthToken {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <String>::sse_decode(deserializer);
        return pulse_protocol::auth::AuthToken(var_field0);
    }
}

impl SseDecode for pulse_protocol::mention::BodySegment {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_kind = <pulse_protocol::mention::SegmentKind>::sse_decode(deserializer);
        let mut var_text = <String>::sse_decode(deserializer);
        let mut var_accountId = <Option<pulse_protocol::ids::AccountId>>::sse_decode(deserializer);
        let mut var_url = <Option<String>>::sse_decode(deserializer);
        return pulse_protocol::mention::BodySegment {
            kind: var_kind,
            text: var_text,
            account_id: var_accountId,
            url: var_url,
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

impl SseDecode for pulse_protocol::campus::CampusInvite {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_campusId = <pulse_protocol::ids::CampusId>::sse_decode(deserializer);
        let mut var_code = <String>::sse_decode(deserializer);
        let mut var_expiresAtMs = <u64>::sse_decode(deserializer);
        return pulse_protocol::campus::CampusInvite {
            campus_id: var_campusId,
            code: var_code,
            expires_at_ms: var_expiresAtMs,
        };
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

impl SseDecode for pulse_protocol::ids::ClientNonce {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::ClientNonce(var_field0);
    }
}

impl SseDecode for pulse_protocol::snapshot::ClientSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_sequence = <u64>::sse_decode(deserializer);
        let mut var_network =
            <pulse_protocol::snapshot::ConnectionSnapshot>::sse_decode(deserializer);
        let mut var_session = <pulse_protocol::snapshot::SessionSnapshot>::sse_decode(deserializer);
        let mut var_timeline =
            <pulse_protocol::snapshot::TimelineSnapshot>::sse_decode(deserializer);
        let mut var_thread = <pulse_protocol::snapshot::ThreadSnapshot>::sse_decode(deserializer);
        let mut var_inbox = <pulse_protocol::snapshot::InboxSnapshot>::sse_decode(deserializer);
        let mut var_search = <pulse_protocol::snapshot::SearchSnapshot>::sse_decode(deserializer);
        let mut var_vault = <pulse_protocol::snapshot::VaultSnapshot>::sse_decode(deserializer);
        let mut var_shelf = <pulse_protocol::snapshot::ShelfSnapshot>::sse_decode(deserializer);
        let mut var_lounges = <pulse_protocol::snapshot::LoungeSnapshot>::sse_decode(deserializer);
        let mut var_stewards =
            <pulse_protocol::snapshot::StewardSnapshot>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ClientSnapshot {
            sequence: var_sequence,
            network: var_network,
            session: var_session,
            timeline: var_timeline,
            thread: var_thread,
            inbox: var_inbox,
            search: var_search,
            vault: var_vault,
            shelf: var_shelf,
            lounges: var_lounges,
            stewards: var_stewards,
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

impl SseDecode for pulse_protocol::media::CreateUploadRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_fileName = <String>::sse_decode(deserializer);
        let mut var_contentType = <String>::sse_decode(deserializer);
        let mut var_byteSize = <u64>::sse_decode(deserializer);
        return pulse_protocol::media::CreateUploadRequest {
            file_name: var_fileName,
            content_type: var_contentType,
            byte_size: var_byteSize,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::DeliveryState {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::DeliveryState::Pending,
            1 => pulse_protocol::snapshot::DeliveryState::Confirmed,
            2 => pulse_protocol::snapshot::DeliveryState::Failed,
            _ => unreachable!("Invalid variant for DeliveryState: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::dev_seed::DevAccount {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_role = <pulse_protocol::rbac::Role>::sse_decode(deserializer);
        return pulse_protocol::dev_seed::DevAccount {
            email: var_email,
            display_name: var_displayName,
            role: var_role,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::DraftAttachmentView {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_draftId = <pulse_protocol::ids::DraftId>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_fileName = <String>::sse_decode(deserializer);
        let mut var_kind = <pulse_protocol::media::AttachmentKind>::sse_decode(deserializer);
        let mut var_byteSize = <u64>::sse_decode(deserializer);
        let mut var_upload = <pulse_protocol::snapshot::UploadState>::sse_decode(deserializer);
        return pulse_protocol::snapshot::DraftAttachmentView {
            draft_id: var_draftId,
            room_id: var_roomId,
            file_name: var_fileName,
            kind: var_kind,
            byte_size: var_byteSize,
            upload: var_upload,
        };
    }
}

impl SseDecode for pulse_protocol::ids::DraftId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::DraftId(var_field0);
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

impl SseDecode for pulse_protocol::snapshot::HighlightSpan {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_text = <String>::sse_decode(deserializer);
        let mut var_matched = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::HighlightSpan {
            text: var_text,
            matched: var_matched,
        };
    }
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for pulse_protocol::snapshot::InboxItem {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_kind = <pulse_protocol::snapshot::InboxItemKind>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_rootMessageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_topicTitle = <Option<String>>::sse_decode(deserializer);
        let mut var_rootAuthorDisplayName = <String>::sse_decode(deserializer);
        let mut var_rootPreview = <String>::sse_decode(deserializer);
        let mut var_latestReply =
            <Option<pulse_protocol::snapshot::ReplyPreview>>::sse_decode(deserializer);
        let mut var_replyCount = <u32>::sse_decode(deserializer);
        let mut var_unreadReplies = <u32>::sse_decode(deserializer);
        let mut var_unreadMentions = <u32>::sse_decode(deserializer);
        let mut var_subscription =
            <pulse_protocol::thread::SubscriptionState>::sse_decode(deserializer);
        let mut var_activityMessageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        return pulse_protocol::snapshot::InboxItem {
            kind: var_kind,
            room_id: var_roomId,
            root_message_id: var_rootMessageId,
            topic_title: var_topicTitle,
            root_author_display_name: var_rootAuthorDisplayName,
            root_preview: var_rootPreview,
            latest_reply: var_latestReply,
            reply_count: var_replyCount,
            unread_replies: var_unreadReplies,
            unread_mentions: var_unreadMentions,
            subscription: var_subscription,
            activity_message_id: var_activityMessageId,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::InboxItemKind {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::InboxItemKind::Thread,
            _ => unreachable!("Invalid variant for InboxItemKind: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::InboxSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_open = <bool>::sse_decode(deserializer);
        let mut var_items = <Vec<pulse_protocol::snapshot::InboxItem>>::sse_decode(deserializer);
        let mut var_unreadThreads = <u32>::sse_decode(deserializer);
        let mut var_selectedRootMessageId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_replying = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::InboxSnapshot {
            open: var_open,
            items: var_items,
            unread_threads: var_unreadThreads,
            selected_root_message_id: var_selectedRootMessageId,
            replying: var_replying,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::InboxStep {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::InboxStep::Next,
            1 => pulse_protocol::snapshot::InboxStep::Previous,
            _ => unreachable!("Invalid variant for InboxStep: {}", inner),
        };
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
            4 => pulse_protocol::api::InputField::MessageBody,
            5 => pulse_protocol::api::InputField::TopicTitle,
            6 => pulse_protocol::api::InputField::ReactionEmoji,
            7 => pulse_protocol::api::InputField::FileName,
            8 => pulse_protocol::api::InputField::ContentType,
            9 => pulse_protocol::api::InputField::ByteSize,
            10 => pulse_protocol::api::InputField::Attachments,
            11 => pulse_protocol::api::InputField::RemindAt,
            12 => pulse_protocol::api::InputField::Rooms,
            13 => pulse_protocol::api::InputField::InviteCode,
            14 => pulse_protocol::api::InputField::Role,
            15 => pulse_protocol::api::InputField::NoteMembers,
            _ => unreachable!("Invalid variant for InputField: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::campus::InviteCode {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_campusId = <pulse_protocol::ids::CampusId>::sse_decode(deserializer);
        let mut var_secret = <String>::sse_decode(deserializer);
        return pulse_protocol::campus::InviteCode {
            campus_id: var_campusId,
            secret: var_secret,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::IssuedStewardToken {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_stewardId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_token = <pulse_protocol::auth::AuthToken>::sse_decode(deserializer);
        return pulse_protocol::snapshot::IssuedStewardToken {
            steward_id: var_stewardId,
            display_name: var_displayName,
            token: var_token,
        };
    }
}

impl SseDecode for pulse_protocol::campus::JoinCampusRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_code = <String>::sse_decode(deserializer);
        return pulse_protocol::campus::JoinCampusRequest { code: var_code };
    }
}

impl SseDecode for Vec<String> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<String>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::ids::AccountId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::ids::AccountId>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::ids::AttachmentId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::ids::AttachmentId>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::AttachmentView> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::AttachmentView>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::mention::BodySegment> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::mention::BodySegment>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
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

impl SseDecode for Vec<pulse_protocol::dev_seed::DevAccount> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::dev_seed::DevAccount>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::DraftAttachmentView> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::DraftAttachmentView>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::HighlightSpan> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::HighlightSpan>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::InboxItem> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::InboxItem>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::LoungeEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::LoungeEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::LoungeParticipantView> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::LoungeParticipantView>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::MemberEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::MemberEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::messaging::MentionRef> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::messaging::MentionRef>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::NoteEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::NoteEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::note::NoteMember> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::note::NoteMember>::sse_decode(deserializer));
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

impl SseDecode for Vec<pulse_protocol::snapshot::ReactionSummary> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::ReactionSummary>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::shelf::ReminderPreset> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::shelf::ReminderPreset>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::rbac::Role> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::rbac::Role>::sse_decode(deserializer));
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

impl SseDecode for Vec<pulse_protocol::ids::RoomId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::ids::RoomId>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::SearchHit> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::SearchHit>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::ShelfEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::ShelfEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::StewardEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::StewardEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::TimelineEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::TimelineEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::steward::ToolDescriptor> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::steward::ToolDescriptor>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for Vec<pulse_protocol::snapshot::VaultEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = Vec::with_capacity(len_ as usize);
        for idx_ in 0..len_ {
            ans_.push(<pulse_protocol::snapshot::VaultEntry>::sse_decode(
                deserializer,
            ));
        }
        return ans_;
    }
}

impl SseDecode for pulse_protocol::snapshot::LoungeEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_loungeId = <pulse_protocol::ids::LoungeId>::sse_decode(deserializer);
        let mut var_scope = <pulse_protocol::lounge::LoungeScope>::sse_decode(deserializer);
        let mut var_roomId = <Option<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_roomSlug = <Option<String>>::sse_decode(deserializer);
        let mut var_participants =
            <Vec<pulse_protocol::snapshot::LoungeParticipantView>>::sse_decode(deserializer);
        let mut var_joined = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::LoungeEntry {
            lounge_id: var_loungeId,
            scope: var_scope,
            room_id: var_roomId,
            room_slug: var_roomSlug,
            participants: var_participants,
            joined: var_joined,
        };
    }
}

impl SseDecode for pulse_protocol::ids::LoungeId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::LoungeId(var_field0);
    }
}

impl SseDecode for pulse_protocol::snapshot::LoungeMediaState {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::LoungeMediaState::Idle,
            1 => pulse_protocol::snapshot::LoungeMediaState::Connecting,
            2 => pulse_protocol::snapshot::LoungeMediaState::Connected,
            3 => pulse_protocol::snapshot::LoungeMediaState::Failed,
            _ => unreachable!("Invalid variant for LoungeMediaState: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::LoungeParticipantView {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_accountId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_fromSelf = <bool>::sse_decode(deserializer);
        let mut var_canSpeak = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::LoungeParticipantView {
            account_id: var_accountId,
            display_name: var_displayName,
            from_self: var_fromSelf,
            can_speak: var_canSpeak,
        };
    }
}

impl SseDecode for pulse_protocol::lounge::LoungeScope {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::lounge::LoungeScope::Campus,
            1 => pulse_protocol::lounge::LoungeScope::Room,
            _ => unreachable!("Invalid variant for LoungeScope: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::LoungeSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_lounges =
            <Vec<pulse_protocol::snapshot::LoungeEntry>>::sse_decode(deserializer);
        let mut var_joinedLoungeId =
            <Option<pulse_protocol::ids::LoungeId>>::sse_decode(deserializer);
        let mut var_joining = <bool>::sse_decode(deserializer);
        let mut var_ticket =
            <Option<pulse_protocol::lounge::LoungeTicket>>::sse_decode(deserializer);
        let mut var_media = <pulse_protocol::snapshot::LoungeMediaState>::sse_decode(deserializer);
        let mut var_canJoin = <bool>::sse_decode(deserializer);
        let mut var_canSpeak = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::LoungeSnapshot {
            lounges: var_lounges,
            joined_lounge_id: var_joinedLoungeId,
            joining: var_joining,
            ticket: var_ticket,
            media: var_media,
            can_join: var_canJoin,
            can_speak: var_canSpeak,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::lounge::LoungeTicket {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_loungeId = <pulse_protocol::ids::LoungeId>::sse_decode(deserializer);
        let mut var_livekitUrl = <String>::sse_decode(deserializer);
        let mut var_roomName = <String>::sse_decode(deserializer);
        let mut var_token = <String>::sse_decode(deserializer);
        let mut var_expiresAtMs = <u64>::sse_decode(deserializer);
        return pulse_protocol::lounge::LoungeTicket {
            lounge_id: var_loungeId,
            livekit_url: var_livekitUrl,
            room_name: var_roomName,
            token: var_token,
            expires_at_ms: var_expiresAtMs,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::MemberEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_accountId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_email = <String>::sse_decode(deserializer);
        let mut var_role = <pulse_protocol::rbac::Role>::sse_decode(deserializer);
        let mut var_manageable = <bool>::sse_decode(deserializer);
        let mut var_isSelf = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::MemberEntry {
            account_id: var_accountId,
            display_name: var_displayName,
            email: var_email,
            role: var_role,
            manageable: var_manageable,
            is_self: var_isSelf,
        };
    }
}

impl SseDecode for pulse_protocol::messaging::MentionRef {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_accountId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        return pulse_protocol::messaging::MentionRef {
            account_id: var_accountId,
            display_name: var_displayName,
        };
    }
}

impl SseDecode for pulse_protocol::ids::MessageId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::MessageId(var_field0);
    }
}

impl SseDecode for pulse_protocol::messaging::MessageRejection {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::messaging::MessageRejection::Forbidden,
            1 => pulse_protocol::messaging::MessageRejection::InvalidBody,
            2 => pulse_protocol::messaging::MessageRejection::UnknownRoom,
            3 => pulse_protocol::messaging::MessageRejection::UnknownThread,
            4 => pulse_protocol::messaging::MessageRejection::InvalidAttachment,
            _ => unreachable!("Invalid variant for MessageRejection: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::note::Note {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_campusId = <pulse_protocol::ids::CampusId>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_members = <Vec<pulse_protocol::note::NoteMember>>::sse_decode(deserializer);
        return pulse_protocol::note::Note {
            campus_id: var_campusId,
            room_id: var_roomId,
            members: var_members,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::NoteEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_title = <String>::sse_decode(deserializer);
        let mut var_memberCount = <u32>::sse_decode(deserializer);
        let mut var_group = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::NoteEntry {
            room_id: var_roomId,
            title: var_title,
            member_count: var_memberCount,
            group: var_group,
        };
    }
}

impl SseDecode for pulse_protocol::note::NoteMember {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_accountId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        return pulse_protocol::note::NoteMember {
            account_id: var_accountId,
            display_name: var_displayName,
        };
    }
}

impl SseDecode for Option<String> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<String>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::AccountId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::AccountId>::sse_decode(deserializer));
        } else {
            return None;
        }
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

impl SseDecode for Option<pulse_protocol::snapshot::AttachmentView> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::AttachmentView>::sse_decode(
                deserializer,
            ));
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

impl SseDecode for Option<pulse_protocol::campus::CampusInvite> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::campus::CampusInvite>::sse_decode(
                deserializer,
            ));
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

impl SseDecode for Option<pulse_protocol::snapshot::IssuedStewardToken> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::IssuedStewardToken>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::LoungeId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::LoungeId>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::lounge::LoungeTicket> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::lounge::LoungeTicket>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::MessageId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::MessageId>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::messaging::MessageRejection> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::messaging::MessageRejection>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::shelf::ReminderPreset> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::shelf::ReminderPreset>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::snapshot::ReplyPreview> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::ReplyPreview>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::RoomId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::RoomId>::sse_decode(deserializer));
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

impl SseDecode for Option<pulse_protocol::snapshot::ShelfEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::ShelfEntry>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::ids::ShelfItemId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::ShelfItemId>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::steward::StewardTool> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::steward::StewardTool>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::snapshot::StreamState> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::StreamState>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::thread::SubscriptionState> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::thread::SubscriptionState>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::snapshot::ThreadPreview> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::ThreadPreview>::sse_decode(
                deserializer,
            ));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<pulse_protocol::snapshot::TimelineEntry> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::snapshot::TimelineEntry>::sse_decode(
                deserializer,
            ));
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

impl SseDecode for Option<u64> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<u64>::sse_decode(deserializer));
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
            5 => pulse_protocol::rbac::Permission::PostMessage,
            6 => pulse_protocol::rbac::Permission::React,
            7 => pulse_protocol::rbac::Permission::PinMessage,
            8 => pulse_protocol::rbac::Permission::UploadAttachment,
            9 => pulse_protocol::rbac::Permission::ManageStewards,
            10 => pulse_protocol::rbac::Permission::StreamReply,
            11 => pulse_protocol::rbac::Permission::UseStewardTools,
            12 => pulse_protocol::rbac::Permission::JoinLounge,
            13 => pulse_protocol::rbac::Permission::SpeakInLounge,
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

impl SseDecode for pulse_protocol::reaction::React {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_emoji = <String>::sse_decode(deserializer);
        let mut var_action = <pulse_protocol::reaction::ReactionAction>::sse_decode(deserializer);
        return pulse_protocol::reaction::React {
            room_id: var_roomId,
            message_id: var_messageId,
            emoji: var_emoji,
            action: var_action,
        };
    }
}

impl SseDecode for pulse_protocol::reaction::ReactionAction {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::reaction::ReactionAction::Add,
            1 => pulse_protocol::reaction::ReactionAction::Remove,
            _ => unreachable!("Invalid variant for ReactionAction: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ReactionSummary {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_emoji = <String>::sse_decode(deserializer);
        let mut var_count = <u32>::sse_decode(deserializer);
        let mut var_reactedBySelf = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ReactionSummary {
            emoji: var_emoji,
            count: var_count,
            reacted_by_self: var_reactedBySelf,
        };
    }
}

impl SseDecode for (String, Vec<pulse_protocol::ids::AttachmentId>) {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <String>::sse_decode(deserializer);
        let mut var_field1 = <Vec<pulse_protocol::ids::AttachmentId>>::sse_decode(deserializer);
        return (var_field0, var_field1);
    }
}

impl SseDecode for pulse_protocol::shelf::ReminderPreset {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::shelf::ReminderPreset::InTwentyMinutes,
            1 => pulse_protocol::shelf::ReminderPreset::InOneHour,
            2 => pulse_protocol::shelf::ReminderPreset::InThreeHours,
            3 => pulse_protocol::shelf::ReminderPreset::Tomorrow,
            4 => pulse_protocol::shelf::ReminderPreset::NextWeek,
            _ => unreachable!("Invalid variant for ReminderPreset: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ReplyPreview {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_authorId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_authorDisplayName = <String>::sse_decode(deserializer);
        let mut var_preview = <String>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ReplyPreview {
            message_id: var_messageId,
            author_id: var_authorId,
            author_display_name: var_authorDisplayName,
            preview: var_preview,
        };
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
            4 => pulse_protocol::rbac::Role::Steward,
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

impl SseDecode for pulse_protocol::snapshot::SearchHit {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_threadRootId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_authorDisplayName = <String>::sse_decode(deserializer);
        let mut var_topicTitle = <Option<String>>::sse_decode(deserializer);
        let mut var_snippet =
            <Vec<pulse_protocol::snapshot::HighlightSpan>>::sse_decode(deserializer);
        let mut var_score = <u32>::sse_decode(deserializer);
        return pulse_protocol::snapshot::SearchHit {
            room_id: var_roomId,
            message_id: var_messageId,
            thread_root_id: var_threadRootId,
            author_display_name: var_authorDisplayName,
            topic_title: var_topicTitle,
            snippet: var_snippet,
            score: var_score,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::SearchSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_open = <bool>::sse_decode(deserializer);
        let mut var_query = <String>::sse_decode(deserializer);
        let mut var_hits = <Vec<pulse_protocol::snapshot::SearchHit>>::sse_decode(deserializer);
        let mut var_selectedIndex = <Option<u32>>::sse_decode(deserializer);
        let mut var_searching = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::SearchSnapshot {
            open: var_open,
            query: var_query,
            hits: var_hits,
            selected_index: var_selectedIndex,
            searching: var_searching,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::mention::SegmentKind {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::mention::SegmentKind::Text,
            1 => pulse_protocol::mention::SegmentKind::Mention,
            2 => pulse_protocol::mention::SegmentKind::Link,
            _ => unreachable!("Invalid variant for SegmentKind: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::messaging::SendMessage {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_clientNonce = <pulse_protocol::ids::ClientNonce>::sse_decode(deserializer);
        let mut var_body = <String>::sse_decode(deserializer);
        let mut var_threadRootId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_attachmentIds =
            <Vec<pulse_protocol::ids::AttachmentId>>::sse_decode(deserializer);
        return pulse_protocol::messaging::SendMessage {
            room_id: var_roomId,
            client_nonce: var_clientNonce,
            body: var_body,
            thread_root_id: var_threadRootId,
            attachment_ids: var_attachmentIds,
        };
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
        let mut var_activeRoomId = <Option<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_invite =
            <Option<pulse_protocol::campus::CampusInvite>>::sse_decode(deserializer);
        let mut var_canInvite = <bool>::sse_decode(deserializer);
        let mut var_members =
            <Vec<pulse_protocol::snapshot::MemberEntry>>::sse_decode(deserializer);
        let mut var_assignableRoles = <Vec<pulse_protocol::rbac::Role>>::sse_decode(deserializer);
        let mut var_quickAccounts =
            <Vec<pulse_protocol::dev_seed::DevAccount>>::sse_decode(deserializer);
        let mut var_notes = <Vec<pulse_protocol::snapshot::NoteEntry>>::sse_decode(deserializer);
        let mut var_noteCandidates =
            <Vec<pulse_protocol::snapshot::MemberEntry>>::sse_decode(deserializer);
        let mut var_busy = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::SessionSnapshot {
            auth: var_auth,
            account: var_account,
            campuses: var_campuses,
            active_campus_id: var_activeCampusId,
            rooms: var_rooms,
            active_room_id: var_activeRoomId,
            invite: var_invite,
            can_invite: var_canInvite,
            members: var_members,
            assignable_roles: var_assignableRoles,
            quick_accounts: var_quickAccounts,
            notes: var_notes,
            note_candidates: var_noteCandidates,
            busy: var_busy,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::thread::SetTopicTitle {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_rootMessageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_topicTitle = <String>::sse_decode(deserializer);
        return pulse_protocol::thread::SetTopicTitle {
            room_id: var_roomId,
            root_message_id: var_rootMessageId,
            topic_title: var_topicTitle,
        };
    }
}

impl SseDecode for pulse_protocol::shelf::ShelfAction {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::shelf::ShelfAction::Save,
            1 => pulse_protocol::shelf::ShelfAction::Reschedule,
            2 => pulse_protocol::shelf::ShelfAction::Complete,
            3 => pulse_protocol::shelf::ShelfAction::Reopen,
            4 => pulse_protocol::shelf::ShelfAction::Remove,
            _ => unreachable!("Invalid variant for ShelfAction: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::shelf::ShelfBucket {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::shelf::ShelfBucket::Due,
            1 => pulse_protocol::shelf::ShelfBucket::Upcoming,
            2 => pulse_protocol::shelf::ShelfBucket::Done,
            _ => unreachable!("Invalid variant for ShelfBucket: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ShelfEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_shelfItemId =
            <Option<pulse_protocol::ids::ShelfItemId>>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_authorDisplayName = <String>::sse_decode(deserializer);
        let mut var_preview = <String>::sse_decode(deserializer);
        let mut var_remindAtMs = <Option<u64>>::sse_decode(deserializer);
        let mut var_doneAtMs = <Option<u64>>::sse_decode(deserializer);
        let mut var_bucket = <pulse_protocol::shelf::ShelfBucket>::sse_decode(deserializer);
        let mut var_pending = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ShelfEntry {
            shelf_item_id: var_shelfItemId,
            room_id: var_roomId,
            message_id: var_messageId,
            author_display_name: var_authorDisplayName,
            preview: var_preview,
            remind_at_ms: var_remindAtMs,
            done_at_ms: var_doneAtMs,
            bucket: var_bucket,
            pending: var_pending,
        };
    }
}

impl SseDecode for pulse_protocol::ids::ShelfItemId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::ShelfItemId(var_field0);
    }
}

impl SseDecode for pulse_protocol::snapshot::ShelfSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_open = <bool>::sse_decode(deserializer);
        let mut var_due = <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_decode(deserializer);
        let mut var_upcoming =
            <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_decode(deserializer);
        let mut var_done = <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_decode(deserializer);
        let mut var_dueCount = <u32>::sse_decode(deserializer);
        let mut var_latestReminder =
            <Option<pulse_protocol::snapshot::ShelfEntry>>::sse_decode(deserializer);
        let mut var_presets =
            <Vec<pulse_protocol::shelf::ReminderPreset>>::sse_decode(deserializer);
        let mut var_pickerAnchorMs = <Option<u64>>::sse_decode(deserializer);
        let mut var_pickerLatestMs = <Option<u64>>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ShelfSnapshot {
            open: var_open,
            due: var_due,
            upcoming: var_upcoming,
            done: var_done,
            due_count: var_dueCount,
            latest_reminder: var_latestReminder,
            presets: var_presets,
            picker_anchor_ms: var_pickerAnchorMs,
            picker_latest_ms: var_pickerLatestMs,
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

impl SseDecode for pulse_protocol::snapshot::StewardEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_stewardId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_displayName = <String>::sse_decode(deserializer);
        let mut var_mentionToken = <String>::sse_decode(deserializer);
        let mut var_roomIds = <Vec<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_revoked = <bool>::sse_decode(deserializer);
        let mut var_manageable = <bool>::sse_decode(deserializer);
        return pulse_protocol::snapshot::StewardEntry {
            steward_id: var_stewardId,
            display_name: var_displayName,
            mention_token: var_mentionToken,
            room_ids: var_roomIds,
            revoked: var_revoked,
            manageable: var_manageable,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::StewardSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_open = <bool>::sse_decode(deserializer);
        let mut var_stewards =
            <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_decode(deserializer);
        let mut var_canManage = <bool>::sse_decode(deserializer);
        let mut var_issued =
            <Option<pulse_protocol::snapshot::IssuedStewardToken>>::sse_decode(deserializer);
        let mut var_pending = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::StewardSnapshot {
            open: var_open,
            stewards: var_stewards,
            can_manage: var_canManage,
            issued: var_issued,
            pending: var_pending,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::steward::StewardTool {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::steward::StewardTool::ListRooms,
            1 => pulse_protocol::steward::StewardTool::ReadRoomHistory,
            2 => pulse_protocol::steward::StewardTool::ReadThread,
            3 => pulse_protocol::steward::StewardTool::PostMessage,
            _ => unreachable!("Invalid variant for StewardTool: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::steward::StreamChunk {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_streamId = <pulse_protocol::ids::StreamId>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_threadRootId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_phase = <pulse_protocol::steward::StreamPhase>::sse_decode(deserializer);
        let mut var_sequence = <u32>::sse_decode(deserializer);
        let mut var_text = <String>::sse_decode(deserializer);
        return pulse_protocol::steward::StreamChunk {
            stream_id: var_streamId,
            room_id: var_roomId,
            thread_root_id: var_threadRootId,
            phase: var_phase,
            sequence: var_sequence,
            text: var_text,
        };
    }
}

impl SseDecode for pulse_protocol::ids::StreamId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::StreamId(var_field0);
    }
}

impl SseDecode for pulse_protocol::steward::StreamPhase {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::steward::StreamPhase::Begin,
            1 => pulse_protocol::steward::StreamPhase::Delta,
            2 => pulse_protocol::steward::StreamPhase::End,
            3 => pulse_protocol::steward::StreamPhase::Abort,
            _ => unreachable!("Invalid variant for StreamPhase: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::StreamState {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::StreamState::Streaming,
            1 => pulse_protocol::snapshot::StreamState::Finalizing,
            _ => unreachable!("Invalid variant for StreamState: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::thread::SubscriptionState {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::thread::SubscriptionState::Following,
            1 => pulse_protocol::thread::SubscriptionState::Muted,
            2 => pulse_protocol::thread::SubscriptionState::Archived,
            _ => unreachable!("Invalid variant for SubscriptionState: {}", inner),
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ThreadPreview {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_topicTitle = <Option<String>>::sse_decode(deserializer);
        let mut var_replyCount = <u32>::sse_decode(deserializer);
        let mut var_latestReply =
            <Option<pulse_protocol::snapshot::ReplyPreview>>::sse_decode(deserializer);
        let mut var_showLatestReply = <bool>::sse_decode(deserializer);
        let mut var_subscription =
            <Option<pulse_protocol::thread::SubscriptionState>>::sse_decode(deserializer);
        let mut var_unreadReplies = <u32>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ThreadPreview {
            topic_title: var_topicTitle,
            reply_count: var_replyCount,
            latest_reply: var_latestReply,
            show_latest_reply: var_showLatestReply,
            subscription: var_subscription,
            unread_replies: var_unreadReplies,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::ThreadSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <Option<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_rootMessageId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_root =
            <Option<pulse_protocol::snapshot::TimelineEntry>>::sse_decode(deserializer);
        let mut var_replies =
            <Vec<pulse_protocol::snapshot::TimelineEntry>>::sse_decode(deserializer);
        let mut var_topicTitle = <Option<String>>::sse_decode(deserializer);
        let mut var_subscription =
            <Option<pulse_protocol::thread::SubscriptionState>>::sse_decode(deserializer);
        let mut var_syncing = <bool>::sse_decode(deserializer);
        let mut var_mentionableStewards =
            <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::ThreadSnapshot {
            room_id: var_roomId,
            root_message_id: var_rootMessageId,
            root: var_root,
            replies: var_replies,
            topic_title: var_topicTitle,
            subscription: var_subscription,
            syncing: var_syncing,
            mentionable_stewards: var_mentionableStewards,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::TimelineEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_clientNonce = <pulse_protocol::ids::ClientNonce>::sse_decode(deserializer);
        let mut var_messageId = <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_authorId = <pulse_protocol::ids::AccountId>::sse_decode(deserializer);
        let mut var_authorDisplayName = <String>::sse_decode(deserializer);
        let mut var_authorMentionToken = <String>::sse_decode(deserializer);
        let mut var_fromSelf = <bool>::sse_decode(deserializer);
        let mut var_body = <String>::sse_decode(deserializer);
        let mut var_segments =
            <Vec<pulse_protocol::mention::BodySegment>>::sse_decode(deserializer);
        let mut var_threadRootId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_reactions =
            <Vec<pulse_protocol::snapshot::ReactionSummary>>::sse_decode(deserializer);
        let mut var_thread =
            <Option<pulse_protocol::snapshot::ThreadPreview>>::sse_decode(deserializer);
        let mut var_delivery = <pulse_protocol::snapshot::DeliveryState>::sse_decode(deserializer);
        let mut var_rejection =
            <Option<pulse_protocol::messaging::MessageRejection>>::sse_decode(deserializer);
        let mut var_attachments =
            <Vec<pulse_protocol::snapshot::AttachmentView>>::sse_decode(deserializer);
        let mut var_pendingDrafts =
            <Vec<pulse_protocol::snapshot::DraftAttachmentView>>::sse_decode(deserializer);
        let mut var_pinned = <bool>::sse_decode(deserializer);
        let mut var_shelved = <bool>::sse_decode(deserializer);
        let mut var_authorIsSteward = <bool>::sse_decode(deserializer);
        let mut var_stream =
            <Option<pulse_protocol::snapshot::StreamState>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::TimelineEntry {
            client_nonce: var_clientNonce,
            message_id: var_messageId,
            author_id: var_authorId,
            author_display_name: var_authorDisplayName,
            author_mention_token: var_authorMentionToken,
            from_self: var_fromSelf,
            body: var_body,
            segments: var_segments,
            thread_root_id: var_threadRootId,
            reactions: var_reactions,
            thread: var_thread,
            delivery: var_delivery,
            rejection: var_rejection,
            attachments: var_attachments,
            pending_drafts: var_pendingDrafts,
            pinned: var_pinned,
            shelved: var_shelved,
            author_is_steward: var_authorIsSteward,
            stream: var_stream,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::TimelineSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <Option<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_entries =
            <Vec<pulse_protocol::snapshot::TimelineEntry>>::sse_decode(deserializer);
        let mut var_syncing = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        let mut var_reactionPalette = <Vec<String>>::sse_decode(deserializer);
        let mut var_drafts =
            <Vec<pulse_protocol::snapshot::DraftAttachmentView>>::sse_decode(deserializer);
        let mut var_focusMessageId =
            <Option<pulse_protocol::ids::MessageId>>::sse_decode(deserializer);
        let mut var_canPin = <bool>::sse_decode(deserializer);
        let mut var_canUpload = <bool>::sse_decode(deserializer);
        let mut var_mentionableStewards =
            <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::TimelineSnapshot {
            room_id: var_roomId,
            entries: var_entries,
            syncing: var_syncing,
            error: var_error,
            reaction_palette: var_reactionPalette,
            drafts: var_drafts,
            focus_message_id: var_focusMessageId,
            can_pin: var_canPin,
            can_upload: var_canUpload,
            mentionable_stewards: var_mentionableStewards,
        };
    }
}

impl SseDecode for pulse_protocol::ids::ToolCallId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::ToolCallId(var_field0);
    }
}

impl SseDecode for pulse_protocol::steward::ToolDescriptor {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_name = <String>::sse_decode(deserializer);
        let mut var_description = <String>::sse_decode(deserializer);
        let mut var_inputSchema = <String>::sse_decode(deserializer);
        return pulse_protocol::steward::ToolDescriptor {
            name: var_name,
            description: var_description,
            input_schema: var_inputSchema,
        };
    }
}

impl SseDecode for pulse_protocol::steward::ToolErrorCode {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::steward::ToolErrorCode::Forbidden,
            1 => pulse_protocol::steward::ToolErrorCode::UnknownTool,
            2 => pulse_protocol::steward::ToolErrorCode::InvalidArguments,
            3 => pulse_protocol::steward::ToolErrorCode::UnknownRoom,
            4 => pulse_protocol::steward::ToolErrorCode::UnknownThread,
            5 => pulse_protocol::steward::ToolErrorCode::Rejected,
            _ => unreachable!("Invalid variant for ToolErrorCode: {}", inner),
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

impl SseDecode for u64 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap()
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

impl SseDecode for pulse_protocol::member::UpdateMemberRequest {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_role = <pulse_protocol::rbac::Role>::sse_decode(deserializer);
        return pulse_protocol::member::UpdateMemberRequest { role: var_role };
    }
}

impl SseDecode for pulse_protocol::shelf::UpdateShelf {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_action = <pulse_protocol::shelf::ShelfAction>::sse_decode(deserializer);
        let mut var_remindAtMs = <Option<u64>>::sse_decode(deserializer);
        return pulse_protocol::shelf::UpdateShelf {
            room_id: var_roomId,
            message_id: var_messageId,
            action: var_action,
            remind_at_ms: var_remindAtMs,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::UploadState {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::snapshot::UploadState::Uploading,
            1 => pulse_protocol::snapshot::UploadState::Uploaded,
            2 => pulse_protocol::snapshot::UploadState::Failed,
            _ => unreachable!("Invalid variant for UploadState: {}", inner),
        };
    }
}

impl SseDecode for usize {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

impl SseDecode for pulse_protocol::snapshot::VaultEntry {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_tab = <pulse_protocol::media::VaultTab>::sse_decode(deserializer);
        let mut var_roomId = <pulse_protocol::ids::RoomId>::sse_decode(deserializer);
        let mut var_messageId = <pulse_protocol::ids::MessageId>::sse_decode(deserializer);
        let mut var_authorDisplayName = <String>::sse_decode(deserializer);
        let mut var_preview = <String>::sse_decode(deserializer);
        let mut var_attachment =
            <Option<pulse_protocol::snapshot::AttachmentView>>::sse_decode(deserializer);
        let mut var_link = <Option<String>>::sse_decode(deserializer);
        let mut var_pinnedAtMs = <Option<u64>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::VaultEntry {
            tab: var_tab,
            room_id: var_roomId,
            message_id: var_messageId,
            author_display_name: var_authorDisplayName,
            preview: var_preview,
            attachment: var_attachment,
            link: var_link,
            pinned_at_ms: var_pinnedAtMs,
        };
    }
}

impl SseDecode for pulse_protocol::snapshot::VaultSnapshot {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_open = <bool>::sse_decode(deserializer);
        let mut var_roomId = <Option<pulse_protocol::ids::RoomId>>::sse_decode(deserializer);
        let mut var_tab = <pulse_protocol::media::VaultTab>::sse_decode(deserializer);
        let mut var_entries = <Vec<pulse_protocol::snapshot::VaultEntry>>::sse_decode(deserializer);
        let mut var_loading = <bool>::sse_decode(deserializer);
        let mut var_hasMore = <bool>::sse_decode(deserializer);
        let mut var_error = <Option<pulse_protocol::api::ApiError>>::sse_decode(deserializer);
        return pulse_protocol::snapshot::VaultSnapshot {
            open: var_open,
            room_id: var_roomId,
            tab: var_tab,
            entries: var_entries,
            loading: var_loading,
            has_more: var_hasMore,
            error: var_error,
        };
    }
}

impl SseDecode for pulse_protocol::media::VaultTab {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => pulse_protocol::media::VaultTab::Media,
            1 => pulse_protocol::media::VaultTab::Files,
            2 => pulse_protocol::media::VaultTab::Links,
            3 => pulse_protocol::media::VaultTab::Pinned,
            _ => unreachable!("Invalid variant for VaultTab: {}", inner),
        };
    }
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
        1 => wire__pulse_protocol__steward__StreamWriter_abort_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        2 => wire__pulse_protocol__steward__StreamWriter_begin_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        3 => wire__pulse_protocol__steward__StreamWriter_delta_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        4 => {
            wire__pulse_protocol__steward__StreamWriter_end_impl(port, ptr, rust_vec_len, data_len)
        }
        5 => {
            wire__pulse_protocol__steward__StreamWriter_new_impl(port, ptr, rust_vec_len, data_len)
        }
        6 => wire__pulse_protocol__steward__StreamWriter_next_sequence_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        8 => wire__pulse_protocol__ids__account_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        9 => wire__pulse_protocol__ids__account_id_generate_impl(port, ptr, rust_vec_len, data_len),
        10 => wire__pulse_protocol__member__add_member_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        11 => wire__pulse_protocol__api__api_error_code_from_http_status_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        12 => wire__pulse_protocol__api__api_error_code_http_status_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        13 => wire__pulse_protocol__api__api_error_conflict_impl(port, ptr, rust_vec_len, data_len),
        14 => wire__pulse_protocol__api__api_error_invalid_impl(port, ptr, rust_vec_len, data_len),
        15 => wire__pulse_protocol__api__api_error_new_impl(port, ptr, rust_vec_len, data_len),
        16 => wire__crate__api__client__archive_inbox_selection_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        17 => {
            wire__pulse_protocol__member__assignable_roles_impl(port, ptr, rust_vec_len, data_len)
        }
        18 => wire__crate__api__client__attach_file_impl(port, ptr, rust_vec_len, data_len),
        19 => {
            wire__pulse_protocol__ids__attachment_id_as_uuid_impl(port, ptr, rust_vec_len, data_len)
        }
        20 => wire__pulse_protocol__ids__attachment_id_generate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        21 => wire__pulse_protocol__media__attachment_kind_as_str_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        22 => wire__pulse_protocol__media__attachment_kind_from_content_type_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        23 => wire__pulse_protocol__mention__body_segments_impl(port, ptr, rust_vec_len, data_len),
        24 => wire__pulse_protocol__api__api_path__campus_downloads_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        25 => wire__pulse_protocol__ids__campus_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        26 => wire__pulse_protocol__ids__campus_id_generate_impl(port, ptr, rust_vec_len, data_len),
        27 => wire__pulse_protocol__api__api_path__campus_invites_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        28 => wire__pulse_protocol__api__api_path__campus_member_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        29 => wire__pulse_protocol__api__api_path__campus_members_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        30 => wire__pulse_protocol__api__api_path__campus_notes_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        31 => wire__pulse_protocol__api__api_path__campus_room_uploads_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        32 => wire__pulse_protocol__api__api_path__campus_rooms_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        33 => wire__crate__api__client__cancel_inbox_reply_impl(port, ptr, rust_vec_len, data_len),
        34 => {
            wire__pulse_protocol__ids__client_nonce_as_uuid_impl(port, ptr, rust_vec_len, data_len)
        }
        35 => {
            wire__pulse_protocol__ids__client_nonce_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        36 => wire__crate__api__client__client_snapshot_impl(port, ptr, rust_vec_len, data_len),
        37 => wire__crate__api__client__close_thread_impl(port, ptr, rust_vec_len, data_len),
        38 => wire__crate__api__client__close_vault_impl(port, ptr, rust_vec_len, data_len),
        39 => wire__crate__api__client__create_campus_impl(port, ptr, rust_vec_len, data_len),
        40 => wire__pulse_protocol__campus__create_campus_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        41 => wire__pulse_protocol__campus__create_room_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        42 => wire__pulse_protocol__media__create_upload_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        43 => wire__pulse_protocol__dev_seed__dev_accounts_impl(port, ptr, rust_vec_len, data_len),
        44 => wire__crate__api__client__discard_draft_impl(port, ptr, rust_vec_len, data_len),
        45 => wire__crate__api__client__discard_message_impl(port, ptr, rust_vec_len, data_len),
        46 => wire__crate__api__client__dismiss_reminder_impl(port, ptr, rust_vec_len, data_len),
        47 => wire__pulse_protocol__ids__draft_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        48 => wire__pulse_protocol__ids__draft_id_generate_impl(port, ptr, rust_vec_len, data_len),
        49 => wire__pulse_protocol__link__extract_links_impl(port, ptr, rust_vec_len, data_len),
        50 => {
            wire__pulse_protocol__mention__extract_mentions_impl(port, ptr, rust_vec_len, data_len)
        }
        51 => {
            wire__pulse_protocol__media__guess_content_type_impl(port, ptr, rust_vec_len, data_len)
        }
        52 => wire__pulse_protocol__snapshot__inbox_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        53 => wire__crate__api__client__init_storage_impl(port, ptr, rust_vec_len, data_len),
        54 => {
            wire__pulse_protocol__campus__invite_code_render_impl(port, ptr, rust_vec_len, data_len)
        }
        55 => wire__pulse_protocol__campus__join_campus_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        56 => wire__crate__api__client__load_more_vault_impl(port, ptr, rust_vec_len, data_len),
        57 => wire__pulse_protocol__ids__lounge_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        58 => wire__pulse_protocol__ids__lounge_id_generate_impl(port, ptr, rust_vec_len, data_len),
        59 => wire__pulse_protocol__snapshot__lounge_media_state_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        60 => wire__pulse_protocol__snapshot__lounge_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        61 => {
            wire__pulse_protocol__member__member_manageable_impl(port, ptr, rust_vec_len, data_len)
        }
        62 => wire__pulse_protocol__mention__mention_token_impl(port, ptr, rust_vec_len, data_len),
        63 => wire__pulse_protocol__ids__message_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        64 => {
            wire__pulse_protocol__ids__message_id_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        65 => {
            wire__crate__api__client__move_inbox_selection_impl(port, ptr, rust_vec_len, data_len)
        }
        66 => {
            wire__crate__api__client__move_search_selection_impl(port, ptr, rust_vec_len, data_len)
        }
        67 => wire__pulse_protocol__validation__normalize_attachment_ids_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        68 => wire__pulse_protocol__validation__normalize_content_type_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        69 => wire__pulse_protocol__validation__normalize_display_name_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        70 => wire__pulse_protocol__validation__normalize_email_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        71 => wire__pulse_protocol__validation__normalize_file_name_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        72 => wire__pulse_protocol__validation__normalize_invite_code_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        73 => wire__pulse_protocol__validation__normalize_message_body_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        74 => wire__pulse_protocol__validation__normalize_message_content_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        75 => wire__pulse_protocol__validation__normalize_note_members_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        76 => wire__pulse_protocol__validation__normalize_reaction_emoji_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        77 => wire__pulse_protocol__validation__normalize_room_ids_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        78 => wire__pulse_protocol__validation__normalize_search_query_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        79 => wire__pulse_protocol__validation__normalize_topic_title_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        80 => wire__pulse_protocol__note__note_member_key_impl(port, ptr, rust_vec_len, data_len),
        81 => wire__pulse_protocol__note__note_title_impl(port, ptr, rust_vec_len, data_len),
        82 => wire__crate__api__client__open_search_hit_impl(port, ptr, rust_vec_len, data_len),
        83 => {
            wire__crate__api__client__open_search_selection_impl(port, ptr, rust_vec_len, data_len)
        }
        84 => wire__crate__api__client__open_thread_impl(port, ptr, rust_vec_len, data_len),
        85 => wire__crate__api__client__open_vault_impl(port, ptr, rust_vec_len, data_len),
        86 => wire__pulse_protocol__shelf__picker_anchor_ms_impl(port, ptr, rust_vec_len, data_len),
        87 => wire__pulse_protocol__shelf__picker_latest_ms_impl(port, ptr, rust_vec_len, data_len),
        88 => wire__crate__api__client__react_impl(port, ptr, rust_vec_len, data_len),
        89 => {
            wire__pulse_protocol__reaction__react_validate_impl(port, ptr, rust_vec_len, data_len)
        }
        90 => wire__crate__api__client__refresh_campuses_impl(port, ptr, rust_vec_len, data_len),
        91 => wire__pulse_protocol__shelf__reminder_preset_offset_ms_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        92 => wire__crate__api__client__retry_draft_impl(port, ptr, rust_vec_len, data_len),
        93 => wire__pulse_protocol__rbac__role_as_str_impl(port, ptr, rust_vec_len, data_len),
        94 => wire__pulse_protocol__member__role_assignable_impl(port, ptr, rust_vec_len, data_len),
        95 => wire__pulse_protocol__rbac__role_is_steward_impl(port, ptr, rust_vec_len, data_len),
        96 => wire__pulse_protocol__rbac__role_permits_impl(port, ptr, rust_vec_len, data_len),
        97 => wire__pulse_protocol__ids__room_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        98 => wire__pulse_protocol__ids__room_id_generate_impl(port, ptr, rust_vec_len, data_len),
        99 => wire__crate__api__client__search_impl(port, ptr, rust_vec_len, data_len),
        100 => wire__pulse_protocol__snapshot__search_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        101 => wire__pulse_protocol__search__search_terms_impl(port, ptr, rust_vec_len, data_len),
        102 => wire__crate__api__client__select_campus_impl(port, ptr, rust_vec_len, data_len),
        103 => wire__crate__api__client__select_room_impl(port, ptr, rust_vec_len, data_len),
        104 => wire__crate__api__client__send_message_impl(port, ptr, rust_vec_len, data_len),
        105 => wire__pulse_protocol__messaging__send_message_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        106 => wire__crate__api__client__send_reply_impl(port, ptr, rust_vec_len, data_len),
        107 => {
            wire__pulse_protocol__ids__session_id_as_uuid_impl(port, ptr, rust_vec_len, data_len)
        }
        108 => {
            wire__pulse_protocol__ids__session_id_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        109 => wire__crate__api__client__set_pin_impl(port, ptr, rust_vec_len, data_len),
        110 => wire__crate__api__client__set_thread_subscription_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        111 => wire__crate__api__client__set_topic_title_impl(port, ptr, rust_vec_len, data_len),
        112 => wire__pulse_protocol__thread__set_topic_title_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        113 => {
            wire__pulse_protocol__ids__shelf_item_id_as_uuid_impl(port, ptr, rust_vec_len, data_len)
        }
        114 => wire__pulse_protocol__ids__shelf_item_id_generate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        115 => wire__pulse_protocol__snapshot__shelf_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        116 => wire__crate__api__client__sign_in_impl(port, ptr, rust_vec_len, data_len),
        117 => wire__pulse_protocol__auth__sign_in_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        118 => wire__crate__api__client__sign_out_impl(port, ptr, rust_vec_len, data_len),
        119 => wire__crate__api__client__sign_up_impl(port, ptr, rust_vec_len, data_len),
        120 => wire__pulse_protocol__auth__sign_up_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        121 => wire__crate__api__client__start_inbox_reply_impl(port, ptr, rust_vec_len, data_len),
        122 => wire__pulse_protocol__snapshot__steward_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        123 => wire__pulse_protocol__steward__steward_tool_description_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        124 => wire__pulse_protocol__steward__steward_tool_from_name_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        125 => wire__pulse_protocol__steward__steward_tool_input_schema_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        126 => {
            wire__pulse_protocol__steward__steward_tool_name_impl(port, ptr, rust_vec_len, data_len)
        }
        127 => wire__pulse_protocol__ids__stream_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        128 => wire__pulse_protocol__ids__stream_id_client_nonce_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        129 => {
            wire__pulse_protocol__ids__stream_id_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        130 => wire__pulse_protocol__thread__subscription_state_as_str_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        131 => wire__pulse_protocol__snapshot__thread_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        132 => wire__crate__api__client__toggle_inbox_impl(port, ptr, rust_vec_len, data_len),
        133 => wire__crate__api__client__toggle_search_impl(port, ptr, rust_vec_len, data_len),
        134 => wire__crate__api__client__toggle_shelf_impl(port, ptr, rust_vec_len, data_len),
        135 => {
            wire__pulse_protocol__ids__tool_call_id_as_uuid_impl(port, ptr, rust_vec_len, data_len)
        }
        136 => {
            wire__pulse_protocol__ids__tool_call_id_generate_impl(port, ptr, rust_vec_len, data_len)
        }
        137 => {
            wire__pulse_protocol__steward__tool_descriptors_impl(port, ptr, rust_vec_len, data_len)
        }
        138 => wire__pulse_protocol__steward__tool_error_code_as_str_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        139 => wire__pulse_protocol__member__update_member_request_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        140 => wire__crate__api__client__update_shelf_impl(port, ptr, rust_vec_len, data_len),
        141 => wire__pulse_protocol__shelf__update_shelf_validate_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        142 => wire__pulse_protocol__validation__validate_assignable_role_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        143 => wire__pulse_protocol__validation__validate_byte_size_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        144 => wire__pulse_protocol__validation__validate_display_name_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        145 => wire__pulse_protocol__validation__validate_password_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        146 => wire__pulse_protocol__validation__validate_remind_at_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        147 => {
            wire__pulse_protocol__validation__validate_slug_impl(port, ptr, rust_vec_len, data_len)
        }
        148 => wire__pulse_protocol__snapshot__vault_snapshot_default_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        149 => {
            wire__pulse_protocol__media__vault_tab_default_impl(port, ptr, rust_vec_len, data_len)
        }
        150 => wire__crate__api__client__watch_snapshots_impl(port, ptr, rust_vec_len, data_len),
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

impl flutter_rust_bridge::IntoDart for FrbWrapper<StreamWriter> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self.0)
            .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for FrbWrapper<StreamWriter> {}

impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<StreamWriter>> for StreamWriter {
    fn into_into_dart(self) -> FrbWrapper<StreamWriter> {
        self.into()
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::member::AddMemberRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.email.into_into_dart().into_dart(),
            self.0.role.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::member::AddMemberRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::member::AddMemberRequest>>
    for pulse_protocol::member::AddMemberRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::member::AddMemberRequest> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::AttachmentId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::AttachmentId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::AttachmentId>>
    for pulse_protocol::ids::AttachmentId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::AttachmentId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::media::AttachmentKind> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::media::AttachmentKind::Image => 0.into_dart(),
            pulse_protocol::media::AttachmentKind::Video => 1.into_dart(),
            pulse_protocol::media::AttachmentKind::Audio => 2.into_dart(),
            pulse_protocol::media::AttachmentKind::File => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::media::AttachmentKind>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::media::AttachmentKind>>
    for pulse_protocol::media::AttachmentKind
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::media::AttachmentKind> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::AttachmentView> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.attachment_id.into_into_dart().into_dart(),
            self.0.file_name.into_into_dart().into_dart(),
            self.0.content_type.into_into_dart().into_dart(),
            self.0.kind.into_into_dart().into_dart(),
            self.0.byte_size.into_into_dart().into_dart(),
            self.0.url.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::AttachmentView>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::AttachmentView>>
    for pulse_protocol::snapshot::AttachmentView
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::AttachmentView> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::auth::AuthToken> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::auth::AuthToken>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::auth::AuthToken>>
    for pulse_protocol::auth::AuthToken
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::auth::AuthToken> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::mention::BodySegment> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.kind.into_into_dart().into_dart(),
            self.0.text.into_into_dart().into_dart(),
            self.0.account_id.into_into_dart().into_dart(),
            self.0.url.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::mention::BodySegment>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::mention::BodySegment>>
    for pulse_protocol::mention::BodySegment
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::mention::BodySegment> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::CampusInvite> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.code.into_into_dart().into_dart(),
            self.0.expires_at_ms.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::CampusInvite>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::CampusInvite>>
    for pulse_protocol::campus::CampusInvite
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::CampusInvite> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::ClientNonce> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::ClientNonce>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::ClientNonce>>
    for pulse_protocol::ids::ClientNonce
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::ClientNonce> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ClientSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.sequence.into_into_dart().into_dart(),
            self.0.network.into_into_dart().into_dart(),
            self.0.session.into_into_dart().into_dart(),
            self.0.timeline.into_into_dart().into_dart(),
            self.0.thread.into_into_dart().into_dart(),
            self.0.inbox.into_into_dart().into_dart(),
            self.0.search.into_into_dart().into_dart(),
            self.0.vault.into_into_dart().into_dart(),
            self.0.shelf.into_into_dart().into_dart(),
            self.0.lounges.into_into_dart().into_dart(),
            self.0.stewards.into_into_dart().into_dart(),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::media::CreateUploadRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.file_name.into_into_dart().into_dart(),
            self.0.content_type.into_into_dart().into_dart(),
            self.0.byte_size.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::media::CreateUploadRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::media::CreateUploadRequest>>
    for pulse_protocol::media::CreateUploadRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::media::CreateUploadRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::DeliveryState> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::DeliveryState::Pending => 0.into_dart(),
            pulse_protocol::snapshot::DeliveryState::Confirmed => 1.into_dart(),
            pulse_protocol::snapshot::DeliveryState::Failed => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::DeliveryState>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::DeliveryState>>
    for pulse_protocol::snapshot::DeliveryState
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::DeliveryState> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::dev_seed::DevAccount> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.email.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.role.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::dev_seed::DevAccount>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::dev_seed::DevAccount>>
    for pulse_protocol::dev_seed::DevAccount
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::dev_seed::DevAccount> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::DraftAttachmentView> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.draft_id.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.file_name.into_into_dart().into_dart(),
            self.0.kind.into_into_dart().into_dart(),
            self.0.byte_size.into_into_dart().into_dart(),
            self.0.upload.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::DraftAttachmentView>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::DraftAttachmentView>>
    for pulse_protocol::snapshot::DraftAttachmentView
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::DraftAttachmentView> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::DraftId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::DraftId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::DraftId>>
    for pulse_protocol::ids::DraftId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::DraftId> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::HighlightSpan> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.text.into_into_dart().into_dart(),
            self.0.matched.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::HighlightSpan>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::HighlightSpan>>
    for pulse_protocol::snapshot::HighlightSpan
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::HighlightSpan> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::InboxItem> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.kind.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.root_message_id.into_into_dart().into_dart(),
            self.0.topic_title.into_into_dart().into_dart(),
            self.0.root_author_display_name.into_into_dart().into_dart(),
            self.0.root_preview.into_into_dart().into_dart(),
            self.0.latest_reply.into_into_dart().into_dart(),
            self.0.reply_count.into_into_dart().into_dart(),
            self.0.unread_replies.into_into_dart().into_dart(),
            self.0.unread_mentions.into_into_dart().into_dart(),
            self.0.subscription.into_into_dart().into_dart(),
            self.0.activity_message_id.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::InboxItem>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::InboxItem>>
    for pulse_protocol::snapshot::InboxItem
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::InboxItem> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::InboxItemKind> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::InboxItemKind::Thread => 0.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::InboxItemKind>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::InboxItemKind>>
    for pulse_protocol::snapshot::InboxItemKind
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::InboxItemKind> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::InboxSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.open.into_into_dart().into_dart(),
            self.0.items.into_into_dart().into_dart(),
            self.0.unread_threads.into_into_dart().into_dart(),
            self.0.selected_root_message_id.into_into_dart().into_dart(),
            self.0.replying.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::InboxSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::InboxSnapshot>>
    for pulse_protocol::snapshot::InboxSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::InboxSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::InboxStep> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::InboxStep::Next => 0.into_dart(),
            pulse_protocol::snapshot::InboxStep::Previous => 1.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::InboxStep>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::InboxStep>>
    for pulse_protocol::snapshot::InboxStep
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::InboxStep> {
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
            pulse_protocol::api::InputField::MessageBody => 4.into_dart(),
            pulse_protocol::api::InputField::TopicTitle => 5.into_dart(),
            pulse_protocol::api::InputField::ReactionEmoji => 6.into_dart(),
            pulse_protocol::api::InputField::FileName => 7.into_dart(),
            pulse_protocol::api::InputField::ContentType => 8.into_dart(),
            pulse_protocol::api::InputField::ByteSize => 9.into_dart(),
            pulse_protocol::api::InputField::Attachments => 10.into_dart(),
            pulse_protocol::api::InputField::RemindAt => 11.into_dart(),
            pulse_protocol::api::InputField::Rooms => 12.into_dart(),
            pulse_protocol::api::InputField::InviteCode => 13.into_dart(),
            pulse_protocol::api::InputField::Role => 14.into_dart(),
            pulse_protocol::api::InputField::NoteMembers => 15.into_dart(),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::InviteCode> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.secret.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::InviteCode>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::InviteCode>>
    for pulse_protocol::campus::InviteCode
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::InviteCode> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::IssuedStewardToken> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.steward_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.token.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::IssuedStewardToken>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::IssuedStewardToken>>
    for pulse_protocol::snapshot::IssuedStewardToken
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::IssuedStewardToken> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::campus::JoinCampusRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.code.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::campus::JoinCampusRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::campus::JoinCampusRequest>>
    for pulse_protocol::campus::JoinCampusRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::campus::JoinCampusRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::LoungeEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.lounge_id.into_into_dart().into_dart(),
            self.0.scope.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.room_slug.into_into_dart().into_dart(),
            self.0.participants.into_into_dart().into_dart(),
            self.0.joined.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::LoungeEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::LoungeEntry>>
    for pulse_protocol::snapshot::LoungeEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::LoungeEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::LoungeId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::LoungeId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::LoungeId>>
    for pulse_protocol::ids::LoungeId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::LoungeId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::LoungeMediaState> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::LoungeMediaState::Idle => 0.into_dart(),
            pulse_protocol::snapshot::LoungeMediaState::Connecting => 1.into_dart(),
            pulse_protocol::snapshot::LoungeMediaState::Connected => 2.into_dart(),
            pulse_protocol::snapshot::LoungeMediaState::Failed => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::LoungeMediaState>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::LoungeMediaState>>
    for pulse_protocol::snapshot::LoungeMediaState
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::LoungeMediaState> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::LoungeParticipantView> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.account_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.from_self.into_into_dart().into_dart(),
            self.0.can_speak.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::LoungeParticipantView>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::LoungeParticipantView>>
    for pulse_protocol::snapshot::LoungeParticipantView
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::LoungeParticipantView> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::lounge::LoungeScope> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::lounge::LoungeScope::Campus => 0.into_dart(),
            pulse_protocol::lounge::LoungeScope::Room => 1.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::lounge::LoungeScope>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::lounge::LoungeScope>>
    for pulse_protocol::lounge::LoungeScope
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::lounge::LoungeScope> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::LoungeSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.lounges.into_into_dart().into_dart(),
            self.0.joined_lounge_id.into_into_dart().into_dart(),
            self.0.joining.into_into_dart().into_dart(),
            self.0.ticket.into_into_dart().into_dart(),
            self.0.media.into_into_dart().into_dart(),
            self.0.can_join.into_into_dart().into_dart(),
            self.0.can_speak.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::LoungeSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::LoungeSnapshot>>
    for pulse_protocol::snapshot::LoungeSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::LoungeSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::lounge::LoungeTicket> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.lounge_id.into_into_dart().into_dart(),
            self.0.livekit_url.into_into_dart().into_dart(),
            self.0.room_name.into_into_dart().into_dart(),
            self.0.token.into_into_dart().into_dart(),
            self.0.expires_at_ms.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::lounge::LoungeTicket>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::lounge::LoungeTicket>>
    for pulse_protocol::lounge::LoungeTicket
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::lounge::LoungeTicket> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::MemberEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.account_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.email.into_into_dart().into_dart(),
            self.0.role.into_into_dart().into_dart(),
            self.0.manageable.into_into_dart().into_dart(),
            self.0.is_self.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::MemberEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::MemberEntry>>
    for pulse_protocol::snapshot::MemberEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::MemberEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::messaging::MentionRef> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.account_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::messaging::MentionRef>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::messaging::MentionRef>>
    for pulse_protocol::messaging::MentionRef
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::messaging::MentionRef> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::MessageId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::MessageId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::MessageId>>
    for pulse_protocol::ids::MessageId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::MessageId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::messaging::MessageRejection> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::messaging::MessageRejection::Forbidden => 0.into_dart(),
            pulse_protocol::messaging::MessageRejection::InvalidBody => 1.into_dart(),
            pulse_protocol::messaging::MessageRejection::UnknownRoom => 2.into_dart(),
            pulse_protocol::messaging::MessageRejection::UnknownThread => 3.into_dart(),
            pulse_protocol::messaging::MessageRejection::InvalidAttachment => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::messaging::MessageRejection>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::messaging::MessageRejection>>
    for pulse_protocol::messaging::MessageRejection
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::messaging::MessageRejection> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::note::Note> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.members.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::note::Note>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::note::Note>>
    for pulse_protocol::note::Note
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::note::Note> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::NoteEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.title.into_into_dart().into_dart(),
            self.0.member_count.into_into_dart().into_dart(),
            self.0.group.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::NoteEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::NoteEntry>>
    for pulse_protocol::snapshot::NoteEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::NoteEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::note::NoteMember> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.account_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::note::NoteMember>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::note::NoteMember>>
    for pulse_protocol::note::NoteMember
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::note::NoteMember> {
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
            pulse_protocol::rbac::Permission::PostMessage => 5.into_dart(),
            pulse_protocol::rbac::Permission::React => 6.into_dart(),
            pulse_protocol::rbac::Permission::PinMessage => 7.into_dart(),
            pulse_protocol::rbac::Permission::UploadAttachment => 8.into_dart(),
            pulse_protocol::rbac::Permission::ManageStewards => 9.into_dart(),
            pulse_protocol::rbac::Permission::StreamReply => 10.into_dart(),
            pulse_protocol::rbac::Permission::UseStewardTools => 11.into_dart(),
            pulse_protocol::rbac::Permission::JoinLounge => 12.into_dart(),
            pulse_protocol::rbac::Permission::SpeakInLounge => 13.into_dart(),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::reaction::React> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.emoji.into_into_dart().into_dart(),
            self.0.action.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::reaction::React>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::reaction::React>>
    for pulse_protocol::reaction::React
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::reaction::React> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::reaction::ReactionAction> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::reaction::ReactionAction::Add => 0.into_dart(),
            pulse_protocol::reaction::ReactionAction::Remove => 1.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::reaction::ReactionAction>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::reaction::ReactionAction>>
    for pulse_protocol::reaction::ReactionAction
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::reaction::ReactionAction> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ReactionSummary> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.emoji.into_into_dart().into_dart(),
            self.0.count.into_into_dart().into_dart(),
            self.0.reacted_by_self.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ReactionSummary>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ReactionSummary>>
    for pulse_protocol::snapshot::ReactionSummary
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ReactionSummary> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::shelf::ReminderPreset> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::shelf::ReminderPreset::InTwentyMinutes => 0.into_dart(),
            pulse_protocol::shelf::ReminderPreset::InOneHour => 1.into_dart(),
            pulse_protocol::shelf::ReminderPreset::InThreeHours => 2.into_dart(),
            pulse_protocol::shelf::ReminderPreset::Tomorrow => 3.into_dart(),
            pulse_protocol::shelf::ReminderPreset::NextWeek => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::shelf::ReminderPreset>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::shelf::ReminderPreset>>
    for pulse_protocol::shelf::ReminderPreset
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::shelf::ReminderPreset> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ReplyPreview> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.message_id.into_into_dart().into_dart(),
            self.0.author_id.into_into_dart().into_dart(),
            self.0.author_display_name.into_into_dart().into_dart(),
            self.0.preview.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ReplyPreview>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ReplyPreview>>
    for pulse_protocol::snapshot::ReplyPreview
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ReplyPreview> {
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
            pulse_protocol::rbac::Role::Steward => 4.into_dart(),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::SearchHit> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.thread_root_id.into_into_dart().into_dart(),
            self.0.author_display_name.into_into_dart().into_dart(),
            self.0.topic_title.into_into_dart().into_dart(),
            self.0.snippet.into_into_dart().into_dart(),
            self.0.score.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::SearchHit>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::SearchHit>>
    for pulse_protocol::snapshot::SearchHit
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::SearchHit> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::SearchSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.open.into_into_dart().into_dart(),
            self.0.query.into_into_dart().into_dart(),
            self.0.hits.into_into_dart().into_dart(),
            self.0.selected_index.into_into_dart().into_dart(),
            self.0.searching.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::SearchSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::SearchSnapshot>>
    for pulse_protocol::snapshot::SearchSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::SearchSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::mention::SegmentKind> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::mention::SegmentKind::Text => 0.into_dart(),
            pulse_protocol::mention::SegmentKind::Mention => 1.into_dart(),
            pulse_protocol::mention::SegmentKind::Link => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::mention::SegmentKind>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::mention::SegmentKind>>
    for pulse_protocol::mention::SegmentKind
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::mention::SegmentKind> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::messaging::SendMessage> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.client_nonce.into_into_dart().into_dart(),
            self.0.body.into_into_dart().into_dart(),
            self.0.thread_root_id.into_into_dart().into_dart(),
            self.0.attachment_ids.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::messaging::SendMessage>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::messaging::SendMessage>>
    for pulse_protocol::messaging::SendMessage
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::messaging::SendMessage> {
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
            self.0.active_room_id.into_into_dart().into_dart(),
            self.0.invite.into_into_dart().into_dart(),
            self.0.can_invite.into_into_dart().into_dart(),
            self.0.members.into_into_dart().into_dart(),
            self.0.assignable_roles.into_into_dart().into_dart(),
            self.0.quick_accounts.into_into_dart().into_dart(),
            self.0.notes.into_into_dart().into_dart(),
            self.0.note_candidates.into_into_dart().into_dart(),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::thread::SetTopicTitle> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.root_message_id.into_into_dart().into_dart(),
            self.0.topic_title.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::thread::SetTopicTitle>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::thread::SetTopicTitle>>
    for pulse_protocol::thread::SetTopicTitle
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::thread::SetTopicTitle> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::shelf::ShelfAction> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::shelf::ShelfAction::Save => 0.into_dart(),
            pulse_protocol::shelf::ShelfAction::Reschedule => 1.into_dart(),
            pulse_protocol::shelf::ShelfAction::Complete => 2.into_dart(),
            pulse_protocol::shelf::ShelfAction::Reopen => 3.into_dart(),
            pulse_protocol::shelf::ShelfAction::Remove => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::shelf::ShelfAction>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::shelf::ShelfAction>>
    for pulse_protocol::shelf::ShelfAction
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::shelf::ShelfAction> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::shelf::ShelfBucket> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::shelf::ShelfBucket::Due => 0.into_dart(),
            pulse_protocol::shelf::ShelfBucket::Upcoming => 1.into_dart(),
            pulse_protocol::shelf::ShelfBucket::Done => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::shelf::ShelfBucket>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::shelf::ShelfBucket>>
    for pulse_protocol::shelf::ShelfBucket
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::shelf::ShelfBucket> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ShelfEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.shelf_item_id.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.author_display_name.into_into_dart().into_dart(),
            self.0.preview.into_into_dart().into_dart(),
            self.0.remind_at_ms.into_into_dart().into_dart(),
            self.0.done_at_ms.into_into_dart().into_dart(),
            self.0.bucket.into_into_dart().into_dart(),
            self.0.pending.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ShelfEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ShelfEntry>>
    for pulse_protocol::snapshot::ShelfEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ShelfEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::ShelfItemId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::ShelfItemId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::ShelfItemId>>
    for pulse_protocol::ids::ShelfItemId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::ShelfItemId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ShelfSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.open.into_into_dart().into_dart(),
            self.0.due.into_into_dart().into_dart(),
            self.0.upcoming.into_into_dart().into_dart(),
            self.0.done.into_into_dart().into_dart(),
            self.0.due_count.into_into_dart().into_dart(),
            self.0.latest_reminder.into_into_dart().into_dart(),
            self.0.presets.into_into_dart().into_dart(),
            self.0.picker_anchor_ms.into_into_dart().into_dart(),
            self.0.picker_latest_ms.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ShelfSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ShelfSnapshot>>
    for pulse_protocol::snapshot::ShelfSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ShelfSnapshot> {
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::StewardEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.steward_id.into_into_dart().into_dart(),
            self.0.display_name.into_into_dart().into_dart(),
            self.0.mention_token.into_into_dart().into_dart(),
            self.0.room_ids.into_into_dart().into_dart(),
            self.0.revoked.into_into_dart().into_dart(),
            self.0.manageable.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::StewardEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::StewardEntry>>
    for pulse_protocol::snapshot::StewardEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::StewardEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::StewardSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.open.into_into_dart().into_dart(),
            self.0.stewards.into_into_dart().into_dart(),
            self.0.can_manage.into_into_dart().into_dart(),
            self.0.issued.into_into_dart().into_dart(),
            self.0.pending.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::StewardSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::StewardSnapshot>>
    for pulse_protocol::snapshot::StewardSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::StewardSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::steward::StewardTool> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::steward::StewardTool::ListRooms => 0.into_dart(),
            pulse_protocol::steward::StewardTool::ReadRoomHistory => 1.into_dart(),
            pulse_protocol::steward::StewardTool::ReadThread => 2.into_dart(),
            pulse_protocol::steward::StewardTool::PostMessage => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::steward::StewardTool>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::steward::StewardTool>>
    for pulse_protocol::steward::StewardTool
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::steward::StewardTool> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::steward::StreamChunk> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.stream_id.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.thread_root_id.into_into_dart().into_dart(),
            self.0.phase.into_into_dart().into_dart(),
            self.0.sequence.into_into_dart().into_dart(),
            self.0.text.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::steward::StreamChunk>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::steward::StreamChunk>>
    for pulse_protocol::steward::StreamChunk
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::steward::StreamChunk> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::StreamId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::StreamId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::StreamId>>
    for pulse_protocol::ids::StreamId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::StreamId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::steward::StreamPhase> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::steward::StreamPhase::Begin => 0.into_dart(),
            pulse_protocol::steward::StreamPhase::Delta => 1.into_dart(),
            pulse_protocol::steward::StreamPhase::End => 2.into_dart(),
            pulse_protocol::steward::StreamPhase::Abort => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::steward::StreamPhase>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::steward::StreamPhase>>
    for pulse_protocol::steward::StreamPhase
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::steward::StreamPhase> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::StreamState> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::StreamState::Streaming => 0.into_dart(),
            pulse_protocol::snapshot::StreamState::Finalizing => 1.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::StreamState>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::StreamState>>
    for pulse_protocol::snapshot::StreamState
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::StreamState> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::thread::SubscriptionState> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::thread::SubscriptionState::Following => 0.into_dart(),
            pulse_protocol::thread::SubscriptionState::Muted => 1.into_dart(),
            pulse_protocol::thread::SubscriptionState::Archived => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::thread::SubscriptionState>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::thread::SubscriptionState>>
    for pulse_protocol::thread::SubscriptionState
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::thread::SubscriptionState> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ThreadPreview> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.topic_title.into_into_dart().into_dart(),
            self.0.reply_count.into_into_dart().into_dart(),
            self.0.latest_reply.into_into_dart().into_dart(),
            self.0.show_latest_reply.into_into_dart().into_dart(),
            self.0.subscription.into_into_dart().into_dart(),
            self.0.unread_replies.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ThreadPreview>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ThreadPreview>>
    for pulse_protocol::snapshot::ThreadPreview
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ThreadPreview> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ThreadSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.root_message_id.into_into_dart().into_dart(),
            self.0.root.into_into_dart().into_dart(),
            self.0.replies.into_into_dart().into_dart(),
            self.0.topic_title.into_into_dart().into_dart(),
            self.0.subscription.into_into_dart().into_dart(),
            self.0.syncing.into_into_dart().into_dart(),
            self.0.mentionable_stewards.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::ThreadSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::ThreadSnapshot>>
    for pulse_protocol::snapshot::ThreadSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::ThreadSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::TimelineEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.client_nonce.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.author_id.into_into_dart().into_dart(),
            self.0.author_display_name.into_into_dart().into_dart(),
            self.0.author_mention_token.into_into_dart().into_dart(),
            self.0.from_self.into_into_dart().into_dart(),
            self.0.body.into_into_dart().into_dart(),
            self.0.segments.into_into_dart().into_dart(),
            self.0.thread_root_id.into_into_dart().into_dart(),
            self.0.reactions.into_into_dart().into_dart(),
            self.0.thread.into_into_dart().into_dart(),
            self.0.delivery.into_into_dart().into_dart(),
            self.0.rejection.into_into_dart().into_dart(),
            self.0.attachments.into_into_dart().into_dart(),
            self.0.pending_drafts.into_into_dart().into_dart(),
            self.0.pinned.into_into_dart().into_dart(),
            self.0.shelved.into_into_dart().into_dart(),
            self.0.author_is_steward.into_into_dart().into_dart(),
            self.0.stream.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::TimelineEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::TimelineEntry>>
    for pulse_protocol::snapshot::TimelineEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::TimelineEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::TimelineSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.entries.into_into_dart().into_dart(),
            self.0.syncing.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
            self.0.reaction_palette.into_into_dart().into_dart(),
            self.0.drafts.into_into_dart().into_dart(),
            self.0.focus_message_id.into_into_dart().into_dart(),
            self.0.can_pin.into_into_dart().into_dart(),
            self.0.can_upload.into_into_dart().into_dart(),
            self.0.mentionable_stewards.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::TimelineSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::TimelineSnapshot>>
    for pulse_protocol::snapshot::TimelineSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::TimelineSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::ids::ToolCallId> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.0.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::ids::ToolCallId>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::ids::ToolCallId>>
    for pulse_protocol::ids::ToolCallId
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::ids::ToolCallId> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::steward::ToolDescriptor> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.name.into_into_dart().into_dart(),
            self.0.description.into_into_dart().into_dart(),
            self.0.input_schema.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::steward::ToolDescriptor>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::steward::ToolDescriptor>>
    for pulse_protocol::steward::ToolDescriptor
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::steward::ToolDescriptor> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::steward::ToolErrorCode> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::steward::ToolErrorCode::Forbidden => 0.into_dart(),
            pulse_protocol::steward::ToolErrorCode::UnknownTool => 1.into_dart(),
            pulse_protocol::steward::ToolErrorCode::InvalidArguments => 2.into_dart(),
            pulse_protocol::steward::ToolErrorCode::UnknownRoom => 3.into_dart(),
            pulse_protocol::steward::ToolErrorCode::UnknownThread => 4.into_dart(),
            pulse_protocol::steward::ToolErrorCode::Rejected => 5.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::steward::ToolErrorCode>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::steward::ToolErrorCode>>
    for pulse_protocol::steward::ToolErrorCode
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::steward::ToolErrorCode> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::member::UpdateMemberRequest> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.0.role.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::member::UpdateMemberRequest>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::member::UpdateMemberRequest>>
    for pulse_protocol::member::UpdateMemberRequest
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::member::UpdateMemberRequest> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::shelf::UpdateShelf> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.room_id.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.action.into_into_dart().into_dart(),
            self.0.remind_at_ms.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::shelf::UpdateShelf>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::shelf::UpdateShelf>>
    for pulse_protocol::shelf::UpdateShelf
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::shelf::UpdateShelf> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::UploadState> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::snapshot::UploadState::Uploading => 0.into_dart(),
            pulse_protocol::snapshot::UploadState::Uploaded => 1.into_dart(),
            pulse_protocol::snapshot::UploadState::Failed => 2.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::UploadState>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::UploadState>>
    for pulse_protocol::snapshot::UploadState
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::UploadState> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::VaultEntry> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.tab.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.message_id.into_into_dart().into_dart(),
            self.0.author_display_name.into_into_dart().into_dart(),
            self.0.preview.into_into_dart().into_dart(),
            self.0.attachment.into_into_dart().into_dart(),
            self.0.link.into_into_dart().into_dart(),
            self.0.pinned_at_ms.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::VaultEntry>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::VaultEntry>>
    for pulse_protocol::snapshot::VaultEntry
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::VaultEntry> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::VaultSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.open.into_into_dart().into_dart(),
            self.0.room_id.into_into_dart().into_dart(),
            self.0.tab.into_into_dart().into_dart(),
            self.0.entries.into_into_dart().into_dart(),
            self.0.loading.into_into_dart().into_dart(),
            self.0.has_more.into_into_dart().into_dart(),
            self.0.error.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::snapshot::VaultSnapshot>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::snapshot::VaultSnapshot>>
    for pulse_protocol::snapshot::VaultSnapshot
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::snapshot::VaultSnapshot> {
        self.into()
    }
}
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::media::VaultTab> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            pulse_protocol::media::VaultTab::Media => 0.into_dart(),
            pulse_protocol::media::VaultTab::Files => 1.into_dart(),
            pulse_protocol::media::VaultTab::Links => 2.into_dart(),
            pulse_protocol::media::VaultTab::Pinned => 3.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<pulse_protocol::media::VaultTab>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<pulse_protocol::media::VaultTab>>
    for pulse_protocol::media::VaultTab
{
    fn into_into_dart(self) -> FrbWrapper<pulse_protocol::media::VaultTab> {
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

impl SseEncode for flutter_rust_bridge::for_generated::anyhow::Error {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(format!("{:?}", self), serializer);
    }
}

impl SseEncode for StreamWriter {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>>>::sse_encode(flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self), serializer);
    }
}

impl SseEncode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>>
{
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        let (ptr, size) = self.sse_encode_raw();
        <usize>::sse_encode(ptr, serializer);
        <i32>::sse_encode(size, serializer);
    }
}

impl SseEncode
    for StreamSink<
        pulse_protocol::snapshot::ClientSnapshot,
        flutter_rust_bridge::for_generated::SseCodec,
    >
{
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        unimplemented!("")
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

impl SseEncode for pulse_protocol::member::AddMemberRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.email, serializer);
        <pulse_protocol::rbac::Role>::sse_encode(self.role, serializer);
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

impl SseEncode for pulse_protocol::ids::AttachmentId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::media::AttachmentKind {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::media::AttachmentKind::Image => 0,
                pulse_protocol::media::AttachmentKind::Video => 1,
                pulse_protocol::media::AttachmentKind::Audio => 2,
                pulse_protocol::media::AttachmentKind::File => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::AttachmentView {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AttachmentId>::sse_encode(self.attachment_id, serializer);
        <String>::sse_encode(self.file_name, serializer);
        <String>::sse_encode(self.content_type, serializer);
        <pulse_protocol::media::AttachmentKind>::sse_encode(self.kind, serializer);
        <u64>::sse_encode(self.byte_size, serializer);
        <Option<String>>::sse_encode(self.url, serializer);
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

impl SseEncode for pulse_protocol::auth::AuthToken {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::mention::BodySegment {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::mention::SegmentKind>::sse_encode(self.kind, serializer);
        <String>::sse_encode(self.text, serializer);
        <Option<pulse_protocol::ids::AccountId>>::sse_encode(self.account_id, serializer);
        <Option<String>>::sse_encode(self.url, serializer);
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

impl SseEncode for pulse_protocol::campus::CampusInvite {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::CampusId>::sse_encode(self.campus_id, serializer);
        <String>::sse_encode(self.code, serializer);
        <u64>::sse_encode(self.expires_at_ms, serializer);
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

impl SseEncode for pulse_protocol::ids::ClientNonce {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ClientSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <u64>::sse_encode(self.sequence, serializer);
        <pulse_protocol::snapshot::ConnectionSnapshot>::sse_encode(self.network, serializer);
        <pulse_protocol::snapshot::SessionSnapshot>::sse_encode(self.session, serializer);
        <pulse_protocol::snapshot::TimelineSnapshot>::sse_encode(self.timeline, serializer);
        <pulse_protocol::snapshot::ThreadSnapshot>::sse_encode(self.thread, serializer);
        <pulse_protocol::snapshot::InboxSnapshot>::sse_encode(self.inbox, serializer);
        <pulse_protocol::snapshot::SearchSnapshot>::sse_encode(self.search, serializer);
        <pulse_protocol::snapshot::VaultSnapshot>::sse_encode(self.vault, serializer);
        <pulse_protocol::snapshot::ShelfSnapshot>::sse_encode(self.shelf, serializer);
        <pulse_protocol::snapshot::LoungeSnapshot>::sse_encode(self.lounges, serializer);
        <pulse_protocol::snapshot::StewardSnapshot>::sse_encode(self.stewards, serializer);
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

impl SseEncode for pulse_protocol::media::CreateUploadRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.file_name, serializer);
        <String>::sse_encode(self.content_type, serializer);
        <u64>::sse_encode(self.byte_size, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::DeliveryState {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::DeliveryState::Pending => 0,
                pulse_protocol::snapshot::DeliveryState::Confirmed => 1,
                pulse_protocol::snapshot::DeliveryState::Failed => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::dev_seed::DevAccount {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.email, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <pulse_protocol::rbac::Role>::sse_encode(self.role, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::DraftAttachmentView {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::DraftId>::sse_encode(self.draft_id, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <String>::sse_encode(self.file_name, serializer);
        <pulse_protocol::media::AttachmentKind>::sse_encode(self.kind, serializer);
        <u64>::sse_encode(self.byte_size, serializer);
        <pulse_protocol::snapshot::UploadState>::sse_encode(self.upload, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::DraftId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
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

impl SseEncode for pulse_protocol::snapshot::HighlightSpan {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.text, serializer);
        <bool>::sse_encode(self.matched, serializer);
    }
}

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for pulse_protocol::snapshot::InboxItem {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::snapshot::InboxItemKind>::sse_encode(self.kind, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.root_message_id, serializer);
        <Option<String>>::sse_encode(self.topic_title, serializer);
        <String>::sse_encode(self.root_author_display_name, serializer);
        <String>::sse_encode(self.root_preview, serializer);
        <Option<pulse_protocol::snapshot::ReplyPreview>>::sse_encode(self.latest_reply, serializer);
        <u32>::sse_encode(self.reply_count, serializer);
        <u32>::sse_encode(self.unread_replies, serializer);
        <u32>::sse_encode(self.unread_mentions, serializer);
        <pulse_protocol::thread::SubscriptionState>::sse_encode(self.subscription, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.activity_message_id, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::InboxItemKind {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::InboxItemKind::Thread => 0,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::InboxSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.open, serializer);
        <Vec<pulse_protocol::snapshot::InboxItem>>::sse_encode(self.items, serializer);
        <u32>::sse_encode(self.unread_threads, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(
            self.selected_root_message_id,
            serializer,
        );
        <bool>::sse_encode(self.replying, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::InboxStep {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::InboxStep::Next => 0,
                pulse_protocol::snapshot::InboxStep::Previous => 1,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
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
                pulse_protocol::api::InputField::MessageBody => 4,
                pulse_protocol::api::InputField::TopicTitle => 5,
                pulse_protocol::api::InputField::ReactionEmoji => 6,
                pulse_protocol::api::InputField::FileName => 7,
                pulse_protocol::api::InputField::ContentType => 8,
                pulse_protocol::api::InputField::ByteSize => 9,
                pulse_protocol::api::InputField::Attachments => 10,
                pulse_protocol::api::InputField::RemindAt => 11,
                pulse_protocol::api::InputField::Rooms => 12,
                pulse_protocol::api::InputField::InviteCode => 13,
                pulse_protocol::api::InputField::Role => 14,
                pulse_protocol::api::InputField::NoteMembers => 15,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::campus::InviteCode {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::CampusId>::sse_encode(self.campus_id, serializer);
        <String>::sse_encode(self.secret, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::IssuedStewardToken {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.steward_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <pulse_protocol::auth::AuthToken>::sse_encode(self.token, serializer);
    }
}

impl SseEncode for pulse_protocol::campus::JoinCampusRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.code, serializer);
    }
}

impl SseEncode for Vec<String> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <String>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::ids::AccountId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::ids::AccountId>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::ids::AttachmentId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::ids::AttachmentId>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::AttachmentView> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::AttachmentView>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::mention::BodySegment> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::mention::BodySegment>::sse_encode(item, serializer);
        }
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

impl SseEncode for Vec<pulse_protocol::dev_seed::DevAccount> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::dev_seed::DevAccount>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::DraftAttachmentView> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::DraftAttachmentView>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::HighlightSpan> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::HighlightSpan>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::InboxItem> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::InboxItem>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::LoungeEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::LoungeEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::LoungeParticipantView> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::LoungeParticipantView>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::MemberEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::MemberEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::messaging::MentionRef> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::messaging::MentionRef>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::NoteEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::NoteEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::note::NoteMember> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::note::NoteMember>::sse_encode(item, serializer);
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

impl SseEncode for Vec<pulse_protocol::snapshot::ReactionSummary> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::ReactionSummary>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::shelf::ReminderPreset> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::shelf::ReminderPreset>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::rbac::Role> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::rbac::Role>::sse_encode(item, serializer);
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

impl SseEncode for Vec<pulse_protocol::ids::RoomId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::ids::RoomId>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::SearchHit> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::SearchHit>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::ShelfEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::ShelfEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::StewardEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::StewardEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::TimelineEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::TimelineEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::steward::ToolDescriptor> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::steward::ToolDescriptor>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<pulse_protocol::snapshot::VaultEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <pulse_protocol::snapshot::VaultEntry>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for pulse_protocol::snapshot::LoungeEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::LoungeId>::sse_encode(self.lounge_id, serializer);
        <pulse_protocol::lounge::LoungeScope>::sse_encode(self.scope, serializer);
        <Option<pulse_protocol::ids::RoomId>>::sse_encode(self.room_id, serializer);
        <Option<String>>::sse_encode(self.room_slug, serializer);
        <Vec<pulse_protocol::snapshot::LoungeParticipantView>>::sse_encode(
            self.participants,
            serializer,
        );
        <bool>::sse_encode(self.joined, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::LoungeId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::LoungeMediaState {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::LoungeMediaState::Idle => 0,
                pulse_protocol::snapshot::LoungeMediaState::Connecting => 1,
                pulse_protocol::snapshot::LoungeMediaState::Connected => 2,
                pulse_protocol::snapshot::LoungeMediaState::Failed => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::LoungeParticipantView {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.account_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <bool>::sse_encode(self.from_self, serializer);
        <bool>::sse_encode(self.can_speak, serializer);
    }
}

impl SseEncode for pulse_protocol::lounge::LoungeScope {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::lounge::LoungeScope::Campus => 0,
                pulse_protocol::lounge::LoungeScope::Room => 1,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::LoungeSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<pulse_protocol::snapshot::LoungeEntry>>::sse_encode(self.lounges, serializer);
        <Option<pulse_protocol::ids::LoungeId>>::sse_encode(self.joined_lounge_id, serializer);
        <bool>::sse_encode(self.joining, serializer);
        <Option<pulse_protocol::lounge::LoungeTicket>>::sse_encode(self.ticket, serializer);
        <pulse_protocol::snapshot::LoungeMediaState>::sse_encode(self.media, serializer);
        <bool>::sse_encode(self.can_join, serializer);
        <bool>::sse_encode(self.can_speak, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::lounge::LoungeTicket {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::LoungeId>::sse_encode(self.lounge_id, serializer);
        <String>::sse_encode(self.livekit_url, serializer);
        <String>::sse_encode(self.room_name, serializer);
        <String>::sse_encode(self.token, serializer);
        <u64>::sse_encode(self.expires_at_ms, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::MemberEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.account_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <String>::sse_encode(self.email, serializer);
        <pulse_protocol::rbac::Role>::sse_encode(self.role, serializer);
        <bool>::sse_encode(self.manageable, serializer);
        <bool>::sse_encode(self.is_self, serializer);
    }
}

impl SseEncode for pulse_protocol::messaging::MentionRef {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.account_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::MessageId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::messaging::MessageRejection {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::messaging::MessageRejection::Forbidden => 0,
                pulse_protocol::messaging::MessageRejection::InvalidBody => 1,
                pulse_protocol::messaging::MessageRejection::UnknownRoom => 2,
                pulse_protocol::messaging::MessageRejection::UnknownThread => 3,
                pulse_protocol::messaging::MessageRejection::InvalidAttachment => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::note::Note {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::CampusId>::sse_encode(self.campus_id, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <Vec<pulse_protocol::note::NoteMember>>::sse_encode(self.members, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::NoteEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <String>::sse_encode(self.title, serializer);
        <u32>::sse_encode(self.member_count, serializer);
        <bool>::sse_encode(self.group, serializer);
    }
}

impl SseEncode for pulse_protocol::note::NoteMember {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.account_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
    }
}

impl SseEncode for Option<String> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <String>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::AccountId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::AccountId>::sse_encode(value, serializer);
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

impl SseEncode for Option<pulse_protocol::snapshot::AttachmentView> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::AttachmentView>::sse_encode(value, serializer);
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

impl SseEncode for Option<pulse_protocol::campus::CampusInvite> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::campus::CampusInvite>::sse_encode(value, serializer);
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

impl SseEncode for Option<pulse_protocol::snapshot::IssuedStewardToken> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::IssuedStewardToken>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::LoungeId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::LoungeId>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::lounge::LoungeTicket> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::lounge::LoungeTicket>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::MessageId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::MessageId>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::messaging::MessageRejection> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::messaging::MessageRejection>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::shelf::ReminderPreset> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::shelf::ReminderPreset>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::snapshot::ReplyPreview> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::ReplyPreview>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::RoomId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::RoomId>::sse_encode(value, serializer);
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

impl SseEncode for Option<pulse_protocol::snapshot::ShelfEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::ShelfEntry>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::ids::ShelfItemId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::ShelfItemId>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::steward::StewardTool> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::steward::StewardTool>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::snapshot::StreamState> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::StreamState>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::thread::SubscriptionState> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::thread::SubscriptionState>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::snapshot::ThreadPreview> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::ThreadPreview>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<pulse_protocol::snapshot::TimelineEntry> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::snapshot::TimelineEntry>::sse_encode(value, serializer);
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

impl SseEncode for Option<u64> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <u64>::sse_encode(value, serializer);
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
                pulse_protocol::rbac::Permission::PostMessage => 5,
                pulse_protocol::rbac::Permission::React => 6,
                pulse_protocol::rbac::Permission::PinMessage => 7,
                pulse_protocol::rbac::Permission::UploadAttachment => 8,
                pulse_protocol::rbac::Permission::ManageStewards => 9,
                pulse_protocol::rbac::Permission::StreamReply => 10,
                pulse_protocol::rbac::Permission::UseStewardTools => 11,
                pulse_protocol::rbac::Permission::JoinLounge => 12,
                pulse_protocol::rbac::Permission::SpeakInLounge => 13,
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

impl SseEncode for pulse_protocol::reaction::React {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <String>::sse_encode(self.emoji, serializer);
        <pulse_protocol::reaction::ReactionAction>::sse_encode(self.action, serializer);
    }
}

impl SseEncode for pulse_protocol::reaction::ReactionAction {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::reaction::ReactionAction::Add => 0,
                pulse_protocol::reaction::ReactionAction::Remove => 1,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::ReactionSummary {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.emoji, serializer);
        <u32>::sse_encode(self.count, serializer);
        <bool>::sse_encode(self.reacted_by_self, serializer);
    }
}

impl SseEncode for (String, Vec<pulse_protocol::ids::AttachmentId>) {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.0, serializer);
        <Vec<pulse_protocol::ids::AttachmentId>>::sse_encode(self.1, serializer);
    }
}

impl SseEncode for pulse_protocol::shelf::ReminderPreset {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::shelf::ReminderPreset::InTwentyMinutes => 0,
                pulse_protocol::shelf::ReminderPreset::InOneHour => 1,
                pulse_protocol::shelf::ReminderPreset::InThreeHours => 2,
                pulse_protocol::shelf::ReminderPreset::Tomorrow => 3,
                pulse_protocol::shelf::ReminderPreset::NextWeek => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::ReplyPreview {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <pulse_protocol::ids::AccountId>::sse_encode(self.author_id, serializer);
        <String>::sse_encode(self.author_display_name, serializer);
        <String>::sse_encode(self.preview, serializer);
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
                pulse_protocol::rbac::Role::Steward => 4,
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

impl SseEncode for pulse_protocol::snapshot::SearchHit {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.thread_root_id, serializer);
        <String>::sse_encode(self.author_display_name, serializer);
        <Option<String>>::sse_encode(self.topic_title, serializer);
        <Vec<pulse_protocol::snapshot::HighlightSpan>>::sse_encode(self.snippet, serializer);
        <u32>::sse_encode(self.score, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::SearchSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.open, serializer);
        <String>::sse_encode(self.query, serializer);
        <Vec<pulse_protocol::snapshot::SearchHit>>::sse_encode(self.hits, serializer);
        <Option<u32>>::sse_encode(self.selected_index, serializer);
        <bool>::sse_encode(self.searching, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::mention::SegmentKind {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::mention::SegmentKind::Text => 0,
                pulse_protocol::mention::SegmentKind::Mention => 1,
                pulse_protocol::mention::SegmentKind::Link => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::messaging::SendMessage {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::ClientNonce>::sse_encode(self.client_nonce, serializer);
        <String>::sse_encode(self.body, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.thread_root_id, serializer);
        <Vec<pulse_protocol::ids::AttachmentId>>::sse_encode(self.attachment_ids, serializer);
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
        <Option<pulse_protocol::ids::RoomId>>::sse_encode(self.active_room_id, serializer);
        <Option<pulse_protocol::campus::CampusInvite>>::sse_encode(self.invite, serializer);
        <bool>::sse_encode(self.can_invite, serializer);
        <Vec<pulse_protocol::snapshot::MemberEntry>>::sse_encode(self.members, serializer);
        <Vec<pulse_protocol::rbac::Role>>::sse_encode(self.assignable_roles, serializer);
        <Vec<pulse_protocol::dev_seed::DevAccount>>::sse_encode(self.quick_accounts, serializer);
        <Vec<pulse_protocol::snapshot::NoteEntry>>::sse_encode(self.notes, serializer);
        <Vec<pulse_protocol::snapshot::MemberEntry>>::sse_encode(self.note_candidates, serializer);
        <bool>::sse_encode(self.busy, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::thread::SetTopicTitle {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.root_message_id, serializer);
        <String>::sse_encode(self.topic_title, serializer);
    }
}

impl SseEncode for pulse_protocol::shelf::ShelfAction {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::shelf::ShelfAction::Save => 0,
                pulse_protocol::shelf::ShelfAction::Reschedule => 1,
                pulse_protocol::shelf::ShelfAction::Complete => 2,
                pulse_protocol::shelf::ShelfAction::Reopen => 3,
                pulse_protocol::shelf::ShelfAction::Remove => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::shelf::ShelfBucket {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::shelf::ShelfBucket::Due => 0,
                pulse_protocol::shelf::ShelfBucket::Upcoming => 1,
                pulse_protocol::shelf::ShelfBucket::Done => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::ShelfEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Option<pulse_protocol::ids::ShelfItemId>>::sse_encode(self.shelf_item_id, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <String>::sse_encode(self.author_display_name, serializer);
        <String>::sse_encode(self.preview, serializer);
        <Option<u64>>::sse_encode(self.remind_at_ms, serializer);
        <Option<u64>>::sse_encode(self.done_at_ms, serializer);
        <pulse_protocol::shelf::ShelfBucket>::sse_encode(self.bucket, serializer);
        <bool>::sse_encode(self.pending, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::ShelfItemId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ShelfSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.open, serializer);
        <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_encode(self.due, serializer);
        <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_encode(self.upcoming, serializer);
        <Vec<pulse_protocol::snapshot::ShelfEntry>>::sse_encode(self.done, serializer);
        <u32>::sse_encode(self.due_count, serializer);
        <Option<pulse_protocol::snapshot::ShelfEntry>>::sse_encode(
            self.latest_reminder,
            serializer,
        );
        <Vec<pulse_protocol::shelf::ReminderPreset>>::sse_encode(self.presets, serializer);
        <Option<u64>>::sse_encode(self.picker_anchor_ms, serializer);
        <Option<u64>>::sse_encode(self.picker_latest_ms, serializer);
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

impl SseEncode for pulse_protocol::snapshot::StewardEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::AccountId>::sse_encode(self.steward_id, serializer);
        <String>::sse_encode(self.display_name, serializer);
        <String>::sse_encode(self.mention_token, serializer);
        <Vec<pulse_protocol::ids::RoomId>>::sse_encode(self.room_ids, serializer);
        <bool>::sse_encode(self.revoked, serializer);
        <bool>::sse_encode(self.manageable, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::StewardSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.open, serializer);
        <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_encode(self.stewards, serializer);
        <bool>::sse_encode(self.can_manage, serializer);
        <Option<pulse_protocol::snapshot::IssuedStewardToken>>::sse_encode(self.issued, serializer);
        <bool>::sse_encode(self.pending, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::steward::StewardTool {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::steward::StewardTool::ListRooms => 0,
                pulse_protocol::steward::StewardTool::ReadRoomHistory => 1,
                pulse_protocol::steward::StewardTool::ReadThread => 2,
                pulse_protocol::steward::StewardTool::PostMessage => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::steward::StreamChunk {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::StreamId>::sse_encode(self.stream_id, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.thread_root_id, serializer);
        <pulse_protocol::steward::StreamPhase>::sse_encode(self.phase, serializer);
        <u32>::sse_encode(self.sequence, serializer);
        <String>::sse_encode(self.text, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::StreamId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::steward::StreamPhase {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::steward::StreamPhase::Begin => 0,
                pulse_protocol::steward::StreamPhase::Delta => 1,
                pulse_protocol::steward::StreamPhase::End => 2,
                pulse_protocol::steward::StreamPhase::Abort => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::StreamState {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::StreamState::Streaming => 0,
                pulse_protocol::snapshot::StreamState::Finalizing => 1,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::thread::SubscriptionState {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::thread::SubscriptionState::Following => 0,
                pulse_protocol::thread::SubscriptionState::Muted => 1,
                pulse_protocol::thread::SubscriptionState::Archived => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::ThreadPreview {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Option<String>>::sse_encode(self.topic_title, serializer);
        <u32>::sse_encode(self.reply_count, serializer);
        <Option<pulse_protocol::snapshot::ReplyPreview>>::sse_encode(self.latest_reply, serializer);
        <bool>::sse_encode(self.show_latest_reply, serializer);
        <Option<pulse_protocol::thread::SubscriptionState>>::sse_encode(
            self.subscription,
            serializer,
        );
        <u32>::sse_encode(self.unread_replies, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ThreadSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Option<pulse_protocol::ids::RoomId>>::sse_encode(self.room_id, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.root_message_id, serializer);
        <Option<pulse_protocol::snapshot::TimelineEntry>>::sse_encode(self.root, serializer);
        <Vec<pulse_protocol::snapshot::TimelineEntry>>::sse_encode(self.replies, serializer);
        <Option<String>>::sse_encode(self.topic_title, serializer);
        <Option<pulse_protocol::thread::SubscriptionState>>::sse_encode(
            self.subscription,
            serializer,
        );
        <bool>::sse_encode(self.syncing, serializer);
        <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_encode(
            self.mentionable_stewards,
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::snapshot::TimelineEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::ClientNonce>::sse_encode(self.client_nonce, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.message_id, serializer);
        <pulse_protocol::ids::AccountId>::sse_encode(self.author_id, serializer);
        <String>::sse_encode(self.author_display_name, serializer);
        <String>::sse_encode(self.author_mention_token, serializer);
        <bool>::sse_encode(self.from_self, serializer);
        <String>::sse_encode(self.body, serializer);
        <Vec<pulse_protocol::mention::BodySegment>>::sse_encode(self.segments, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.thread_root_id, serializer);
        <Vec<pulse_protocol::snapshot::ReactionSummary>>::sse_encode(self.reactions, serializer);
        <Option<pulse_protocol::snapshot::ThreadPreview>>::sse_encode(self.thread, serializer);
        <pulse_protocol::snapshot::DeliveryState>::sse_encode(self.delivery, serializer);
        <Option<pulse_protocol::messaging::MessageRejection>>::sse_encode(
            self.rejection,
            serializer,
        );
        <Vec<pulse_protocol::snapshot::AttachmentView>>::sse_encode(self.attachments, serializer);
        <Vec<pulse_protocol::snapshot::DraftAttachmentView>>::sse_encode(
            self.pending_drafts,
            serializer,
        );
        <bool>::sse_encode(self.pinned, serializer);
        <bool>::sse_encode(self.shelved, serializer);
        <bool>::sse_encode(self.author_is_steward, serializer);
        <Option<pulse_protocol::snapshot::StreamState>>::sse_encode(self.stream, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::TimelineSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Option<pulse_protocol::ids::RoomId>>::sse_encode(self.room_id, serializer);
        <Vec<pulse_protocol::snapshot::TimelineEntry>>::sse_encode(self.entries, serializer);
        <bool>::sse_encode(self.syncing, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
        <Vec<String>>::sse_encode(self.reaction_palette, serializer);
        <Vec<pulse_protocol::snapshot::DraftAttachmentView>>::sse_encode(self.drafts, serializer);
        <Option<pulse_protocol::ids::MessageId>>::sse_encode(self.focus_message_id, serializer);
        <bool>::sse_encode(self.can_pin, serializer);
        <bool>::sse_encode(self.can_upload, serializer);
        <Vec<pulse_protocol::snapshot::StewardEntry>>::sse_encode(
            self.mentionable_stewards,
            serializer,
        );
    }
}

impl SseEncode for pulse_protocol::ids::ToolCallId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::steward::ToolDescriptor {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.name, serializer);
        <String>::sse_encode(self.description, serializer);
        <String>::sse_encode(self.input_schema, serializer);
    }
}

impl SseEncode for pulse_protocol::steward::ToolErrorCode {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::steward::ToolErrorCode::Forbidden => 0,
                pulse_protocol::steward::ToolErrorCode::UnknownTool => 1,
                pulse_protocol::steward::ToolErrorCode::InvalidArguments => 2,
                pulse_protocol::steward::ToolErrorCode::UnknownRoom => 3,
                pulse_protocol::steward::ToolErrorCode::UnknownThread => 4,
                pulse_protocol::steward::ToolErrorCode::Rejected => 5,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
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

impl SseEncode for u64 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u64::<NativeEndian>(self).unwrap();
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

impl SseEncode for pulse_protocol::member::UpdateMemberRequest {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::rbac::Role>::sse_encode(self.role, serializer);
    }
}

impl SseEncode for pulse_protocol::shelf::UpdateShelf {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <pulse_protocol::shelf::ShelfAction>::sse_encode(self.action, serializer);
        <Option<u64>>::sse_encode(self.remind_at_ms, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::UploadState {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::snapshot::UploadState::Uploading => 0,
                pulse_protocol::snapshot::UploadState::Uploaded => 1,
                pulse_protocol::snapshot::UploadState::Failed => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for usize {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

impl SseEncode for pulse_protocol::snapshot::VaultEntry {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::media::VaultTab>::sse_encode(self.tab, serializer);
        <pulse_protocol::ids::RoomId>::sse_encode(self.room_id, serializer);
        <pulse_protocol::ids::MessageId>::sse_encode(self.message_id, serializer);
        <String>::sse_encode(self.author_display_name, serializer);
        <String>::sse_encode(self.preview, serializer);
        <Option<pulse_protocol::snapshot::AttachmentView>>::sse_encode(self.attachment, serializer);
        <Option<String>>::sse_encode(self.link, serializer);
        <Option<u64>>::sse_encode(self.pinned_at_ms, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::VaultSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.open, serializer);
        <Option<pulse_protocol::ids::RoomId>>::sse_encode(self.room_id, serializer);
        <pulse_protocol::media::VaultTab>::sse_encode(self.tab, serializer);
        <Vec<pulse_protocol::snapshot::VaultEntry>>::sse_encode(self.entries, serializer);
        <bool>::sse_encode(self.loading, serializer);
        <bool>::sse_encode(self.has_more, serializer);
        <Option<pulse_protocol::api::ApiError>>::sse_encode(self.error, serializer);
    }
}

impl SseEncode for pulse_protocol::media::VaultTab {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                pulse_protocol::media::VaultTab::Media => 0,
                pulse_protocol::media::VaultTab::Files => 1,
                pulse_protocol::media::VaultTab::Links => 2,
                pulse_protocol::media::VaultTab::Pinned => 3,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
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
    use pulse_protocol::steward::*;
    use pulse_protocol::validation::Validate;

    flutter_rust_bridge::frb_generated_boilerplate_io!();

    #[unsafe(no_mangle)]
    pub extern "C" fn frbgen_pulse_mobile_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>>::increment_strong_count(ptr as _);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn frbgen_pulse_mobile_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StreamWriter>>::decrement_strong_count(ptr as _);
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
