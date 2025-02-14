import aws_encryption_sdk_test_vectors.internaldafny.generated.module_ as module_
import aws_encryption_sdk_test_vectors.internaldafny.generated.WrappedESDKMain as WrappedESDKMain


def new_test_main(args):
    WrappedESDKMain.default__.Main2(args)


module_.default__.Test____Main____ = new_test_main
