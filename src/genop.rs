// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `genop.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct GenopArg {
    // message fields
    pub size: u32,
    pub buf: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GenopArg {
    fn default() -> &'a GenopArg {
        <GenopArg as ::protobuf::Message>::default_instance()
    }
}

impl GenopArg {
    pub fn new() -> GenopArg {
        ::std::default::Default::default()
    }

    // uint32 size = 1;


    pub fn get_size(&self) -> u32 {
        self.size
    }
    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u32) {
        self.size = v;
    }

    // bytes buf = 2;


    pub fn get_buf(&self) -> &[u8] {
        &self.buf
    }
    pub fn clear_buf(&mut self) {
        self.buf.clear();
    }

    // Param is passed by value, moved
    pub fn set_buf(&mut self, v: ::std::vec::Vec<u8>) {
        self.buf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buf(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.buf
    }

    // Take field
    pub fn take_buf(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.buf, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for GenopArg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.size = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.buf)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.buf.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.buf);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_uint32(1, self.size)?;
        }
        if !self.buf.is_empty() {
            os.write_bytes(2, &self.buf)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GenopArg {
        GenopArg::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "size",
                |m: &GenopArg| { &m.size },
                |m: &mut GenopArg| { &mut m.size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "buf",
                |m: &GenopArg| { &m.buf },
                |m: &mut GenopArg| { &mut m.buf },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GenopArg>(
                "GenopArg",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GenopArg {
        static instance: ::protobuf::rt::LazyV2<GenopArg> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GenopArg::new)
    }
}

impl ::protobuf::Clear for GenopArg {
    fn clear(&mut self) {
        self.size = 0;
        self.buf.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenopArg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenopArg {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenopRequest {
    // message fields
    pub session_id: u32,
    pub read_args: ::protobuf::RepeatedField<GenopArg>,
    pub write_args: ::protobuf::RepeatedField<GenopArg>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GenopRequest {
    fn default() -> &'a GenopRequest {
        <GenopRequest as ::protobuf::Message>::default_instance()
    }
}

impl GenopRequest {
    pub fn new() -> GenopRequest {
        ::std::default::Default::default()
    }

    // uint32 session_id = 1;


    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn clear_session_id(&mut self) {
        self.session_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_session_id(&mut self, v: u32) {
        self.session_id = v;
    }

    // repeated .vaccel.GenopArg read_args = 2;


    pub fn get_read_args(&self) -> &[GenopArg] {
        &self.read_args
    }
    pub fn clear_read_args(&mut self) {
        self.read_args.clear();
    }

    // Param is passed by value, moved
    pub fn set_read_args(&mut self, v: ::protobuf::RepeatedField<GenopArg>) {
        self.read_args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_read_args(&mut self) -> &mut ::protobuf::RepeatedField<GenopArg> {
        &mut self.read_args
    }

    // Take field
    pub fn take_read_args(&mut self) -> ::protobuf::RepeatedField<GenopArg> {
        ::std::mem::replace(&mut self.read_args, ::protobuf::RepeatedField::new())
    }

    // repeated .vaccel.GenopArg write_args = 3;


    pub fn get_write_args(&self) -> &[GenopArg] {
        &self.write_args
    }
    pub fn clear_write_args(&mut self) {
        self.write_args.clear();
    }

    // Param is passed by value, moved
    pub fn set_write_args(&mut self, v: ::protobuf::RepeatedField<GenopArg>) {
        self.write_args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_write_args(&mut self) -> &mut ::protobuf::RepeatedField<GenopArg> {
        &mut self.write_args
    }

    // Take field
    pub fn take_write_args(&mut self) -> ::protobuf::RepeatedField<GenopArg> {
        ::std::mem::replace(&mut self.write_args, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GenopRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.read_args {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.write_args {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.session_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.read_args)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.write_args)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.session_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.session_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.read_args {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.write_args {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.session_id != 0 {
            os.write_uint32(1, self.session_id)?;
        }
        for v in &self.read_args {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.write_args {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GenopRequest {
        GenopRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "session_id",
                |m: &GenopRequest| { &m.session_id },
                |m: &mut GenopRequest| { &mut m.session_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GenopArg>>(
                "read_args",
                |m: &GenopRequest| { &m.read_args },
                |m: &mut GenopRequest| { &mut m.read_args },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GenopArg>>(
                "write_args",
                |m: &GenopRequest| { &m.write_args },
                |m: &mut GenopRequest| { &mut m.write_args },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GenopRequest>(
                "GenopRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GenopRequest {
        static instance: ::protobuf::rt::LazyV2<GenopRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GenopRequest::new)
    }
}

impl ::protobuf::Clear for GenopRequest {
    fn clear(&mut self) {
        self.session_id = 0;
        self.read_args.clear();
        self.write_args.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenopRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenopRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenopResult {
    // message fields
    pub write_args: ::protobuf::RepeatedField<GenopArg>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GenopResult {
    fn default() -> &'a GenopResult {
        <GenopResult as ::protobuf::Message>::default_instance()
    }
}

impl GenopResult {
    pub fn new() -> GenopResult {
        ::std::default::Default::default()
    }

    // repeated .vaccel.GenopArg write_args = 1;


    pub fn get_write_args(&self) -> &[GenopArg] {
        &self.write_args
    }
    pub fn clear_write_args(&mut self) {
        self.write_args.clear();
    }

    // Param is passed by value, moved
    pub fn set_write_args(&mut self, v: ::protobuf::RepeatedField<GenopArg>) {
        self.write_args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_write_args(&mut self) -> &mut ::protobuf::RepeatedField<GenopArg> {
        &mut self.write_args
    }

    // Take field
    pub fn take_write_args(&mut self) -> ::protobuf::RepeatedField<GenopArg> {
        ::std::mem::replace(&mut self.write_args, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GenopResult {
    fn is_initialized(&self) -> bool {
        for v in &self.write_args {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.write_args)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.write_args {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.write_args {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GenopResult {
        GenopResult::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GenopArg>>(
                "write_args",
                |m: &GenopResult| { &m.write_args },
                |m: &mut GenopResult| { &mut m.write_args },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GenopResult>(
                "GenopResult",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GenopResult {
        static instance: ::protobuf::rt::LazyV2<GenopResult> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GenopResult::new)
    }
}

impl ::protobuf::Clear for GenopResult {
    fn clear(&mut self) {
        self.write_args.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenopResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenopResult {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenopResponse {
    // message fields
    pub timer_buf: ::std::string::String,
    // message oneof groups
    pub result: ::std::option::Option<GenopResponse_oneof_result>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GenopResponse {
    fn default() -> &'a GenopResponse {
        <GenopResponse as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum GenopResponse_oneof_result {
    error(super::error::VaccelError),
    result(GenopResult),
}

impl GenopResponse {
    pub fn new() -> GenopResponse {
        ::std::default::Default::default()
    }

    // .vaccel.VaccelError error = 1;


    pub fn get_error(&self) -> &super::error::VaccelError {
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::error(ref v)) => v,
            _ => <super::error::VaccelError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_error(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::error::VaccelError) {
        self.result = ::std::option::Option::Some(GenopResponse_oneof_result::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut super::error::VaccelError {
        if let ::std::option::Option::Some(GenopResponse_oneof_result::error(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(GenopResponse_oneof_result::error(super::error::VaccelError::new()));
        }
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> super::error::VaccelError {
        if self.has_error() {
            match self.result.take() {
                ::std::option::Option::Some(GenopResponse_oneof_result::error(v)) => v,
                _ => panic!(),
            }
        } else {
            super::error::VaccelError::new()
        }
    }

    // .vaccel.GenopResult result = 2;


    pub fn get_result(&self) -> &GenopResult {
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::result(ref v)) => v,
            _ => <GenopResult as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: GenopResult) {
        self.result = ::std::option::Option::Some(GenopResponse_oneof_result::result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_result(&mut self) -> &mut GenopResult {
        if let ::std::option::Option::Some(GenopResponse_oneof_result::result(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(GenopResponse_oneof_result::result(GenopResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(GenopResponse_oneof_result::result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_result(&mut self) -> GenopResult {
        if self.has_result() {
            match self.result.take() {
                ::std::option::Option::Some(GenopResponse_oneof_result::result(v)) => v,
                _ => panic!(),
            }
        } else {
            GenopResult::new()
        }
    }

    // string timer_buf = 3;


    pub fn get_timer_buf(&self) -> &str {
        &self.timer_buf
    }
    pub fn clear_timer_buf(&mut self) {
        self.timer_buf.clear();
    }

    // Param is passed by value, moved
    pub fn set_timer_buf(&mut self, v: ::std::string::String) {
        self.timer_buf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timer_buf(&mut self) -> &mut ::std::string::String {
        &mut self.timer_buf
    }

    // Take field
    pub fn take_timer_buf(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.timer_buf, ::std::string::String::new())
    }
}

impl ::protobuf::Message for GenopResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GenopResponse_oneof_result::error(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(GenopResponse_oneof_result::result(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(GenopResponse_oneof_result::error(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(GenopResponse_oneof_result::result(is.read_message()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.timer_buf)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.timer_buf.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.timer_buf);
        }
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &GenopResponse_oneof_result::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenopResponse_oneof_result::result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.timer_buf.is_empty() {
            os.write_string(3, &self.timer_buf)?;
        }
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &GenopResponse_oneof_result::error(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenopResponse_oneof_result::result(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GenopResponse {
        GenopResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::error::VaccelError>(
                "error",
                GenopResponse::has_error,
                GenopResponse::get_error,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GenopResult>(
                "result",
                GenopResponse::has_result,
                GenopResponse::get_result,
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "timer_buf",
                |m: &GenopResponse| { &m.timer_buf },
                |m: &mut GenopResponse| { &mut m.timer_buf },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GenopResponse>(
                "GenopResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GenopResponse {
        static instance: ::protobuf::rt::LazyV2<GenopResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GenopResponse::new)
    }
}

impl ::protobuf::Clear for GenopResponse {
    fn clear(&mut self) {
        self.result = ::std::option::Option::None;
        self.result = ::std::option::Option::None;
        self.timer_buf.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenopResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenopResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bgenop.proto\x12\x06vaccel\x1a\x0berror.proto\"6\n\x08GenopArg\x12\
    \x14\n\x04size\x18\x01\x20\x01(\rR\x04sizeB\0\x12\x12\n\x03buf\x18\x02\
    \x20\x01(\x0cR\x03bufB\0:\0\"\x95\x01\n\x0cGenopRequest\x12\x1f\n\nsessi\
    on_id\x18\x01\x20\x01(\rR\tsessionIdB\0\x12/\n\tread_args\x18\x02\x20\
    \x03(\x0b2\x10.vaccel.GenopArgR\x08readArgsB\0\x121\n\nwrite_args\x18\
    \x03\x20\x03(\x0b2\x10.vaccel.GenopArgR\twriteArgsB\0:\0\"B\n\x0bGenopRe\
    sult\x121\n\nwrite_args\x18\x01\x20\x03(\x0b2\x10.vaccel.GenopArgR\twrit\
    eArgsB\0:\0\"\x9a\x01\n\rGenopResponse\x12-\n\x05error\x18\x01\x20\x01(\
    \x0b2\x13.vaccel.VaccelErrorH\0R\x05errorB\0\x12/\n\x06result\x18\x02\
    \x20\x01(\x0b2\x13.vaccel.GenopResultH\0R\x06resultB\0\x12\x1d\n\ttimer_\
    buf\x18\x03\x20\x01(\tR\x08timerBufB\0B\x08\n\x06result:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
