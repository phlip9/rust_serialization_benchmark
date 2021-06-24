use crate::{generate_btreemap, generate_oneof, generate_vec, Generate};
use rand::Rng;
use serde::{
    de::{self, Error},
    ser, Deserialize, Serialize,
};
use std::{collections::BTreeMap, convert::TryFrom, fmt};

fn gen<T: Generate, R: Rng>(rng: &mut R) -> T {
    T::generate(rng)
}

////////////////
// Data Types //
////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ConsensusMsg {
    BlockRetrievalRequest(Box<BlockRetrievalRequest>),
    BlockRetrievalResponse(Box<BlockRetrievalResponse>),
    EpochRetrievalRequest(Box<EpochRetrievalRequest>),
    ProposalMsg(Box<ProposalMsg>),
    SyncInfo(Box<SyncInfo>),
    EpochChangeProof(Box<EpochChangeProof>),
    VoteMsg(Box<VoteMsg>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockRetrievalRequest {
    block_id: HashValue,
    num_blocks: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockRetrievalResponse {
    status: BlockRetrievalStatus,
    blocks: Vec<Block>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EpochRetrievalRequest {
    start_epoch: u64,
    end_epoch: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProposalMsg {
    proposal: Block,
    sync_info: SyncInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyncInfo {
    highest_quorum_cert: QuorumCert,
    highest_commit_cert: Option<QuorumCert>,
    highest_timeout_cert: Option<TimeoutCertificate>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EpochChangeProof {
    ledger_info_with_sigs: Vec<LedgerInfoWithSignatures>,
    more: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VoteMsg {
    vote: Vote,
    sync_info: SyncInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Vote {
    vote_data: VoteData,
    author: AccountAddress,
    ledger_info: LedgerInfo,
    signature: Ed25519Signature,
    timeout_signature: Option<Ed25519Signature>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimeoutCertificate {
    epoch: u64,
    round: u64,
    signatures: BTreeMap<AccountAddress, Ed25519Signature>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BlockRetrievalStatus {
    Succeeded,
    IdNotFound,
    NotEnoughBlocks,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Block {
    block_data: BlockData,
    signature: Option<Ed25519Signature>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BlockData {
    epoch: u64,
    round: u64,
    timestamp_usecs: u64,
    quorum_cert: QuorumCert,
    block_type: BlockType,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum BlockType {
    Proposal {
        payload: Vec<SignedTransaction>,
        author: AccountAddress,
    },
    NilBlock,
    Genesis,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuorumCert {
    vote_data: VoteData,
    signed_ledger_info: LedgerInfoWithSignatures,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VoteData {
    proposed: BlockInfo,
    parent: BlockInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BlockInfo {
    epoch: u64,
    round: u64,
    id: HashValue,
    executed_state_id: HashValue,
    version: u64,
    timestamp_usecs: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerInfo {
    commit_info: BlockInfo,
    consensus_data_hash: HashValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LedgerInfoWithSignatures {
    V0(LedgerInfoWithV0),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerInfoWithV0 {
    ledger_info: LedgerInfo,
    signatures: BTreeMap<AccountAddress, Ed25519Signature>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    raw_txn: RawTransaction,
    authenticator: TransactionAuthenticator,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawTransaction {
    sender: AccountAddress,
    sequence_number: u64,
    payload: TransactionPayload,
    max_gas_amount: u64,
    gas_unit_price: u64,
    gas_currency_code: String,
    expiration_timestamp_secs: u64,
    chain_id: ChainId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionPayload {
    WriteSet,
    Script,
    Module,
    ScriptFunction(ScriptFunction),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptFunction {
    module: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag>,
    args: Vec<Argument>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleId {
    address: AccountAddress,
    name: Identifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identifier(Box<str>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TypeTag {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<TypeTag>),
    // Struct(StructTag),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Argument(#[serde(with = "serde_bytes")] Vec<u8>);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ChainId(u8);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionAuthenticator {
    Ed25519 {
        public_key: HashValue,
        signature: Ed25519Signature,
    },
}

///////////////
// HashValue //
///////////////

#[derive(Clone, Copy, Debug)]
pub struct HashValue([u8; HashValue::LENGTH]);

impl HashValue {
    pub const LENGTH: usize = 32;

    pub const fn new(hash: [u8; HashValue::LENGTH]) -> Self {
        HashValue(hash)
    }

    pub fn from_slice<T: AsRef<[u8]>>(bytes: T) -> Result<Self, HashValueParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| HashValueParseError)
            .map(Self::new)
    }
}

impl ser::Serialize for HashValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            hex::encode(&self.0[..]).serialize(serializer)
        } else {
            serializer.serialize_newtype_struct("HashValue", serde_bytes::Bytes::new(&self.0[..]))
        }
    }
}

impl<'de> de::Deserialize<'de> for HashValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            let mut buf = [0; Self::LENGTH];
            hex::decode_to_slice(s, &mut buf[..]).map_err(D::Error::custom)?;
            Ok(Self::new(buf))
        } else {
            #[derive(::serde::Deserialize)]
            #[serde(rename = "HashValue")]
            struct Value<'a>(&'a [u8]);

            let value = Value::deserialize(deserializer)?;
            Self::from_slice(value.0).map_err(<D::Error as de::Error>::custom)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HashValueParseError;

impl fmt::Display for HashValueParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse HashValue")
    }
}

impl std::error::Error for HashValueParseError {}

////////////////////
// AccountAddress //
////////////////////

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct AccountAddress([u8; Self::LENGTH]);

impl AccountAddress {
    pub const LENGTH: usize = 16;

    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }
}

impl ser::Serialize for AccountAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            hex::encode(&self.0[..]).serialize(serializer)
        } else {
            serializer.serialize_newtype_struct("AccountAddress", &self.0)
        }
    }
}

impl<'de> de::Deserialize<'de> for AccountAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            let mut buf = [0; Self::LENGTH];
            hex::decode_to_slice(s, &mut buf[..]).map_err(D::Error::custom)?;
            Ok(Self::new(buf))
        } else {
            #[derive(::serde::Deserialize)]
            #[serde(rename = "AccountAddress")]
            struct Value([u8; AccountAddress::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(Self::new(value.0))
        }
    }
}

//////////////////////
// Ed25519Signature //
//////////////////////

#[derive(Clone, Copy, Debug)]
pub struct Ed25519Signature([u8; Self::LENGTH]);

impl Ed25519Signature {
    pub const LENGTH: usize = 64;

    pub const fn new(sig: [u8; Self::LENGTH]) -> Self {
        Self(sig)
    }

    pub fn from_slice<T: AsRef<[u8]>>(bytes: T) -> Result<Self, HashValueParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| HashValueParseError)
            .map(Self::new)
    }
}

impl ser::Serialize for Ed25519Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            hex::encode(&self.0[..]).serialize(serializer)
        } else {
            serializer
                .serialize_newtype_struct("Ed25519Signature", serde_bytes::Bytes::new(&self.0[..]))
        }
    }
}

impl<'de> de::Deserialize<'de> for Ed25519Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            let mut buf = [0; Self::LENGTH];
            hex::decode_to_slice(s, &mut buf[..]).map_err(D::Error::custom)?;
            Ok(Self::new(buf))
        } else {
            #[derive(::serde::Deserialize)]
            #[serde(rename = "Ed25519Signature")]
            struct Value<'a>(&'a [u8]);

            let value = Value::deserialize(deserializer)?;
            Self::from_slice(value.0).map_err(<D::Error as de::Error>::custom)
        }
    }
}

////////////////
// Generators //
////////////////

impl Generate for ConsensusMsg {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::ProposalMsg(Box::new(gen(rng)))
    }
}

impl Generate for ProposalMsg {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            proposal: gen(rng),
            sync_info: gen(rng),
        }
    }
}

