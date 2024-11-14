// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

#[allow(non_snake_case)]
#[allow(clippy::type_complexity)]
pub mod SortedSets {
    use std::cmp::Ordering;

    #[allow(non_camel_case_types)]
    pub struct _default {}
    impl _default {
        pub fn SetToSequence<T: ::dafny_runtime::DafnyTypeEq>(
            elems: &::dafny_runtime::Set<T>,
        ) -> ::dafny_runtime::Sequence<T> {
            elems.iter().cloned().collect()
        }

        pub fn SetToOrderedSequence<T: ::dafny_runtime::DafnyTypeEq>(
            elems: &::dafny_runtime::Set<::dafny_runtime::Sequence<T>>,
            less: &::std::rc::Rc<dyn Fn(&T, &T) -> bool>,
        ) -> ::dafny_runtime::Sequence<::dafny_runtime::Sequence<T>> {
            let mut vec = elems.iter().cloned().collect::<Vec<_>>();
            vec.sort_by(|a, b| Self::order(a, b, less));
            dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&vec, |x| x.clone())
        }

        pub fn SetToOrderedSequence2<T: ::dafny_runtime::DafnyTypeEq>(
            elems: &::dafny_runtime::Set<::dafny_runtime::Sequence<T>>,
            less: &::std::rc::Rc<dyn Fn(&T, &T) -> bool>,
        ) -> ::dafny_runtime::Sequence<::dafny_runtime::Sequence<T>> {
            Self::SetToOrderedSequence(elems, less)
        }

        fn order<T: ::dafny_runtime::DafnyTypeEq>(
            x: &::dafny_runtime::Sequence<T>,
            y: &::dafny_runtime::Sequence<T>,
            less: &::std::rc::Rc<dyn Fn(&T, &T) -> bool>,
        ) -> Ordering {
            let mut iter1 = x.iter();
            let mut iter2 = y.iter();

            loop {
                match (iter1.next(), iter2.next()) {
                    (Some(lhs), Some(rhs)) => {
                        if less(&lhs, &rhs) {
                            return Ordering::Less;
                        }
                        if less(&rhs, &lhs) {
                            return Ordering::Greater;
                        }
                    }
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (None, None) => return Ordering::Equal,
                }
            }
        }
    }
}
