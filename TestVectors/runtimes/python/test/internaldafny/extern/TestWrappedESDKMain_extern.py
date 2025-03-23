import TestWrappedESDKMain
import _dafny
import os

class default__(TestWrappedESDKMain.default__):
    @staticmethod
    def GetTestVectorExecutionDirectory():
        return _dafny.Seq(os.getcwd() + "/../../")

TestWrappedESDKMain.default__ = default__