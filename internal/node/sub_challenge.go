package node

import (
	"encoding/json"
	"io/ioutil"
	"math"
	"os"
	"path/filepath"
	"time"

	"github.com/kumandra/kumandra-barrel/configs"
	"github.com/kumandra/kumandra-barrel/internal/chain"
	. "github.com/kumandra/kumandra-barrel/internal/logger"
	"github.com/kumandra/kumandra-barrel/internal/pattern"
	api "github.com/kumandra/kumandra-barrel/internal/proof/apiv1"
	"github.com/kumandra/kumandra-barrel/utils"

	"github.com/pkg/errors"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// The task_HandlingChallenges task will automatically help you complete file challenges.
// Apart from human influence, it ensures that you submit your certificates in a timely manner.
// It keeps running as a subtask.
func (node *Node) task_HandlingChallenges(ch chan bool) {
	var (
		txhash          string
		fileid          string
		fileFullPath    string
		fileTagFullPath string
		blocksize       int64
		filetag         api.TagInfo
		poDR2prove      api.PoDR2Prove
		proveResponse   api.PoDR2ProveResponse
		proveInfos      = make([]chain.ProveInfo, 0)
	)
	defer func() {
		if err := recover(); err != nil {
			Pnc.Sugar().Errorf("%v", utils.RecoverError(err))
		}
		ch <- true
	}()
	Chg.Info(">>>>> Start task_HandlingChallenges <<<<<")

	for {
		if pattern.GetMinerState() != pattern.M_Positive {
			if pattern.GetMinerState() == pattern.M_Pending {
				time.Sleep(time.Second * 6)
				continue
			}
			time.Sleep(time.Minute * time.Duration(utils.RandomInRange(1, 5)))
			continue
		}

		chlng, err := chain.GetChallenges()
		if err != nil {
			if err.Error() != chain.ERR_Empty {
				Chg.Sugar().Errorf("%v", err)
				chlng, _ = chain.GetChallenges()
			}
		}

		if len(chlng) == 0 {
			time.Sleep(time.Minute)
			continue
		}

		time.Sleep(time.Second * time.Duration(utils.RandomInRange(30, 60)))
		Chg.Sugar().Infof("--> Number of challenges: %v ", len(chlng))

		for i := 0; i < len(chlng); i++ {
			if len(proveInfos) >= 50 {
				// proof up chain
				for {
					txhash, err = chain.SubmitProofs(proveInfos)
					if txhash == "" {
						Chg.Sugar().Errorf("SubmitProofs fail: %v", err)
					} else {
						proveInfos = make([]chain.ProveInfo, 0)
						Chg.Sugar().Infof("SubmitProofs suc: %v", txhash)
						break
					}
					time.Sleep(time.Second * time.Duration(utils.RandomInRange(5, 20)))
				}
			}

			fileid = string(chlng[i].File_id[:])
			if chlng[i].File_type == 1 {
				//space file
				fileFullPath = filepath.Join(configs.SpaceDir, fileid)
				fileTagFullPath = filepath.Join(configs.SpaceDir, fileid+".tag")
			} else {
				//user file
				fileFullPath = filepath.Join(configs.FilesDir, fileid)
				fileTagFullPath = filepath.Join(configs.FilesDir, fileid+".tag")
			}

			fstat, err := os.Stat(fileFullPath)
			if err != nil {
				Chg.Sugar().Errorf("[%v] %v", fileid, err)
				continue
			}
			if chlng[i].File_type == 1 {
				blocksize = configs.BlockSize
			} else {
				blocksize, _ = calcFileBlockSizeAndScanSize(fstat.Size())
			}

			qSlice, err := api.PoDR2ChallengeGenerateFromChain(chlng[i].Block_list, chlng[i].Random)
			if err != nil {
				Chg.Sugar().Errorf("[%v] %v", fileid, err)
				continue
			}

			ftag, err := ioutil.ReadFile(fileTagFullPath)
			if err != nil {
				Chg.Sugar().Errorf("[%v] %v", fileid, err)
				continue
			}
			err = json.Unmarshal(ftag, &filetag)
			if err != nil {
				Chg.Sugar().Errorf("[%v] %v", fileid, err)
				continue
			}

			poDR2prove.QSlice = qSlice
			poDR2prove.T = filetag.T
			poDR2prove.Sigmas = filetag.Sigmas

			matrix, _, err := split(fileFullPath, blocksize, fstat.Size())
			if err != nil {
				Chg.Sugar().Errorf("[%v] %v", fileid, err)
				continue
			}

			poDR2prove.Matrix = matrix
			poDR2prove.S = blocksize
			proveResponseCh := poDR2prove.PoDR2ProofProve(
				configs.Spk,
				string(configs.Shared_params),
				configs.Shared_g,
				int64(configs.ScanBlockSize),
			)
			select {
			case proveResponse = <-proveResponseCh:
			}
			if proveResponse.StatueMsg.StatusCode != api.Success {
				Chg.Sugar().Errorf("[%v] PoDR2ProofProve failed", fileid)
				continue
			}

			var proveInfoTemp chain.ProveInfo
			proveInfoTemp.U = make([]types.Bytes, len(filetag.T.T0.U))
			proveInfoTemp.Cinfo = chlng[i]
			proveInfoTemp.FileId = chlng[i].File_id

			var mus []types.Bytes = make([]types.Bytes, len(proveResponse.MU))
			for i := 0; i < len(proveResponse.MU); i++ {
				mus[i] = make(types.Bytes, 0)
				mus[i] = append(mus[i], proveResponse.MU[i]...)
			}
			proveInfoTemp.Mu = mus
			proveInfoTemp.Sigma = types.Bytes(proveResponse.Sigma)
			proveInfoTemp.MinerAcc = types.NewAccountID(pattern.GetMinerAcc())
			proveInfoTemp.Name = filetag.T.T0.Name
			for j := 0; j < len(filetag.T.T0.U); j++ {
				proveInfoTemp.U[j] = make(types.Bytes, 0)
				proveInfoTemp.U[j] = append(proveInfoTemp.U[j], filetag.T.T0.U[j]...)
			}
			proveInfos = append(proveInfos, proveInfoTemp)
		}

		if len(proveInfos) == 0 {
			continue
		}

		// proof up chain
		ts := time.Now().Unix()
		for {
			txhash, err = chain.SubmitProofs(proveInfos)
			if txhash == "" {
				Chg.Sugar().Errorf("SubmitProofs fail: %v", err)
			} else {
				Chg.Sugar().Infof("SubmitProofs suc: %v", txhash)
				proveInfos = make([]chain.ProveInfo, 0)
				break
			}
			if time.Since(time.Unix(ts, 0)).Minutes() > 2.0 {
				Chg.Sugar().Errorf("SubmitProofs fail and exit")
				break
			}
			time.Sleep(time.Second * time.Duration(utils.RandomInRange(5, 20)))
		}
	}
}

func calcFileBlockSizeAndScanSize(fsize int64) (int64, int64) {
	var (
		blockSize     int64
		scanBlockSize int64
	)
	if fsize < configs.SIZE_1KiB {
		return fsize, fsize
	}
	if fsize > math.MaxUint32 {
		blockSize = math.MaxUint32
		scanBlockSize = blockSize / 8
		return blockSize, scanBlockSize
	}
	blockSize = fsize / 16
	scanBlockSize = blockSize / 8
	return blockSize, scanBlockSize
}

func split(filefullpath string, blocksize, filesize int64) ([][]byte, uint64, error) {
	file, err := os.Open(filefullpath)
	if err != nil {
		return nil, 0, err
	}
	defer file.Close()

	if filesize/blocksize == 0 {
		return nil, 0, errors.New("filesize invalid")
	}
	n := filesize / blocksize
	if n == 0 {
		n = 1
	}
	// matrix is indexed as m_ij, so the first dimension has n items and the second has s.
	matrix := make([][]byte, n)
	for i := int64(0); i < n; i++ {
		piece := make([]byte, blocksize)
		_, err := file.Read(piece)
		if err != nil {
			return nil, 0, err
		}
		matrix[i] = piece
	}
	return matrix, uint64(n), nil
}
