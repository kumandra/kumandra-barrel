package node

import (
	"errors"

	"github.com/kumandra/kumandra-barrel/utils"
	"github.com/kumandra/kumandra/keyring"
)

func verifysign(pkey, signmsg, sign []byte) (bool, error) {
	if len(signmsg) == 0 || len(sign) < 64 {
		return false, errors.new("wrong signature")
	}

	ss58, err := utils.encodetoss58(pkey)
	if err != nil {
		return false, err
	}

	verkr, _ := keyring.fromuri(ss58, keyring.netsubstrate{})

	var sign_array [64]byte
	for i := 0; i < 64; i++ {
		sign_array[i] = sign[i]
	}

	// verify signature
	return verkr.verify(verkr.signingcontext(signmsg), sign_array), nil
}
