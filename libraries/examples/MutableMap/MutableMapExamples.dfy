/*******************************************************************************
*  Copyright by the contributors to the Dafny Project
*  SPDX-License-Identifier: MIT
*******************************************************************************/

// RUN: %run --target:java "%s" --input "%S/../../src/MutableMap/MutableMap.java"

include "../../src/MutableMap/MutableMap.dfy"
include "../../src/Wrappers.dfy"

/**
  *  Tests the Java interfacing implementation of mutable maps.
  */
module {:options "-functionSyntax:4"} MutableMapExamples {
  import opened DafnyLibraries
  import opened Wrappers

  method AssertAndExpect(p: bool, maybeMsg: Option<string> := None) requires p {
    match maybeMsg {
      case None => {
          expect p;
      }
      case Some(msg) => {
          expect p, msg;
      }
    }
  }

  method Main() {
    var m := new MutableMap<string,string>();
    AssertAndExpect(m.Keys() == {});
    AssertAndExpect(m.Values() == {});
    AssertAndExpect(m.Items() == {});
    AssertAndExpect(m.Size() == 0);

    m.Put("testkey", "testvalue");
    AssertAndExpect(m.Select("testkey") == "testvalue");
    AssertAndExpect(m.Keys() == {"testkey"});
    AssertAndExpect(m.Values() == {"testvalue"});
    AssertAndExpect(m.Items() == {("testkey", "testvalue")});
    print m.Select("testkey"), "\n";

    AssertAndExpect(m.Size() == 1);
    m.Put("testkey", "testvalue2");
    AssertAndExpect(m.Keys() == {"testkey"});
    AssertAndExpect(m.Values() == {"testvalue2"});
    AssertAndExpect(m.Items() == {("testkey", "testvalue2")});

    m.Put("testkey2", "testvalue2");
    AssertAndExpect(m.Keys() == {"testkey", "testkey2"});
    AssertAndExpect(m.Values() == {"testvalue2"});
    AssertAndExpect(m.Items() == {("testkey", "testvalue2"), ("testkey2", "testvalue2")});
    AssertAndExpect(m.Size() == 2);
    AssertAndExpect("testkey" in m.Keys());
    AssertAndExpect("testkey2" in m.Keys());
    print m.Select("testkey"), "\n";
    print m.Select("testkey2"), "\n";

    m.Remove("testkey");
    AssertAndExpect(m.Keys() == {"testkey2"});
    AssertAndExpect(m.Values() == {"testvalue2"});
    AssertAndExpect(m.Items() == {("testkey2", "testvalue2")});
  }
}