package chain

import "github.com/centrifuge/go-substrate-rpc-client/v4/types"

// Pallert
const (
	State_Sminer      = "Sminer"
	State_SegmentBook = "SegmentBook"
	State_FileMap     = "FileMap"
	State_FileBank    = "FileBank"
)

// Chain state
const (
	Sminer_MinerItems          = "MinerItems"
	Sminer_MinerDetails        = "MinerDetails"
	SegmentBook_MinerHoldSlice = "MinerHoldSlice"
	SegmentBook_ChallengeMap   = "ChallengeMap"
	FileMap_SchedulerPuk       = "SchedulerPuk"
	FileBank_FillerMap         = "FillerMap"
	FileMap_SchedulerInfo      = "SchedulerMap"
	FileBank_InvalidFile       = "InvalidFile"
	Sminer_MinerLockIn         = "MinerLockIn"
)

// Extrinsics
const (
	ChainTx_Sminer_Register          = "Sminer.regnstk"
	ChainTx_SegmentBook_IntentSubmit = "SegmentBook.intent_submit"
	ChainTx_Sminer_ExitMining        = "Sminer.exit_miner"
	ChainTx_Sminer_Withdraw          = "Sminer.withdraw"
	ChainTx_Sminer_UpdateIp          = "Sminer.update_ip"
	ChainTx_Sminer_UpdateBeneficiary = "Sminer.update_beneficiary"
	ChainTx_Sminer_Increase          = "Sminer.increase_collateral"
	SegmentBook_SubmitProve          = "SegmentBook.submit_challenge_prove"
	FileBank_ClearInvalidFile        = "FileBank.clear_invalid_file"
	FileBank_ClearFiller             = "FileBank.clear_all_filler"
)

type FileHash [64]types.U8
type FileBlockId [68]types.U8

// Storage Miner Information Structure
type MinerInfo struct {
	PeerId      types.U64
	IncomeAcc   types.AccountID
	Ip          Ipv4Type
	Collaterals types.U128
	State       types.Bytes
	Power       types.U128
	Space       types.U128
	RewardInfo  RewardInfo
}

type RewardInfo struct {
	Total       types.U128
	Received    types.U128
	NotReceived types.U128
}

// Scheduling Node Information Structure
type SchedulerInfo struct {
	Ip              Ipv4Type
	Stash_user      types.AccountID
	Controller_user types.AccountID
}

// Challenge information structure
type ChallengesInfo struct {
	File_size  types.U64
	File_type  types.U8
	Block_list types.Bytes
	File_id    FileHash
	Shard_id   FileBlockId
	Random     []types.Bytes
}

// Scheduling node public key information structure
type Chain_SchedulerPuk struct {
	Spk           [128]types.U8
	Shared_params types.Bytes
	Shared_g      [128]types.U8
}

// Proof information structure
type ProveInfo struct {
	FileId   FileHash
	MinerAcc types.AccountID
	Cinfo    ChallengesInfo
	Mu       []types.Bytes
	Sigma    types.Bytes
	Name     types.Bytes
	U        []types.Bytes
}

type Ipv4Type_Query struct {
	Placeholder types.U8 //
	Index       types.U8
	Value       [4]types.U8
	Port        types.U16
}

type IpAddress struct {
	IPv4 Ipv4Type
	IPv6 Ipv6Type
}
type Ipv4Type struct {
	Index types.U8
	Value [4]types.U8
	Port  types.U16
}
type Ipv6Type struct {
	Index types.U8
	Value [8]types.U16
	Port  types.U16
}

const (
	ERR_Failed  = "Failed"
	ERR_Timeout = "Timeout"
	ERR_Empty   = "Empty"
)
