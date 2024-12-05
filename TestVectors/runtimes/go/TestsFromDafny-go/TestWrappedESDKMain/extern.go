package TestWrappedESDKMain

import (
	os "os"

	"github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
)

func GetTestVectorExecutionDirectory() dafny.Sequence {
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	return dafny.SeqOfString(cwd + "/../../../")
}