impl Generate for Block {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            block_data: gen(rng),
            signature: Some(gen(rng)),
        }
    }
}

impl Generate for BlockData {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            epoch: gen(rng),
            round: gen(rng),
            timestamp_usecs: gen(rng),
            quorum_cert: gen(rng),
            block_type: gen(rng),
        }
    }
}

impl Generate for SyncInfo {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            highest_quorum_cert: gen(rng),
            highest_commit_cert: Some(gen(rng)),
            highest_timeout_cert: Some(gen(rng)),
        }
    }
}

impl Generate for TimeoutCertificate {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            epoch: gen(rng),
            round: gen(rng),
            signatures: generate_btreemap(rng, 20..30),
        }
    }
}

impl Generate for QuorumCert {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            vote_data: gen(rng),
            signed_ledger_info: gen(rng),
        }
    }
}

impl Generate for VoteData {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            proposed: gen(rng),
            parent: gen(rng),
        }
    }
}

impl Generate for BlockInfo {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            epoch: gen(rng),
            round: gen(rng),
            id: gen(rng),
            executed_state_id: gen(rng),
            version: gen(rng),
            timestamp_usecs: gen(rng),
        }
    }
}

impl Generate for BlockType {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::Proposal {
            payload: generate_vec(rng, 50..100),
            author: gen(rng),
        }
    }
}

