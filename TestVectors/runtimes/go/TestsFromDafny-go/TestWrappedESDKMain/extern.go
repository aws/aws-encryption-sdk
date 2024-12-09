package TestWrappedESDKMain

import (
	os "os"

	"github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
)

var m_TestWrappedESDKMain = CompanionStruct_Default___{}

func (CompanionStruct_Default___) GetTestVectorExecutionDirectory() dafny.Sequence {
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	return dafny.SeqOfString(cwd + "/../../../")
}
