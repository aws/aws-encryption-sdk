// RUN: %verify "%s"

/*******************************************************************************
 *  Original Copyright under the following: 
 *  Copyright 2018-2021 VMware, Inc., Microsoft Inc., Carnegie Mellon University, 
 *  ETH Zurich, and University of Washington
 *  SPDX-License-Identifier: BSD-2-Clause 
 * 
 *  Copyright (c) Microsoft Corporation
 *  SPDX-License-Identifier: MIT 
 * 
 *  Modifications and Extensions: Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT 
 *******************************************************************************/

module {:options "-functionSyntax:4"} Functions {
  ghost predicate Injective<X(!new), Y>(f: X-->Y)
    reads f.reads
    requires forall x :: f.requires(x)
  {
    forall x1, x2 :: f(x1) == f(x2) ==> x1 == x2
  }

}
