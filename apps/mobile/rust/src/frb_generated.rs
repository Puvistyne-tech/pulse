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

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.13.0";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -190957032;

flutter_rust_bridge::frb_generated_default_handler!();

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
fn wire__crate__api__snapshot__initial_snapshot_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::SseCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "initial_snapshot",
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
                            Ok::<_, ()>(crate::api::snapshot::initial_snapshot().await)?;
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

#[allow(clippy::unnecessary_literal_unwrap)]
const _: fn() = || {
    {
        let CampusId_ = None::<pulse_protocol::ids::CampusId>.unwrap();
        let _: uuid::Uuid = CampusId_.0;
    }
    {
        let ClientSnapshot = None::<pulse_protocol::snapshot::ClientSnapshot>.unwrap();
        let _: pulse_protocol::version::ProtocolVersion = ClientSnapshot.protocol_version;
        let _: pulse_protocol::snapshot::ConnectionPhase = ClientSnapshot.connection;
        let _: Option<u32> = ClientSnapshot.retry_in_ms;
        let _: Option<pulse_protocol::ids::CampusId> = ClientSnapshot.campus_id;
        let _: Option<pulse_protocol::ids::SessionId> = ClientSnapshot.session_id;
        let _: u32 = ClientSnapshot.attempt;
        let _: Option<pulse_protocol::frames::VersionMismatch> = ClientSnapshot.rejection;
    }
    {
        let ProtocolVersion_ = None::<pulse_protocol::version::ProtocolVersion>.unwrap();
        let _: u16 = ProtocolVersion_.0;
    }
    {
        let SessionId_ = None::<pulse_protocol::ids::SessionId>.unwrap();
        let _: uuid::Uuid = SessionId_.0;
    }
    {
        let VersionMismatch = None::<pulse_protocol::frames::VersionMismatch>.unwrap();
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.supported;
        let _: pulse_protocol::version::ProtocolVersion = VersionMismatch.received;
    }
};

impl SseDecode for uuid::Uuid {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return uuid::Uuid::from_slice(&inner).expect("fail to decode uuid");
    }
}

impl SseDecode for pulse_protocol::ids::CampusId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::CampusId(var_field0);
    }
}

impl SseDecode for pulse_protocol::snapshot::ClientSnapshot {
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
        return pulse_protocol::snapshot::ClientSnapshot {
            protocol_version: var_protocolVersion,
            connection: var_connection,
            retry_in_ms: var_retryInMs,
            campus_id: var_campusId,
            session_id: var_sessionId,
            attempt: var_attempt,
            rejection: var_rejection,
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

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
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

impl SseDecode for Option<pulse_protocol::ids::CampusId> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<pulse_protocol::ids::CampusId>::sse_decode(deserializer));
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

impl SseDecode for pulse_protocol::version::ProtocolVersion {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <u16>::sse_decode(deserializer);
        return pulse_protocol::version::ProtocolVersion(var_field0);
    }
}

impl SseDecode for pulse_protocol::ids::SessionId {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <uuid::Uuid>::sse_decode(deserializer);
        return pulse_protocol::ids::SessionId(var_field0);
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

impl SseDecode for bool {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
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
        1 => wire__pulse_protocol__ids__campus_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        2 => wire__pulse_protocol__ids__campus_id_generate_impl(port, ptr, rust_vec_len, data_len),
        3 => wire__crate__api__snapshot__initial_snapshot_impl(port, ptr, rust_vec_len, data_len),
        4 => wire__pulse_protocol__ids__session_id_as_uuid_impl(port, ptr, rust_vec_len, data_len),
        5 => wire__pulse_protocol__ids__session_id_generate_impl(port, ptr, rust_vec_len, data_len),
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
impl flutter_rust_bridge::IntoDart for FrbWrapper<pulse_protocol::snapshot::ClientSnapshot> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.protocol_version.into_into_dart().into_dart(),
            self.0.connection.into_into_dart().into_dart(),
            self.0.retry_in_ms.into_into_dart().into_dart(),
            self.0.campus_id.into_into_dart().into_dart(),
            self.0.session_id.into_into_dart().into_dart(),
            self.0.attempt.into_into_dart().into_dart(),
            self.0.rejection.into_into_dart().into_dart(),
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

impl SseEncode for uuid::Uuid {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.as_bytes().to_vec(), serializer);
    }
}

impl SseEncode for pulse_protocol::ids::CampusId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::snapshot::ClientSnapshot {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <pulse_protocol::version::ProtocolVersion>::sse_encode(self.protocol_version, serializer);
        <pulse_protocol::snapshot::ConnectionPhase>::sse_encode(self.connection, serializer);
        <Option<u32>>::sse_encode(self.retry_in_ms, serializer);
        <Option<pulse_protocol::ids::CampusId>>::sse_encode(self.campus_id, serializer);
        <Option<pulse_protocol::ids::SessionId>>::sse_encode(self.session_id, serializer);
        <u32>::sse_encode(self.attempt, serializer);
        <Option<pulse_protocol::frames::VersionMismatch>>::sse_encode(self.rejection, serializer);
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

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
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

impl SseEncode for Option<pulse_protocol::ids::CampusId> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <pulse_protocol::ids::CampusId>::sse_encode(value, serializer);
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

impl SseEncode for pulse_protocol::version::ProtocolVersion {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <u16>::sse_encode(self.0, serializer);
    }
}

impl SseEncode for pulse_protocol::ids::SessionId {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <uuid::Uuid>::sse_encode(self.0, serializer);
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

impl SseEncode for bool {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
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

    flutter_rust_bridge::frb_generated_boilerplate_io!();
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
