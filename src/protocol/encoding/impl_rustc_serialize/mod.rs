//Stolen from https://github.com/TyOverby/bincode
//This has been imported for change mainly because of annoying differences between what Kafka expects and what bincode does

extern crate bincode;

use std::io::{ Write, Read };
use rustc_serialize::{ Encodable, Decodable };
use bincode::SizeLimit;
use bincode::rustc_serialize::SizeChecker;

pub use super::usize_as_u32;
pub use self::writer::{ EncoderWriter, EncodingResult, EncodingError };
pub use self::reader::{ DecoderReader, DecodingResult, DecodingError };

mod reader;
mod writer;

/// Encodes an encodable object into a `Vec` of bytes.
///
/// If the encoding would take more bytes than allowed by `size_limit`,
/// an error is returned.
pub fn encode<T: Encodable>(t: &T) -> EncodingResult<Vec<u8>> {
    // Since we are putting values directly into a vector, we can do size
    // computation out here and pre-allocate a buffer of *exactly*
    // the right size.
    let mut w = vec![];

    match encode_into(t, &mut w, SizeLimit::Infinite) {
        Ok(()) => Ok(w),
        Err(e) => Err(e)
    }
}

/// Decodes a slice of bytes into an object.
///
/// This method does not have a size-limit because if you already have the bytes
/// in memory, then you don't gain anything by having a limiter.
pub fn decode<T: Decodable>(b: &[u8]) -> DecodingResult<T> {
    let mut b = b;
    decode_from(&mut b, SizeLimit::Infinite)
}

/// Encodes an object directly into a `Writer`.
///
/// If the encoding would take more bytes than allowed by `size_limit`, an error
/// is returned and *no bytes* will be written into the `Writer`.
///
/// If this returns an `EncodingError` (other than SizeLimit), assume that the
/// writer is in an invalid state, as writing could bail out in the middle of
/// encoding.
pub fn encode_into<T: Encodable, W: Write>(t: &T, w: &mut W, size_limit: SizeLimit) -> EncodingResult<()> {
    try!(match size_limit {
        SizeLimit::Infinite => Ok(()),
        SizeLimit::Bounded(x) => {
            let mut size_checker = SizeChecker::new(x);
            t.encode(&mut size_checker)
        }
    });

    t.encode(&mut writer::EncoderWriter::new(w))
}

/// Decoes an object directly from a `Buffer`ed Reader.
///
/// If the provided `SizeLimit` is reached, the decode will bail immediately.
/// A SizeLimit can help prevent an attacker from flooding your server with
/// a neverending stream of values that runs your server out of memory.
///
/// If this returns an `DecodingError`, assume that the buffer that you passed
/// in is in an invalid state, as the error could be returned during any point
/// in the reading.
pub fn decode_from<R: Read, T: Decodable>(r: &mut R, size_limit: SizeLimit) -> DecodingResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r, size_limit))
}

/// Returns the size that an object would be if encoded using bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub fn encoded_size<T: Encodable>(t: &T) -> u64 {
    use std::u64::MAX;
    let mut size_checker = SizeChecker::new(MAX);
    t.encode(&mut size_checker).ok();
    size_checker.written
}