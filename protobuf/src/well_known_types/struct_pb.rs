// This file is generated by rust-protobuf 2.9.0-pre. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/protobuf/struct.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Struct {
    // message fields
    pub fields: ::std::collections::HashMap<::std::string::String, Value>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: crate::CachedSize,
}

impl<'a> ::std::default::Default for &'a Struct {
    fn default() -> &'a Struct {
        <Struct as crate::Message>::default_instance()
    }
}

impl Struct {
    pub fn new() -> Struct {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Struct.FieldsEntry fields = 1;


    pub fn get_fields(&self) -> &::std::collections::HashMap<::std::string::String, Value> {
        &self.fields
    }
    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::std::collections::HashMap<::std::string::String, Value>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Value> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::std::collections::HashMap<::std::string::String, Value> {
        ::std::mem::replace(&mut self.fields, ::std::collections::HashMap::new())
    }
}

impl crate::Message for Struct {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, crate::types::ProtobufTypeMessage<Value>>(wire_type, is, &mut self.fields)?;
                },
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += crate::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, crate::types::ProtobufTypeMessage<Value>>(1, &self.fields);
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, crate::types::ProtobufTypeMessage<Value>>(1, &self.fields, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static crate::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Struct {
        Struct::new()
    }

    fn descriptor_static() -> &'static crate::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<crate::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const crate::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(crate::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, crate::types::ProtobufTypeMessage<Value>>(
                    "fields",
                    |m: &Struct| { &m.fields },
                    |m: &mut Struct| { &mut m.fields },
                ));
                crate::reflect::MessageDescriptor::new::<Struct>(
                    "Struct",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Struct {
        static mut instance: ::protobuf::lazy::Lazy<Struct> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Struct,
        };
        unsafe {
            instance.get(Struct::new)
        }
    }
}

