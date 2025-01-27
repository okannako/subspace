// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// Copyright (C) 2021 Subspace Labs, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Verification for Subspace headers.
use crate::digests::{CompatibleDigestItem, PreDigest};
use crate::{find_pre_digest, FarmerPublicKey};
use schnorrkel::context::SigningContext;
use sp_api::HeaderT;
use sp_consensus_slots::Slot;
use sp_core::crypto::ByteArray;
use sp_runtime::{DigestItem, RuntimeAppPublic};
use subspace_archiving::archiver;
use subspace_core_primitives::{PieceIndex, Randomness, Salt, Sha256Hash, Solution};
use subspace_solving::{
    derive_global_challenge, is_local_challenge_valid, PieceDistance, SubspaceCodec,
};

/// Errors encountered by the Subspace authorship task.
#[derive(Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
pub enum VerificationError<Header: HeaderT> {
    /// No Subspace pre-runtime digest found
    #[cfg_attr(feature = "thiserror", error("No Subspace pre-runtime digest found"))]
    NoPreRuntimeDigest,
    /// Header has a bad seal
    #[cfg_attr(feature = "thiserror", error("Header {0:?} has a bad seal"))]
    HeaderBadSeal(Header::Hash),
    /// Header is unsealed
    #[cfg_attr(feature = "thiserror", error("Header {0:?} is unsealed"))]
    HeaderUnsealed(Header::Hash),
    /// Bad signature
    #[cfg_attr(feature = "thiserror", error("Bad signature on {0:?}"))]
    BadSignature(Header::Hash),
    /// Bad solution signature
    #[cfg_attr(
        feature = "thiserror",
        error("Bad solution signature on slot {0:?}: {1:?}")
    )]
    BadSolutionSignature(Slot, schnorrkel::SignatureError),
    /// Bad local challenge
    #[cfg_attr(
        feature = "thiserror",
        error("Local challenge is invalid for slot {0}: {1}")
    )]
    BadLocalChallenge(Slot, schnorrkel::SignatureError),
    /// Solution is outside of solution range
    #[cfg_attr(
        feature = "thiserror",
        error("Solution is outside of solution range for slot {0}")
    )]
    OutsideOfSolutionRange(Slot),
    /// Solution is outside of max plot size
    #[cfg_attr(
        feature = "thiserror",
        error("Solution is outside of max plot size {0}")
    )]
    OutsideOfMaxPlot(Slot),
    /// Invalid encoding of a piece
    #[cfg_attr(feature = "thiserror", error("Invalid encoding for slot {0}"))]
    InvalidEncoding(Slot),
    /// Invalid tag for salt
    #[cfg_attr(feature = "thiserror", error("Invalid tag for salt for slot {0}"))]
    InvalidTag(Slot),
}

/// A header which has been checked
pub enum CheckedHeader<H, S> {
    /// A header which has slot in the future. this is the full header (not stripped)
    /// and the slot in which it should be processed.
    Deferred(H, Slot),
    /// A header which is fully checked, including signature. This is the pre-header
    /// accompanied by the seal components.
    ///
    /// Includes the digest item that encoded the seal.
    Checked(H, S),
}

/// Subspace verification parameters
pub struct VerificationParams<'a, Header>
where
    Header: HeaderT + 'a,
{
    /// The header being verified.
    pub header: Header,
    /// The slot number of the current time.
    pub slot_now: Slot,
    /// Parameters for solution verification
    pub verify_solution_params: VerifySolutionParams<'a>,
}

/// Information from verified header
pub struct VerifiedHeaderInfo {
    /// Pre-digest
    pub pre_digest: PreDigest<FarmerPublicKey>,
    /// Seal (signature)
    pub seal: DigestItem,
}

/// Check a header has been signed by the right key. If the slot is too far in
/// the future, an error will be returned. If successful, returns the pre-header
/// and the digest item containing the seal.
///
/// The seal must be the last digest. Otherwise, the whole header is considered unsigned. This is
/// required for security and must not be changed.
///
/// This digest item will always return `Some` when used with `as_subspace_pre_digest`.
pub fn check_header<Header>(
    params: VerificationParams<Header>,
) -> Result<CheckedHeader<Header, VerifiedHeaderInfo>, VerificationError<Header>>
where
    Header: HeaderT,
{
    let VerificationParams {
        mut header,
        slot_now,
        verify_solution_params,
    } = params;

    let pre_digest =
        find_pre_digest::<Header>(&header).ok_or(VerificationError::NoPreRuntimeDigest)?;
    let slot = pre_digest.slot;

    let seal = header
        .digest_mut()
        .pop()
        .ok_or_else(|| VerificationError::HeaderUnsealed(header.hash()))?;

    let sig = seal
        .as_subspace_seal()
        .ok_or_else(|| VerificationError::HeaderBadSeal(header.hash()))?;

    // The pre-hash of the header doesn't include the seal and that's what we sign
    let pre_hash = header.hash();

    if pre_digest.slot > slot_now {
        header.digest_mut().push(seal);
        return Ok(CheckedHeader::Deferred(header, pre_digest.slot));
    }

    // Verify that block is signed properly
    if !pre_digest.solution.public_key.verify(&pre_hash, &sig) {
        return Err(VerificationError::BadSignature(pre_hash));
    }

    // Verify that solution is valid
    verify_solution(&pre_digest.solution, slot, verify_solution_params)?;

    Ok(CheckedHeader::Checked(
        header,
        VerifiedHeaderInfo { pre_digest, seal },
    ))
}

