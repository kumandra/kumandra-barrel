package configs

import "time"

// type and version
const Version = "Kumandra-Barrel v0.1.1"

// return code
const (
	//success
	Code_200 = 200
	//bad request
	Code_400 = 400
	//forbidden
	Code_403 = 403
	//not found
	Code_404 = 404
	//server internal error
	Code_500 = 500
	//The block was produced but the event was not resolved
	Code_600 = 600
)

const (
	SIZE_1KiB          = 1024
	SIZE_1MiB          = SIZE_1KiB * 1024 // 1MB
	SIZE_1GiB          = SIZE_1MiB * 1024
	FillerSize         = 8 * SIZE_1MiB
	TimeToWaitEvents_S = 20             //The time to wait for the event, in seconds
	TokenAccuracy      = "000000000000" //Unit precision of Kumandra coins
	ExitColling        = 28800          //blocks
	BlockSize          = 1024 * 1024    //1MB
	ScanBlockSize      = 512 * 1024     //512KB
	// the time to wait for the event, in seconds
	TimeToWaitEvents = time.Duration(time.Second * 15)
)

const (
	HELP_common = `Please check with the following help information:
    1.Check if the wallet balance is sufficient
    2.Block hash:`
	HELP_register = `    3.Check the Sminer_Registered transaction event result in the block hash above:
        If system.ExtrinsicFailed is prompted, it means failure;
        If system.ExtrinsicSuccess is prompted, it means success;`
	HELP_UpdateAddress = `    3.Check the Sminer_UpdataIp transaction event result in the block hash above:
        If system.ExtrinsicFailed is prompted, it means failure;
        If system.ExtrinsicSuccess is prompted, it means success;`
	HELP_UpdataBeneficiary = `    3.Check the Sminer_UpdataBeneficiary transaction event result in the block hash above:
        If system.ExtrinsicFailed is prompted, it means failure;
        If system.ExtrinsicSuccess is prompted, it means success;`
)

// Miner info
// updated at runtime
var (
	Spk           []byte
	Shared_params []byte
	Shared_g      []byte
	//PublicKey     []byte

	//data path
	BaseDir    = "bucket"
	LogfileDir = "/log"
	SpaceDir   = "space"
	FilesDir   = "files"
)