impl crate::Clear for Struct {
    fn clear(&mut self) {
        self.fields.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for Struct {
    fn as_ref(&self) -> crate::reflect::ReflectValueRef {
        crate::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Value {
    // message oneof groups
    pub kind: ::std::option::Option<Value_oneof_kind>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: crate::CachedSize,
}

impl<'a> ::std::default::Default for &'a Value {
    fn default() -> &'a Value {
        <Value as crate::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Value_oneof_kind {
    null_value(NullValue),
    number_value(f64),
    string_value(::std::string::String),
    bool_value(bool),
    struct_value(Struct),
    list_value(ListValue),
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    // .google.protobuf.NullValue null_value = 1;


    pub fn get_null_value(&self) -> NullValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(v)) => v,
            _ => NullValue::NULL_VALUE,
        }
    }
    pub fn clear_null_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_null_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_null_value(&mut self, v: NullValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(v))
    }

    // double number_value = 2;


    pub fn get_number_value(&self) -> f64 {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(v)) => v,
            _ => 0.,
        }
    }
    pub fn clear_number_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_number_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_number_value(&mut self, v: f64) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(v))
    }

    // string string_value = 3;


    pub fn get_string_value(&self) -> &str {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_string_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_kind::string_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(::std::string::String::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bool bool_value = 4;


    pub fn get_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(v)) => v,
            _ => false,
        }
    }
    pub fn clear_bool_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(v))
    }

    // .google.protobuf.Struct struct_value = 5;


    pub fn get_struct_value(&self) -> &Struct {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref v)) => v,
            _ => Struct::default_instance(),
        }
    }
    pub fn clear_struct_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_struct_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_struct_value(&mut self, v: Struct) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_struct_value(&mut self) -> &mut Struct {
        if let ::std::option::Option::Some(Value_oneof_kind::struct_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(Struct::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_struct_value(&mut self) -> Struct {
        if self.has_struct_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::struct_value(v)) => v,
                _ => panic!(),
            }
        } else {
            Struct::new()
        }
    }

    // .google.protobuf.ListValue list_value = 6;


    pub fn get_list_value(&self) -> &ListValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref v)) => v,
            _ => ListValue::default_instance(),
        }
    }
    pub fn clear_list_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_list_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_list_value(&mut self, v: ListValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_list_value(&mut self) -> &mut ListValue {
        if let ::std::option::Option::Some(Value_oneof_kind::list_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(ListValue::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list_value(&mut self) -> ListValue {
        if self.has_list_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::list_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ListValue::new()
        }
    }
}

impl crate::Message for Value {
    fn is_initialized(&self) -> bool {
        if let Some(Value_oneof_kind::struct_value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Value_oneof_kind::list_value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(is.read_enum()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(is.read_double()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(is.read_message()?));
                },
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    my_size += ::protobuf::rt::enum_size(1, v);
                },
                &Value_oneof_kind::number_value(v) => {
                    my_size += 9;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Value_oneof_kind::bool_value(v) => {
                    my_size += 2;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_kind::list_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    os.write_enum(1, v.value())?;
                },
                &Value_oneof_kind::number_value(v) => {
                    os.write_double(2, v)?;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    os.write_string(3, v)?;
                },
                &Value_oneof_kind::bool_value(v) => {
                    os.write_bool(4, v)?;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Value_oneof_kind::list_value(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static crate::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static() -> &'static crate::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<crate::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const crate::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(crate::reflect::accessor::make_singular_enum_accessor::<_, NullValue>(
                    "null_value",
                    Value::has_null_value,
                    Value::get_null_value,
                ));
                fields.push(crate::reflect::accessor::make_singular_f64_accessor::<_>(
                    "number_value",
                    Value::has_number_value,
                    Value::get_number_value,
                ));
                fields.push(crate::reflect::accessor::make_singular_string_accessor::<_>(
                    "string_value",
                    Value::has_string_value,
                    Value::get_string_value,
                ));
                fields.push(crate::reflect::accessor::make_singular_bool_accessor::<_>(
                    "bool_value",
                    Value::has_bool_value,
                    Value::get_bool_value,
                ));
                fields.push(crate::reflect::accessor::make_singular_message_accessor::<_, Struct>(
                    "struct_value",
                    Value::has_struct_value,
                    Value::get_struct_value,
                ));
                fields.push(crate::reflect::accessor::make_singular_message_accessor::<_, ListValue>(
                    "list_value",
                    Value::has_list_value,
                    Value::get_list_value,
                ));
                crate::reflect::MessageDescriptor::new::<Value>(
                    "Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Value {
        static mut instance: ::protobuf::lazy::Lazy<Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value,
        };
        unsafe {
            instance.get(Value::new)
        }
    }
}

impl crate::Clear for Value {
    fn clear(&mut self) {
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> crate::reflect::ReflectValueRef {
        crate::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ListValue {
    // message fields
    pub values: crate::RepeatedField<Value>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: crate::CachedSize,
}

impl<'a> ::std::default::Default for &'a ListValue {
    fn default() -> &'a ListValue {
        <ListValue as crate::Message>::default_instance()
    }
}

impl ListValue {
    pub fn new() -> ListValue {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Value values = 1;


    pub fn get_values(&self) -> &[Value] {
        &self.values
    }
    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: crate::RepeatedField<Value>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut crate::RepeatedField<Value> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> crate::RepeatedField<Value> {
        ::std::mem::replace(&mut self.values, crate::RepeatedField::new())
    }
}

impl crate::Message for ListValue {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        for v in &self.values {
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

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static crate::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ListValue {
        ListValue::new()
    }

    fn descriptor_static() -> &'static crate::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<crate::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const crate::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(crate::reflect::accessor::make_repeated_field_accessor::<_, crate::types::ProtobufTypeMessage<Value>>(
                    "values",
                    |m: &ListValue| { &m.values },
                    |m: &mut ListValue| { &mut m.values },
                ));
                crate::reflect::MessageDescriptor::new::<ListValue>(
                    "ListValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ListValue {
        static mut instance: ::protobuf::lazy::Lazy<ListValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListValue,
        };
        unsafe {
            instance.get(ListValue::new)
        }
    }
}

impl crate::Clear for ListValue {
    fn clear(&mut self) {
        self.values.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for ListValue {
    fn as_ref(&self) -> crate::reflect::ReflectValueRef {
        crate::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum NullValue {
    NULL_VALUE = 0,
}

impl crate::ProtobufEnum for NullValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NullValue> {
        match value {
            0 => ::std::option::Option::Some(NullValue::NULL_VALUE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NullValue] = &[
            NullValue::NULL_VALUE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static crate::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<crate::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const crate::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                crate::reflect::EnumDescriptor::new("NullValue", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NullValue {
}

impl ::std::default::Default for NullValue {
    fn default() -> Self {
        NullValue::NULL_VALUE
    }
}

impl crate::reflect::ProtobufValue for NullValue {
    fn as_ref(&self) -> crate::reflect::ReflectValueRef {
        crate::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/protobuf/struct.proto\x12\x0fgoogle.protobuf\"\x98\x01\n\
    \x06Struct\x12;\n\x06fields\x18\x01\x20\x03(\x0b2#.google.protobuf.Struc\
    t.FieldsEntryR\x06fields\x1aQ\n\x0bFieldsEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\tR\x03key\x12,\n\x05value\x18\x02\x20\x01(\x0b2\x16.google.pro\
    tobuf.ValueR\x05value:\x028\x01\"\xb2\x02\n\x05Value\x12;\n\nnull_value\
    \x18\x01\x20\x01(\x0e2\x1a.google.protobuf.NullValueH\0R\tnullValue\x12#\
    \n\x0cnumber_value\x18\x02\x20\x01(\x01H\0R\x0bnumberValue\x12#\n\x0cstr\
    ing_value\x18\x03\x20\x01(\tH\0R\x0bstringValue\x12\x1f\n\nbool_value\
    \x18\x04\x20\x01(\x08H\0R\tboolValue\x12<\n\x0cstruct_value\x18\x05\x20\
    \x01(\x0b2\x17.google.protobuf.StructH\0R\x0bstructValue\x12;\n\nlist_va\
    lue\x18\x06\x20\x01(\x0b2\x1a.google.protobuf.ListValueH\0R\tlistValueB\
    \x06\n\x04kind\";\n\tListValue\x12.\n\x06values\x18\x01\x20\x03(\x0b2\
    \x16.google.protobuf.ValueR\x06values*\x1b\n\tNullValue\x12\x0e\n\nNULL_\
    VALUE\x10\0B\x81\x01\n\x13com.google.protobufB\x0bStructProtoP\x01Z1gith\
    ub.com/golang/protobuf/ptypes/struct;structpb\xf8\x01\x01\xa2\x02\x03GPB\
    \xaa\x02\x1eGoogle.Protobuf.WellKnownTypesJ\xa8\x1d\n\x06\x12\x04\x1e\0_\
    \x01\n\xcc\x0c\n\x01\x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffe\
    rs\x20-\x20Google's\x20data\x20interchange\x20format\n\x20Copyright\x202\
    008\x20Google\x20Inc.\x20\x20All\x20rights\x20reserved.\n\x20https://dev\
    elopers.google.com/protocol-buffers/\n\n\x20Redistribution\x20and\x20use\
    \x20in\x20source\x20and\x20binary\x20forms,\x20with\x20or\x20without\n\
    \x20modification,\x20are\x20permitted\x20provided\x20that\x20the\x20foll\
    owing\x20conditions\x20are\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistr\
    ibutions\x20of\x20source\x20code\x20must\x20retain\x20the\x20above\x20co\
    pyright\n\x20notice,\x20this\x20list\x20of\x20conditions\x20and\x20the\
    \x20following\x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Redistributions\
    \x20in\x20binary\x20form\x20must\x20reproduce\x20the\x20above\n\x20copyr\
    ight\x20notice,\x20this\x20list\x20of\x20conditions\x20and\x20the\x20fol\
    lowing\x20disclaimer\n\x20in\x20the\x20documentation\x20and/or\x20other\
    \x20materials\x20provided\x20with\x20the\n\x20distribution.\n\x20\x20\
    \x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\x20Inc.\x20nor\
    \x20the\x20names\x20of\x20its\n\x20contributors\x20may\x20be\x20used\x20\
    to\x20endorse\x20or\x20promote\x20products\x20derived\x20from\n\x20this\
    \x20software\x20without\x20specific\x20prior\x20written\x20permission.\n\
    \n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HO\
    LDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\
    \x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITE\
    D\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\
    \x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\
    \x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20C\
    ONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INC\
    IDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\
    \x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\
    \x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DA\
    TA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20C\
    AUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20\
    IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\
    \x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\
    \x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVIS\
    ED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\
    \x02\x12\x03\x20\0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\
    \x03\"\0;\n\x08\n\x01\x08\x12\x03#\0\x1f\n\t\n\x02\x08\x1f\x12\x03#\0\
    \x1f\n\x08\n\x01\x08\x12\x03$\0H\n\t\n\x02\x08\x0b\x12\x03$\0H\n\x08\n\
    \x01\x08\x12\x03%\0,\n\t\n\x02\x08\x01\x12\x03%\0,\n\x08\n\x01\x08\x12\
    \x03&\0,\n\t\n\x02\x08\x08\x12\x03&\0,\n\x08\n\x01\x08\x12\x03'\0\"\n\t\
    \n\x02\x08\n\x12\x03'\0\"\n\x08\n\x01\x08\x12\x03(\0!\n\t\n\x02\x08$\x12\
    \x03(\0!\n\xb3\x03\n\x02\x04\0\x12\x043\06\x01\x1a\xa6\x03\x20`Struct`\
    \x20represents\x20a\x20structured\x20data\x20value,\x20consisting\x20of\
    \x20fields\n\x20which\x20map\x20to\x20dynamically\x20typed\x20values.\
    \x20In\x20some\x20languages,\x20`Struct`\n\x20might\x20be\x20supported\
    \x20by\x20a\x20native\x20representation.\x20For\x20example,\x20in\n\x20s\
    cripting\x20languages\x20like\x20JS\x20a\x20struct\x20is\x20represented\
    \x20as\x20an\n\x20object.\x20The\x20details\x20of\x20that\x20representat\
    ion\x20are\x20described\x20together\n\x20with\x20the\x20proto\x20support\
    \x20for\x20the\x20language.\n\n\x20The\x20JSON\x20representation\x20for\
    \x20`Struct`\x20is\x20JSON\x20object.\n\n\n\n\x03\x04\0\x01\x12\x033\x08\
    \x0e\n9\n\x04\x04\0\x02\0\x12\x035\x02\x20\x1a,\x20Unordered\x20map\x20o\
    f\x20dynamically\x20typed\x20values.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    5\x023\x10\n\x0c\n\x05\x04\0\x02\0\x06\x12\x035\x02\x14\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x035\x15\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x035\x1e\
    \x1f\n\xc3\x02\n\x02\x04\x01\x12\x04>\0N\x01\x1a\xb6\x02\x20`Value`\x20r\
    epresents\x20a\x20dynamically\x20typed\x20value\x20which\x20can\x20be\
    \x20either\n\x20null,\x20a\x20number,\x20a\x20string,\x20a\x20boolean,\
    \x20a\x20recursive\x20struct\x20value,\x20or\x20a\n\x20list\x20of\x20val\
    ues.\x20A\x20producer\x20of\x20value\x20is\x20expected\x20to\x20set\x20o\
    ne\x20of\x20that\n\x20variants,\x20absence\x20of\x20any\x20variant\x20in\
    dicates\x20an\x20error.\n\n\x20The\x20JSON\x20representation\x20for\x20`\
    Value`\x20is\x20JSON\x20value.\n\n\n\n\x03\x04\x01\x01\x12\x03>\x08\r\n\
    \"\n\x04\x04\x01\x08\0\x12\x04@\x02M\x03\x1a\x14\x20The\x20kind\x20of\
    \x20value.\n\n\x0c\n\x05\x04\x01\x08\0\x01\x12\x03@\x08\x0c\n'\n\x04\x04\
    \x01\x02\0\x12\x03B\x04\x1d\x1a\x1a\x20Represents\x20a\x20null\x20value.\
    \n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03B\x04\r\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03B\x0e\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03B\x1b\x1c\n\
    )\n\x04\x04\x01\x02\x01\x12\x03D\x04\x1c\x1a\x1c\x20Represents\x20a\x20d\
    ouble\x20value.\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03D\x04\n\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03D\x0b\x17\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03D\x1a\x1b\n)\n\x04\x04\x01\x02\x02\x12\x03F\x04\x1c\x1a\x1c\
    \x20Represents\x20a\x20string\x20value.\n\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03F\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03F\x0b\x17\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03F\x1a\x1b\n*\n\x04\x04\x01\x02\x03\
    \x12\x03H\x04\x18\x1a\x1d\x20Represents\x20a\x20boolean\x20value.\n\n\
    \x0c\n\x05\x04\x01\x02\x03\x05\x12\x03H\x04\x08\n\x0c\n\x05\x04\x01\x02\
    \x03\x01\x12\x03H\t\x13\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03H\x16\x17\
    \n-\n\x04\x04\x01\x02\x04\x12\x03J\x04\x1c\x1a\x20\x20Represents\x20a\
    \x20structured\x20value.\n\n\x0c\n\x05\x04\x01\x02\x04\x06\x12\x03J\x04\
    \n\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03J\x0b\x17\n\x0c\n\x05\x04\x01\
    \x02\x04\x03\x12\x03J\x1a\x1b\n-\n\x04\x04\x01\x02\x05\x12\x03L\x04\x1d\
    \x1a\x20\x20Represents\x20a\x20repeated\x20`Value`.\n\n\x0c\n\x05\x04\
    \x01\x02\x05\x06\x12\x03L\x04\r\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03L\
    \x0e\x18\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03L\x1b\x1c\n\xa9\x01\n\
    \x02\x05\0\x12\x04T\0W\x01\x1a\x9c\x01\x20`NullValue`\x20is\x20a\x20sing\
    leton\x20enumeration\x20to\x20represent\x20the\x20null\x20value\x20for\
    \x20the\n\x20`Value`\x20type\x20union.\n\n\x20\x20The\x20JSON\x20represe\
    ntation\x20for\x20`NullValue`\x20is\x20JSON\x20`null`.\n\n\n\n\x03\x05\0\
    \x01\x12\x03T\x05\x0e\n\x1a\n\x04\x05\0\x02\0\x12\x03V\x02\x11\x1a\r\x20\
    Null\x20value.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03V\x02\x0c\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03V\x0f\x10\n\x82\x01\n\x02\x04\x02\x12\x04\\\
    \0_\x01\x1av\x20`ListValue`\x20is\x20a\x20wrapper\x20around\x20a\x20repe\
    ated\x20field\x20of\x20values.\n\n\x20The\x20JSON\x20representation\x20f\
    or\x20`ListValue`\x20is\x20JSON\x20array.\n\n\n\n\x03\x04\x02\x01\x12\
    \x03\\\x08\x11\n:\n\x04\x04\x02\x02\0\x12\x03^\x02\x1c\x1a-\x20Repeated\
    \x20field\x20of\x20dynamically\x20typed\x20values.\n\n\x0c\n\x05\x04\x02\
    \x02\0\x04\x12\x03^\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03^\x0b\x10\
    \n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03^\x11\x17\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03^\x1a\x1bb\x06proto3\
";

static mut file_descriptor_proto_lazy: crate::lazy::Lazy<crate::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: crate::lazy::ONCE_INIT,
    ptr: 0 as *const crate::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