/// Check the solution signature validity.
fn check_signature(
    signing_context: &SigningContext,
    solution: &Solution<FarmerPublicKey>,
) -> Result<(), schnorrkel::SignatureError> {
    let public_key = schnorrkel::PublicKey::from_bytes(solution.public_key.as_slice())?;
    let signature = schnorrkel::Signature::from_bytes(&solution.signature)?;
    public_key.verify(signing_context.bytes(&solution.tag), &signature)
}

/// Check if the tag of a solution's piece is valid.
fn check_piece_tag<Header>(
    slot: Slot,
    salt: Salt,
    solution: &Solution<FarmerPublicKey>,
) -> Result<(), VerificationError<Header>>
where
    Header: HeaderT,
{
    if !subspace_solving::is_tag_valid(&solution.encoding, salt, solution.tag) {
        return Err(VerificationError::InvalidTag(slot));
    }

    Ok(())
}

/// Check piece validity.
///
/// If `records_root` is `None`, piece validity check will be skipped.
pub fn check_piece<Header>(
    slot: Slot,
    records_root: Sha256Hash,
    position: u64,
    record_size: u32,
    solution: &Solution<FarmerPublicKey>,
) -> Result<(), VerificationError<Header>>
where
    Header: HeaderT,
{
    let mut piece = solution.encoding;

    // Ensure piece is decodable.
    let subspace_codec = SubspaceCodec::new(&solution.public_key);
    subspace_codec
        .decode(&mut piece, solution.piece_index)
        .map_err(|_| VerificationError::InvalidEncoding(slot))?;

    if !archiver::is_piece_valid(
        &piece,
        records_root,
        position as usize,
        record_size as usize,
    ) {
        return Err(VerificationError::InvalidEncoding(slot));
    }

    Ok(())
}

/// Returns true if `solution.tag` is within the solution range.
fn is_within_solution_range(solution: &Solution<FarmerPublicKey>, solution_range: u64) -> bool {
    let solution_tag = u64::from_be_bytes(solution.tag);
    let target = u64::from_be_bytes(solution.local_challenge.derive_target());

    let distance = subspace_core_primitives::bidirectional_distance(&target, &solution_tag);

    distance <= solution_range / 2
}

/// Returns true if piece index is within farmer sector
fn is_within_max_plot(
    piece_index: PieceIndex,
    key: &FarmerPublicKey,
    total_pieces: u64,
    max_plot_size: u64,
) -> bool {
    if total_pieces < max_plot_size {
        return true;
    }
    let max_distance_one_direction = PieceDistance::MAX / total_pieces * max_plot_size / 2;
    PieceDistance::distance(&piece_index.into(), key) <= max_distance_one_direction
}

/// Parameters for checking piece validity
pub struct PieceCheckParams {
    /// Records root of segment to which piece belongs
    pub records_root: Sha256Hash,
    /// Position of the piece in the segment
    pub position: u64,
    /// Record size, system parameter
    pub record_size: u32,
    /// Max plot size in pieces, system parameter
    pub max_plot_size: u64,
    /// Total number of pieces in the whole archival history
    pub total_pieces: u64,
}

/// Parameters for solution verification
pub struct VerifySolutionParams<'a> {
    /// Global randomness
    pub global_randomness: &'a Randomness,
    /// Solution range
    pub solution_range: u64,
    /// Salt
    pub salt: Salt,
    /// Parameters for checking piece validity.
    ///
    /// If `None`, piece validity check will be skipped.
    pub piece_check_params: Option<PieceCheckParams>,
    /// Signing context for solution signature
    pub signing_context: &'a SigningContext,
}

/// Solution verification
pub fn verify_solution<Header>(
    solution: &Solution<FarmerPublicKey>,
    slot: Slot,
    params: VerifySolutionParams,
) -> Result<(), VerificationError<Header>>
where
    Header: HeaderT,
{
    let VerifySolutionParams {
        global_randomness,
        solution_range,
        salt,
        piece_check_params,
        signing_context,
    } = params;

    if let Err(error) = is_local_challenge_valid(
        derive_global_challenge(global_randomness, slot),
        &solution.local_challenge,
        &solution.public_key,
    ) {
        return Err(VerificationError::BadLocalChallenge(slot, error));
    }

    if !is_within_solution_range(solution, solution_range) {
        return Err(VerificationError::OutsideOfSolutionRange(slot));
    }

    check_signature(signing_context, solution)
        .map_err(|e| VerificationError::BadSolutionSignature(slot, e))?;

    check_piece_tag(slot, salt, solution)?;

    if let Some(PieceCheckParams {
        records_root,
        position,
        record_size,
        max_plot_size,
        total_pieces,
    }) = piece_check_params
    {
        if !is_within_max_plot(
            solution.piece_index,
            &solution.public_key,
            total_pieces,
            max_plot_size,
        ) {
            return Err(VerificationError::OutsideOfMaxPlot(slot));
        }

        check_piece(slot, records_root, position, record_size, solution)?;
    }

    Ok(())
}
