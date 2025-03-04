// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `types/media_packet.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MediaPacket)
pub struct MediaPacket {
    // message fields
    // @@protoc_insertion_point(field:MediaPacket.media_type)
    pub media_type: ::protobuf::EnumOrUnknown<media_packet::MediaType>,
    // @@protoc_insertion_point(field:MediaPacket.email)
    pub email: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:MediaPacket.frame_type)
    pub frame_type: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.timestamp)
    pub timestamp: f64,
    // @@protoc_insertion_point(field:MediaPacket.duration)
    pub duration: f64,
    // @@protoc_insertion_point(field:MediaPacket.audio_metadata)
    pub audio_metadata: ::protobuf::MessageField<AudioMetadata>,
    // special fields
    // @@protoc_insertion_point(special_field:MediaPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MediaPacket {
    fn default() -> &'a MediaPacket {
        <MediaPacket as ::protobuf::Message>::default_instance()
    }
}

impl MediaPacket {
    pub fn new() -> MediaPacket {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "media_type",
            |m: &MediaPacket| { &m.media_type },
            |m: &mut MediaPacket| { &mut m.media_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "email",
            |m: &MediaPacket| { &m.email },
            |m: &mut MediaPacket| { &mut m.email },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &MediaPacket| { &m.data },
            |m: &mut MediaPacket| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "frame_type",
            |m: &MediaPacket| { &m.frame_type },
            |m: &mut MediaPacket| { &mut m.frame_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "timestamp",
            |m: &MediaPacket| { &m.timestamp },
            |m: &mut MediaPacket| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "duration",
            |m: &MediaPacket| { &m.duration },
            |m: &mut MediaPacket| { &mut m.duration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, AudioMetadata>(
            "audio_metadata",
            |m: &MediaPacket| { &m.audio_metadata },
            |m: &mut MediaPacket| { &mut m.audio_metadata },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MediaPacket>(
            "MediaPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MediaPacket {
    const NAME: &'static str = "MediaPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.media_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.email = is.read_string()?;
                },
                26 => {
                    self.data = is.read_bytes()?;
                },
                34 => {
                    self.frame_type = is.read_string()?;
                },
                41 => {
                    self.timestamp = is.read_double()?;
                },
                49 => {
                    self.duration = is.read_double()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.audio_metadata)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.media_type != ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO) {
            my_size += ::protobuf::rt::int32_size(1, self.media_type.value());
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.email);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        if !self.frame_type.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.frame_type);
        }
        if self.timestamp != 0. {
            my_size += 1 + 8;
        }
        if self.duration != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.audio_metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.media_type != ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.media_type))?;
        }
        if !self.email.is_empty() {
            os.write_string(2, &self.email)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
        }
        if !self.frame_type.is_empty() {
            os.write_string(4, &self.frame_type)?;
        }
        if self.timestamp != 0. {
            os.write_double(5, self.timestamp)?;
        }
        if self.duration != 0. {
            os.write_double(6, self.duration)?;
        }
        if let Some(v) = self.audio_metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MediaPacket {
        MediaPacket::new()
    }

    fn clear(&mut self) {
        self.media_type = ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO);
        self.email.clear();
        self.data.clear();
        self.frame_type.clear();
        self.timestamp = 0.;
        self.duration = 0.;
        self.audio_metadata.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MediaPacket {
        static instance: MediaPacket = MediaPacket {
            media_type: ::protobuf::EnumOrUnknown::from_i32(0),
            email: ::std::string::String::new(),
            data: ::std::vec::Vec::new(),
            frame_type: ::std::string::String::new(),
            timestamp: 0.,
            duration: 0.,
            audio_metadata: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MediaPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MediaPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MediaPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MediaPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MediaPacket`
pub mod media_packet {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MediaPacket.MediaType)
    pub enum MediaType {
        // @@protoc_insertion_point(enum_value:MediaPacket.MediaType.VIDEO)
        VIDEO = 0,
        // @@protoc_insertion_point(enum_value:MediaPacket.MediaType.AUDIO)
        AUDIO = 1,
    }

    impl ::protobuf::Enum for MediaType {
        const NAME: &'static str = "MediaType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<MediaType> {
            match value {
                0 => ::std::option::Option::Some(MediaType::VIDEO),
                1 => ::std::option::Option::Some(MediaType::AUDIO),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [MediaType] = &[
            MediaType::VIDEO,
            MediaType::AUDIO,
        ];
    }

    impl ::protobuf::EnumFull for MediaType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("MediaPacket.MediaType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for MediaType {
        fn default() -> Self {
            MediaType::VIDEO
        }
    }

    impl MediaType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MediaType>("MediaPacket.MediaType")
        }
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:AudioMetadata)
pub struct AudioMetadata {
    // message fields
    // @@protoc_insertion_point(field:AudioMetadata.audio_format)
    pub audio_format: ::std::string::String,
    // @@protoc_insertion_point(field:AudioMetadata.audio_number_of_channels)
    pub audio_number_of_channels: u32,
    // @@protoc_insertion_point(field:AudioMetadata.audio_number_of_frames)
    pub audio_number_of_frames: u32,
    // @@protoc_insertion_point(field:AudioMetadata.audio_sample_rate)
    pub audio_sample_rate: f32,
    // special fields
    // @@protoc_insertion_point(special_field:AudioMetadata.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AudioMetadata {
    fn default() -> &'a AudioMetadata {
        <AudioMetadata as ::protobuf::Message>::default_instance()
    }
}

impl AudioMetadata {
    pub fn new() -> AudioMetadata {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_format",
            |m: &AudioMetadata| { &m.audio_format },
            |m: &mut AudioMetadata| { &mut m.audio_format },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_number_of_channels",
            |m: &AudioMetadata| { &m.audio_number_of_channels },
            |m: &mut AudioMetadata| { &mut m.audio_number_of_channels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_number_of_frames",
            |m: &AudioMetadata| { &m.audio_number_of_frames },
            |m: &mut AudioMetadata| { &mut m.audio_number_of_frames },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_sample_rate",
            |m: &AudioMetadata| { &m.audio_sample_rate },
            |m: &mut AudioMetadata| { &mut m.audio_sample_rate },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AudioMetadata>(
            "AudioMetadata",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AudioMetadata {
    const NAME: &'static str = "AudioMetadata";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.audio_format = is.read_string()?;
                },
                16 => {
                    self.audio_number_of_channels = is.read_uint32()?;
                },
                24 => {
                    self.audio_number_of_frames = is.read_uint32()?;
                },
                37 => {
                    self.audio_sample_rate = is.read_float()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.audio_format.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.audio_format);
        }
        if self.audio_number_of_channels != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.audio_number_of_channels);
        }
        if self.audio_number_of_frames != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.audio_number_of_frames);
        }
        if self.audio_sample_rate != 0. {
            my_size += 1 + 4;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.audio_format.is_empty() {
            os.write_string(1, &self.audio_format)?;
        }
        if self.audio_number_of_channels != 0 {
            os.write_uint32(2, self.audio_number_of_channels)?;
        }
        if self.audio_number_of_frames != 0 {
            os.write_uint32(3, self.audio_number_of_frames)?;
        }
        if self.audio_sample_rate != 0. {
            os.write_float(4, self.audio_sample_rate)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AudioMetadata {
        AudioMetadata::new()
    }

    fn clear(&mut self) {
        self.audio_format.clear();
        self.audio_number_of_channels = 0;
        self.audio_number_of_frames = 0;
        self.audio_sample_rate = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AudioMetadata {
        static instance: AudioMetadata = AudioMetadata {
            audio_format: ::std::string::String::new(),
            audio_number_of_channels: 0,
            audio_number_of_frames: 0,
            audio_sample_rate: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AudioMetadata {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AudioMetadata").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AudioMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AudioMetadata {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18types/media_packet.proto\"\xa1\x02\n\x0bMediaPacket\x125\n\nmedia_\
    type\x18\x01\x20\x01(\x0e2\x16.MediaPacket.MediaTypeR\tmediaType\x12\x14\
    \n\x05email\x18\x02\x20\x01(\tR\x05email\x12\x12\n\x04data\x18\x03\x20\
    \x01(\x0cR\x04data\x12\x1d\n\nframe_type\x18\x04\x20\x01(\tR\tframeType\
    \x12\x1c\n\ttimestamp\x18\x05\x20\x01(\x01R\ttimestamp\x12\x1a\n\x08dura\
    tion\x18\x06\x20\x01(\x01R\x08duration\x125\n\x0eaudio_metadata\x18\x07\
    \x20\x01(\x0b2\x0e.AudioMetadataR\raudioMetadata\"!\n\tMediaType\x12\t\n\
    \x05VIDEO\x10\0\x12\t\n\x05AUDIO\x10\x01\"\xcc\x01\n\rAudioMetadata\x12!\
    \n\x0caudio_format\x18\x01\x20\x01(\tR\x0baudioFormat\x127\n\x18audio_nu\
    mber_of_channels\x18\x02\x20\x01(\rR\x15audioNumberOfChannels\x123\n\x16\
    audio_number_of_frames\x18\x03\x20\x01(\rR\x13audioNumberOfFrames\x12*\n\
    \x11audio_sample_rate\x18\x04\x20\x01(\x02R\x0faudioSampleRateJ\x99\x06\
    \n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\
    \0\x12\x04\x02\0\x0e\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x13\n\x0c\n\
    \x04\x04\0\x04\0\x12\x04\x03\x02\x06\x03\n\x0c\n\x05\x04\0\x04\0\x01\x12\
    \x03\x03\x07\x10\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x04\x04\x0e\n\x0e\n\
    \x07\x04\0\x04\0\x02\0\x01\x12\x03\x04\x04\t\n\x0e\n\x07\x04\0\x04\0\x02\
    \0\x02\x12\x03\x04\x0c\r\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03\x05\x04\
    \x0e\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x05\x04\t\n\x0e\n\x07\
    \x04\0\x04\0\x02\x01\x02\x12\x03\x05\x0c\r\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x07\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x07\x02\x0b\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x07\x0c\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x07\x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x02\x13\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x08\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x08\x11\x12\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\t\x02\x11\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\t\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\t\x08\x0c\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\t\x0f\x10\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\n\x02\x18\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\n\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03\n\t\x13\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\n\x16\x17\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x0b\x02\x17\n\x0c\n\x05\
    \x04\0\x02\x04\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\x0b\t\x12\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0b\x15\x16\n\x0b\n\
    \x04\x04\0\x02\x05\x12\x03\x0c\x02\x16\n\x0c\n\x05\x04\0\x02\x05\x05\x12\
    \x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0c\t\x11\n\x0c\n\
    \x05\x04\0\x02\x05\x03\x12\x03\x0c\x14\x15\n\x0b\n\x04\x04\0\x02\x06\x12\
    \x03\r\x02#\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\r\x02\x0f\n\x0c\n\x05\
    \x04\0\x02\x06\x01\x12\x03\r\x10\x1e\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03\r!\"\n\n\n\x02\x04\x01\x12\x04\x10\0\x15\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x10\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x11\x02\x1a\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03\x11\x02\x08\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x11\t\x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\x18\x19\
    \n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x12\x02&\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x12\
    \t!\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x12$%\n\x0b\n\x04\x04\x01\
    \x02\x02\x12\x03\x13\x02$\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x13\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x13\t\x1f\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03\x13\"#\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\
    \x14\x02\x1e\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x14\x02\x07\n\x0c\n\
    \x05\x04\x01\x02\x03\x01\x12\x03\x14\x08\x19\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03\x14\x1c\x1db\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MediaPacket::generated_message_descriptor_data());
            messages.push(AudioMetadata::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(media_packet::MediaType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