impl Generate for LedgerInfoWithSignatures {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::V0(gen(rng))
    }
}

impl Generate for LedgerInfoWithV0 {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            ledger_info: gen(rng),
            signatures: generate_btreemap(rng, 20..30),
        }
    }
}

impl Generate for LedgerInfo {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            commit_info: gen(rng),
            consensus_data_hash: gen(rng),
        }
    }
}

impl Generate for SignedTransaction {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            raw_txn: gen(rng),
            authenticator: gen(rng),
        }
    }
}

impl Generate for RawTransaction {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            sender: gen(rng),
            sequence_number: gen(rng),
            payload: gen(rng),
            max_gas_amount: gen(rng),
            gas_unit_price: gen(rng),
            gas_currency_code: "XUS".to_string(),
            expiration_timestamp_secs: gen(rng),
            chain_id: ChainId(gen(rng)),
        }
    }
}

impl Generate for TransactionPayload {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::ScriptFunction(gen(rng))
    }
}

impl Generate for ScriptFunction {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            module: gen(rng),
            function: gen(rng),
            ty_args: generate_vec(rng, 0..6),
            args: generate_vec(rng, 0..6),
        }
    }
}

impl Generate for ModuleId {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            address: gen(rng),
            name: gen(rng),
        }
    }
}

impl Generate for Identifier {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDENTS: &[&str] = &[
            "peer_to_peer_with_metadata",
            "create_child_vasp_account",
            "rotate_authentication_key",
            "rotate_authentication_key_with_nonce",
            "rotate_authentication_key_with_nonce_admin",
            "rotate_authentication_key_with_recovery_address",
            "rotate_dual_attestation_info",
        ];
        let ident = *generate_oneof(rng, &IDENTS[..]);
        Self(ident.into())
    }
}

impl Generate for TypeTag {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const TYPES: &[TypeTag] = &[
            TypeTag::Bool,
            TypeTag::U8,
            TypeTag::U64,
            TypeTag::U128,
            TypeTag::Address,
            TypeTag::Signer,
        ];
        fn gen_base_type<R: Rng>(rng: &mut R) -> TypeTag {
            generate_oneof(rng, &TYPES[..]).clone()
        }

        let base_type = gen_base_type(rng);

        if rng.gen_bool(0.25) {
            TypeTag::Vector(Box::new(base_type))
        } else {
            base_type
        }
    }
}

impl Generate for Argument {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self(generate_vec(rng, 1..64))
    }
}

impl Generate for TransactionAuthenticator {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::Ed25519 {
            public_key: gen(rng),
            signature: gen(rng),
        }
    }
}

impl Generate for HashValue {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::new([gen(rng); Self::LENGTH])
    }
}

impl Generate for AccountAddress {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::new([gen(rng); Self::LENGTH])
    }
}

impl Generate for Ed25519Signature {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::new([gen(rng); Self::LENGTH])
    }
}
