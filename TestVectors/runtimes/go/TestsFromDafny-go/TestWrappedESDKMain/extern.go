package TestWrappedESDKMain

import (
	os "os"

	"github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
)

// TODO: Remove this once Dafny bug is fixed.
// Dafny bug: https://t.corp.amazon.com/9a9028fd-2711-4843-b8f0-09965f7e61a7/communication#f03694be-7410-47f9-866d-e43093b018f9
var m_TestWrappedESDKMain = CompanionStruct_Default___{}

func (CompanionStruct_Default___) GetTestVectorExecutionDirectory() dafny.Sequence {
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	return dafny.SeqOfString(cwd + "/../../../")
}
