use std::collections::HashSet;

use crate::structure::{EntityId, GuidPrefix, ProtocolVersion, VendorId};

pub struct SubMessage {
    header: Header,
    elements: Vec<Element>,
}

pub enum SubMessageKind {
    RtpsHe,
    Data,
    Gap,
    Heartbeat,
    Acknack,
    Pad,
    InfoTs,
    InfoReply,
    InfoDst,
    InfoSrc,
    DataFrag,
    NackFrag,
    HeartbeatFrag,
}

pub struct Header;

/// Each RTPS [`SubMessage`] is built from a set of predefined atomic building
/// blocks called [`ELement`]s.
///
/// See [Section 8.3.5](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=46) of the specification.
pub enum Element {
    /// A Submessage element used to contain [`GuidPrefix`].
    GuidPrefix { value: GuidPrefix },

    /// A SubmessageElement to contain [`EntityId`].
    EntityId { value: EntityId },

    /// The VendorId identifies the vendor of the middleware implementing the
    /// RTPS protocol and allows this vendor to add specific extensions to the
    /// protocol. The vendor ID does not refer to the vendor of the device or
    /// product that contains DDS middleware.
    VendorId { value: VendorId },

    /// The ProtocolVersion defines the version of the RTPS protocol.
    ProtocolVersion { value: ProtocolVersion },

    /// A [`SequenceNumber`] is a 64-bit signed integer, that can take values in
    /// the range: -2^63 <= N <= 2^63-1. The
    /// selection of 64 bits as the representation of a s[`SequenceNumber`]
    /// ensures the [`SequenceNumber`]s never wrap. [`SequenceNumber`]s
    /// begin at 1.
    SequenceNumber { value: SequenceNumber },

    /// SequenceNumberSet [`SubMessage`] [`Element`]s are used as
    /// parts of several messages to provide binary information about
    /// individual sequence numbers within a range. The sequence numbers
    /// represented in the SequenceNumberSet are limited to belong  to
    /// an  interval  with  a range  no  bigger  than  256.  In  other
    /// words,  a valid SequenceNumberSet  must  verify  that:
    ///
    ///     maximum(SequenceNumberSet) - minimum(SequenceNumberSet) < 256
    ///     minimum(SequenceNumberSet) >= 1
    ///
    /// The above restriction allows SequenceNumberSet to be represented in
    /// an efficient and compact way using bitmaps. SequenceNumberSet
    /// [`SubMessage`] [`Element`]s are used for example to selectively request
    /// re-sending of a set of sequence numbers.
    SequenceNumberSet {
        base: SequenceNumber,
        set: HashSet<SequenceNumber>,
    },

    /// A fragment number is a 32-bit unsigned integer and is used by
    /// Submessages to identify a particular fragment in fragmented serialized
    /// data. Fragment numbers start at 1.
    FragmentNumber { value: FragmentNumber },

    /// FragmentNumberSet [`SubMessage`] [`Element`]s are used to provide binary
    /// information about individual fragment numbers within a range. The
    /// fragment numbers represented in the FragmentNumberSet are limited to
    /// belongto an interval with a range no bigger than 256. In other words, a
    /// valid FragmentNumberSet must verify that:
    ///
    ///     maximum(FragmentNumberSet) - minimum(FragmentNumberSet) < 256
    ///     minimum(FragmentNumberSet) >= 1
    ///
    /// The above restriction allows FragmentNumberSet to be represented in an
    /// efficient and compact way using bitmaps. FragmentNumberSet
    /// [`SubMessage`] [`Element`]s are used for example to selectively request
    /// re-sending of a set of fragments.
    FragmentNumberSet {
        base: FragmentNumber,
        set: HashSet<FragmentNumber>,
    },

    /// Timestamp is used to represent time. The representation should be
    /// capable of having a resolution of nano-seconds or better.
    TimeStamp { value: Time },
}

/// See [Section 8.3.5.4](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=39) of the specification.
pub enum SequenceNumber {
    Known(i64),
    Unknown,
}

/// See [Section 8.3.5.6](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=40) of the specification.
pub type FragmentNumber = u32;

/// See [Section 8.3.5.8](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=41) of the specification.
pub enum Time {
    Value(std::time::SystemTime),
    Zero,
    Invalid,
    Infinite,
}
