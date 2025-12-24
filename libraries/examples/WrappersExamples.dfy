// RUN: %run "%s" > "%t"

// OutputCheck is broken, see https://github.com/dafny-lang/libraries/issues/164
// not working: %OutputCheck --file-to-check "%t" "%s"
// CHECK-L: Hello
// CHECK-NEXT-L: Error: 'name' was not found
// CHECK-NEXT-L: Hello
// CHECK-NEXT-L: Dafny
// CHECK-NEXT-L: Test dummy file

include "../src/dafny/Wrappers.dfy"

module {:options "--function-syntax:4"} Demo {
  import opened Dafny.Wrappers

  // ------ Demo for Option ----------------------------
  // We use Option when we don't need to pass around a reason for the failure,
  // ie when just 'None' is sufficient:

  class MyMap<K(==), V> {
    var m: map<K, V>
    constructor () {
      m := map[];
    }
    function Get(k: K): Option<V>
      reads this
    {
      if k in m then Some(m[k]) else None()
    }
    method Put(k: K, v: V)
      modifies this
    {
      this.m := this.m[k := v];
    }
  }

  method TestMyMap() {
    var m := new MyMap<string, string>();
    m.Put("message", "Hello");
    Greet(m);

    m.Put("name", "Dafny");
    Greet(m);
  }

  method Greet(m: MyMap<string, string>) {
    var o: Option<string> := m.Get("message");
    if o.Some? {
      print o.value, "\n";
    } else {
      print "oops\n";
    }

    var r: Result<string, string> := FindName(m);
    if r.Success? {
      print r.value, "\n";
    } else {
      print "Error: ", r.error, "\n";
    }
  }

  // Sometimes we want to go from Option to Result:
  method FindName(m: MyMap<string, string>) returns (res: Result<string, string>) {
    // Will return a default error message in case of None:
    res := m.Get("name").ToResult("'name' not found");
    // We can also match on the option to write a custom error:
    match m.Get("name")
    case Some(n) => res := Success(n);
    case None => res := Failure("'name' was not found");
  }

  // Propagating failures using :- statements
  method GetGreeting(m: MyMap<string, string>) returns (res: Option<string>) {
    var message: string :- m.Get("message");
    var nameResult := FindName(m);
    var name :- nameResult.ToOption();
    res := Some(message + " " + name);
  }

  // ------ Demo for Result ----------------------------
  // We use Result when we want to give a reason for the failure:

  class MyFilesystem {
    var files: map<string, string>
    constructor() {
      files := map[];
    }

    // Result<()> is used to return a Result without any data
    method WriteFile(path: string, contents: string) returns (res: Result<(), string>)
      modifies this
    {
      if path in files {
        files := files[path := contents];
        res := Success(()); // `()` is the only value of type `()`
      } else {
        // The "Result" datatype allows us to give precise error messages
        // instead of just "None"
        res := Failure("File not found, use 'Create' before");
      }
    }

    method CreateFile(path: string) returns (res: Result<(), string>)
      modifies this
    {
      if path in files {
        res := Failure("File already exists");
      } else {
        files := files[path := ""];
        res := Success(());
      }
    }

    method ReadFile(path: string) returns (res: Result<string, string>) {
      if path in files {
        res := Success(files[path]);
      } else {
        res := Failure("File not found");
      }
    }
  }

  // Propagating failures using :- statements
  method CopyFile(fs: MyFilesystem, fromPath: string, toPath: string) returns (res: Result<(), string>)
    modifies fs
  {
    var contents :- fs.ReadFile(fromPath);
    var _ :- fs.CreateFile(toPath);
    var _ :- fs.WriteFile(toPath, contents);
    res := Success(());
  }

  method TestMyFilesystem() {
    var fs := new MyFilesystem();
    // Note: these verbose "outcome.Failure?" patterns will soon
    // not be needed any more, see https://github.com/dafny-lang/rfcs/pull/1
    var outcome: Result<(), string> := fs.CreateFile("test.txt");
    if outcome.Failure? {
      print outcome.error, "\n";
      return;
    }
    outcome := fs.WriteFile("test.txt", "Test dummy file");
    if outcome.Failure? {
      print outcome.error, "\n";
      return;
    }
    var fileResult: Result<string, string> := fs.ReadFile("test.txt");
    if outcome.Failure? {
      print fileResult.error, "\n";
      return;
    }
    if fileResult.Success? {
      print fileResult.value, "\n";
    }
  }

  // ------ Demo for Need ----------------------------
  // We use Need when something has to be true but we can't prove it statically
  // This is a very contrived example

  method whatIsCharacterFive(fs: MyFilesystem, fromPath: string) returns (res: Result<char, string>)
    modifies fs
  {

    // Get a string that we can't reason about statically
    var contents :- fs.ReadFile(fromPath);

    // Dynamically test whether the string is at least 5 characters long, and return a failure if not.
    // If we pass this line, Dafny can now assume that the string is long enough.
    :- Need<string>(|contents| >= 5, "File contents not long enough.");

    // Now we can get the character
    var c := contents[4];

    return Success(c);
  }

  // For a method that returns an Outcome, use Outcome.Need
  method whatIsCharacterFive'(fs: MyFilesystem, fromPath: string) returns (res: Outcome<string>)
    modifies fs
  {

    // Get a string that we can't reason about statically
    var contents: string := *;

    // Dynamically test whether the string is at least 5 characters long, and return a failure if not.
    // If we pass this line, Dafny can now assume that the string is long enough.
    :- Outcome.Need(|contents| >= 5, "File contents not long enough.");

    // Now we can get the character
    var c := contents[4];
    // and do other stuff

    return Pass;
  }

  method Main() {
    TestMyMap();
    TestMyFilesystem();
  }

}
