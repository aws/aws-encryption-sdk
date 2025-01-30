// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

using System;
using System.Collections.Generic;

namespace AWS.Cryptography.EncryptionSDK
{
    public class CollectionOfErrors : Exception
    {
        public readonly System.Collections.Generic.List<Exception> list;
        public CollectionOfErrors(System.Collections.Generic.List<Exception> list, string message) : base(message + $"\n List: \n{ListAsString(list)}") { this.list = list; }
        public CollectionOfErrors(string message) : base(message) { this.list = new System.Collections.Generic.List<Exception>(); }
        public CollectionOfErrors() : base("CollectionOfErrors") { this.list = new System.Collections.Generic.List<Exception>(); }

        private static string ListAsString(List<Exception> list)
        {
            if (list.Count < 1) return "";
            string [] msgArr = new string [list.Count];
            for (int i = 0; i < list.Count; i++)
                msgArr[i] = $"{list[i].GetType().Name} :: {list[i].Message}";
            return String.Join("\n\t", msgArr);
        }
    }

}
