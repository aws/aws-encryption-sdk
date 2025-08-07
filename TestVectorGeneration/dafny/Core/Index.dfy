include "../../../mpl/StandardLibrary/src/Index.dfy"

module TestVectorGeneration {
    import FileIO
    import opened Wrappers
    import opened StandardLibrary.UInt
    import opened JSON.Values
    import JSON.API
    import JSON.Errors

    method WriteVectorsFile(location: string, json: JSON)
        returns (output: Result<(), string>)
    {
        var jsonBytes :- API.Serialize(json)
            .MapFailure(( e: Errors.SerializationError ) => e.ToString());
        output := FileIO.WriteBytesToFile(location, jsonBytes);
    }

    // This module represents a value space.
    // A value space only defines a particular type.
    // This module should not be refined directly, but rather used as a base for
    // the EnumeratedValueSpace and PartitionedValueSpace modules.
    abstract module ValueSpace {
        type ValueType(!new, ==)

        // Function to check if a value is in the value space and is valid
        // Implemented by refining modules with specific checks for "is valid"
        function method IsValidMember(x: ValueType): bool
            ensures IsValidMember(x) ==>
                && x is ValueType

        // "Public" method to get a list of testing values from the value space
        method GetValues() returns (output: seq<ValueType>)
            // This MUST return at least one value
            ensures |output| > 0
            // All values must be valid
            ensures forall i :: 0 <= i < |output| ==>
                IsValidMember(output[i])

        // Method to get a single testing value from the value space
        method SomeValue() returns (output: ValueType)
            // All values must be valid
            ensures IsValidMember(output)
        {
            var values := GetValues();
            output := values[0];
        }

        // If the value space has any pruning configurations,
        // they must be enumerated in this list.
        method PruningConfigurationList(x: ValueType) returns (output: seq<(bool, ValueType)>)
        {
            return [];
        }

        // Helper method to replace a value in the value space with its representative value
        // if it matches a pruning configuration.
        method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
        {
            var pruningConfigurationList := PruningConfigurationList(x);
            output := x;
            for i := 0 to |pruningConfigurationList| {
                // If the pruning configuration matches,
                if pruningConfigurationList[i].0 {
                    // replace with the representative value for this pruning configuration
                    output := pruningConfigurationList[i].1;
                    return output;
                }
            }
        }

        // Public helper method to be called from implementing RepresentativeValues methods.
        // This method adds a value to a list of values if it is not pruned.
        method AddIfNotPruned(x: ValueType, vals: seq<ValueType>) returns (output: seq<ValueType>)
        {
            var replace := MaybeReplaceWithRepresentativeValue(x);
            if x == replace {
                // If the representative value is the same as the input, we can return it directly.
                return vals + [x];
            } else {
                return vals;
            }
        }
    }

    // This module represents a value space whose values are best represented in test vectors
    // as a finite set of enumerated values.
    // This module guarantees that `ValueSpaceList` method fully enumerates the value space; i.e.
    // 1. All values in `ValueSpaceList` are in the value space;
    // 2. All values in the value space are in `ValueSpaceList`.
    // For example, if the value space were an enum, then this module would
    // guarantee that the `Values` method returns all possible enum values.
    // (If a new enum value is added, then the module would fail to verify until the `Values` method is updated.)
    // This module is abstract and should be refined by concrete implementations
    // that enumerate a concrete value space.
    abstract module EnumeratedValueSpace refines ValueSpace {
        import opened JSON.Values

        type ValueType(!new, ==)

        // Function that enumerates all values in the value space.
        // This MUST be implemented to return a finite set of values that represents all values in the value space.
        // This is enforced by the `ensures` clause in the `Values` method.
        function method ValueSpaceList(): seq<ValueType>
            // This MUST return at least one value
            ensures |ValueSpaceList()| > 0
            // All values in the list MUST be valid
            ensures forall i :: 0 <= i < |ValueSpaceList()| ==> 
                IsValidMember(ValueSpaceList()[i])

        // Counts the number of `true` values in the partition list
        function method CountTrues(partitions: seq<bool>): int
            ensures 0 <= CountTrues(partitions) <= |partitions|
            // Result of 0 implies no `true` values in the list
            ensures CountTrues(partitions) == 0 ==> 
                forall i :: 0 <= i < |partitions| ==> !partitions[i]
            // Result of 1 implies exactly one `true` value in the list
            ensures CountTrues(partitions) == 1 ==> 
                exists i :: 0 <= i < |partitions| && partitions[i] &&
                    forall j :: 0 <= j < |partitions| && j != i ==> !partitions[j]
            // Positive result implies at least one `true` value in the list
            ensures CountTrues(partitions) >= 1 ==> 
                exists i :: 0 <= i < |partitions| && partitions[i]
        {
            if |partitions| == 0 then
                0
            else if partitions[0] then
                1 + CountTrues(partitions[1..])
            else
                CountTrues(partitions[1..])
        }

        // This is a temporary method I want to remove.
        // Right now, the implementer must hardcode a list of checks against the ValueSpaceList,
        // but this can be written in this module to derive the checks from the ValueSpaceList..
        function method ValueSpaceEqualityList(x: ValueType): seq<bool>
            ensures |ValueSpaceEqualityList(x)| > 0
            ensures CountTrues(ValueSpaceEqualityList(x)) == 1

        // Returns true if the input is exactly one value in the value space
        // This is used to assert that the value space is exhaustive and non-repeating.
        function method IsExactlyOneValueInValueSpace(x: ValueType): bool
            ensures IsExactlyOneValueInValueSpace(x) ==>
                exists i :: 0 <= i < |ValueSpaceEqualityList(x)| && ValueSpaceEqualityList(x)[i]
                    && forall j :: 0 <= j < |ValueSpaceEqualityList(x)| && j != i ==> !ValueSpaceEqualityList(x)[j]
        {
            var partitionList := ValueSpaceEqualityList(x);
            CountTrues(partitionList) == 1
        }

        function method IsValidMember(x: ValueType): bool
        {
            && x is ValueType
            && IsExactlyOneValueInValueSpace(x)
        }

        // Method to get a list of all values in the value space for testing
        method GetValues() returns (output: seq<ValueType>)
            // This MUST return at least one value
            ensures |output| > 0
            // All returned values MUST be valid
            ensures forall i :: 0 <= i < |output| ==> 
                IsValidMember(output[i])
        {
            output := ValueSpaceList();
        }
    }

    // This module represents a subset of a value space.
    // It defines a partition of the value space and provides methods to check
    // if a value is in the partition and to get representative values from it.
    // You should only use a Partition if the partition has an infinite number of values,
    // or if it is impractical to enumerate all values in the partition.
    // Partition provides "representative" values, which are a finite set of values
    // that are intended to "represent" the infinite set of values in the partition.
    // This module guarantees that the values provided by `RepresentativeValues` are in the partition.
    // This module is abstract and should be refined by specific value space partitions.
    abstract module Partition refines ValueSpace {
        type ValueType(!new, ==)

        // Function to check if a value is in the partition
        function method IsInPartition(x: ValueType): bool

        function method IsValidMember(x: ValueType): bool
            ensures IsValidMember(x) ==>
                && x is ValueType
                && IsInPartition(x)
        {
            IsInPartition(x)
        }
        
        // "Private" method that returns a list of representative values from the partition.
        // This shouldn't be called directly, but rather through the `Values` method.
        method RepresentativeValues() returns (output: seq<ValueType>)
            // This MUST return at least one representative value
            ensures |output| > 0
            // All representative values MUST be in the partition
            ensures forall i :: 0 <= i < |output| ==> 
                IsInPartition(output[i])

        method GetValues() returns (output: seq<ValueType>)
            // This MUST return at least one value
            ensures |output| > 0
            // All values must be valid
            ensures forall i :: 0 <= i < |output| ==>
                IsValidMember(output[i])
        {
            output := RepresentativeValues();
        }
    }

    // This module represents a complete partitioning of a value space into one or more partitions.
    // This module guarantees that the value sapce is partitioned by the
    // `Partitions` specified in the `PartitionMemberships` method.
    // This guarantees that the partitions wholly cover the value space and that the partitions do not overlap.
    // The `PartitionMemberships` method MUST be a list of calls to `IsInPartition` methods
    // of the `Partitions`s that cover the entire value space; this is enforced by Dafny clauses.
    abstract module PartitioningScheme refines ValueSpace {
        import opened JSON.Values

        type ValueType(!new, ==)

        const PartitionCount: int

        // List of `IsInPartition` results for each partition in the value space
        function method PartitionMemberships(x: ValueType): seq<bool>
            // All values in the value space MUST be in exactly one partition
            ensures CountTrues(PartitionMemberships(x)) == 1
            ensures |PartitionMemberships(x)| == PartitionCount

        // Counts the number of `true` values in the partition list
        function method CountTrues(partitions: seq<bool>): int
            // MUST return as many values as there are partitions
            ensures 0 <= CountTrues(partitions) <= |partitions|
            // If there are no `true` values in the list, then MUST return 0
            ensures CountTrues(partitions) == 0 ==> 
                forall i :: 0 <= i < |partitions| ==> !partitions[i]
            // If there is exactly one `true` value in the list, then MUST return 1
            ensures CountTrues(partitions) == 1 ==> 
                exists i :: 0 <= i < |partitions| && partitions[i] &&
                    forall j :: 0 <= j < |partitions| && j != i ==> !partitions[j]
            // If there is at least one `true` value in the list, then MUST return a positive number
            ensures CountTrues(partitions) >= 1 ==> 
                exists i :: 0 <= i < |partitions| && partitions[i]
        {
            if |partitions| == 0 then
                0
            else if partitions[0] then
                1 + CountTrues(partitions[1..])
            else
                CountTrues(partitions[1..])
        }

        // Returns true if the input is in exactly one partition
        function method IsInExactlyOnePartition(x: ValueType): bool
            ensures IsInExactlyOnePartition(x) ==>
                exists i :: 0 <= i < |PartitionMemberships(x)| && PartitionMemberships(x)[i]
                    && forall j :: 0 <= j < |PartitionMemberships(x)| && j != i ==> !PartitionMemberships(x)[j]
        {
            var partitionList := PartitionMemberships(x);
            CountTrues(partitionList) == 1
        }

        // Assertion that every partition in the partitioning scheme is represented by at least one value
        function method ValuesExhaustPartitions(x: seq<ValueType>): bool
            // Ensures that all values in the value space are in exactly one partition
            requires |x| > 0
            requires forall i :: 0 <= i < |x| ==> |PartitionMemberships(x[0])| == |PartitionMemberships(x[i])|
        {
            // All inputs are in exactly one partition
            (forall i :: 0 <= i < |x| ==> IsInExactlyOnePartition(x[i]))
            &&
            // Every partition is represented by at least one value
            (forall p {:trigger PartitionMemberships(x[0])[p]} :: 0 <= p < |PartitionMemberships(x[0])| ==> 
                exists j {:trigger PartitionMemberships(x[j])[p]} :: 0 <= j < |x| && PartitionMemberships(x[j])[p])
        }

        function method IsValidMember(x: ValueType): bool
        {
            && x is ValueType
            && IsInExactlyOnePartition(x)
        }
        
        function method ToJSON(x: ValueType): JSON

        method GenerateJSON() returns (outputJSON: JSON)
        {
            var values := GetValues();
            var out := [];
            for i := 0 to |values| {
                var value := values[i];
                var json := ToJSON(value);
                out := out + [json];
            }
            outputJSON := JSON.Array(out);
        }

        // "Private" method that returns a list of representative values from the partitioning scheme.
        // This shouldn't be called directly, but rather through the `GetValues` method.
        method RepresentativeValues() returns (output: seq<ValueType>)
            // This MUST return at least one representative value
            ensures |output| > 0
            // All representative values must be in exactly one partition
            ensures forall i :: 0 <= i < |output| ==> 
                IsInExactlyOnePartition(output[i])
            // All values must exhaust the partitions
            ensures ValuesExhaustPartitions(output)

        method GetValues() returns (output: seq<ValueType>)
            // This MUST return at least one value
            ensures |output| > 0
            // All values must be in exactly one partition
            ensures forall i :: 0 <= i < |output| ==> 
                IsInExactlyOnePartition(output[i])
            ensures ValuesExhaustPartitions(output)
        {
            var raw := RepresentativeValues();
            var seen := {};
            output := [];

            for i := 0 to |raw| {
                if raw[i] !in seen {
                    output := output + [raw[i]];
                    seen := seen + {raw[i]};
                }
            }

            // TODO: prove deduping values
            assert |output| > 0 by {
                assert |raw| > 0;
                assert |seen| > 0 by {
                    assume {:axiom} raw[0] in seen;
                }
                assert |seen| == |output| by {
                    assume {:axiom} |output| == |seen|;
                }
            }

            assume {:axiom} ValuesExhaustPartitions(output);
        }
    }

    // This module provides a pruning configuration for a value space.
    // A pruning configuration is a combination of a rule and a representative value.
    // The rule identifies input configurations that can be pruned,
    // and the representative value is a value that can be used in place of the pruned values.
    abstract module PruningConfiguration {
        type ValueType(==)

        function method MatchesPruningConfiguration(x: ValueType): bool

        method RepresentativeValue(input: ValueType) returns (output: ValueType)
            ensures MatchesPruningConfiguration(output)

        method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
            ensures !MatchesPruningConfiguration(x) ==> output == x
            ensures MatchesPruningConfiguration(x) ==> MatchesPruningConfiguration(output)
        {
            if MatchesPruningConfiguration(x) {
                output := RepresentativeValue(x);
            } else {
                output := x;
            }
        }

        method PruningConfiguration(x: ValueType) returns (output: (bool, ValueType))
        {
            var matches := MatchesPruningConfiguration(x);
            var value := RepresentativeValue(x);
            output := (matches, value);
        }
    }

    // This module provides a base for operation models.
    // An operation model defines a domain and range for an operation,
    // and provides methods to evaluate the operation and generate JSON representations.
    // This module is abstract and should be refined by specific operation models.
    abstract module OperationModel
    {
        import opened JSON.Values

        type Domain
        type Range

        const operationName: string

        // Implementing modules need to implement this method,
        // which computes the expected output for a given input.
        method EvaluationRule(x: Domain) returns (output: Range)

        // Implementing modules need to implement this method,
        // which returns a list of representative values for the domain.
        method DomainRepresentativeValues() returns (output: seq<Domain>)
            ensures |output| > 0

        method GenerateJSON() returns (outputJSON: JSON)
        {
            var operationsJSON := [];
            var inputValues := DomainRepresentativeValues();
            for i := 0 to |inputValues| {
                var input := inputValues[i];
                var json := JSONToWriteForOperationInputValue(input);
                operationsJSON := operationsJSON + [json];
            }
            outputJSON := Object([(operationName, Array(operationsJSON))]);
        }

        // Implementing modules need to implement this method.
        function method InputToJSON(x: Domain): JSON

        // Implementing modules need to implement this method.
        function method OutputToJSON(x: Range): JSON

        method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
        {
            var outputValue := EvaluationRule(input);
            var inputJSON := InputToJSON(input);
            var outputJSON := OutputToJSON(outputValue);
            output := Object(
                [("input", inputJSON), ("output", outputJSON)]
            );
        }
    }

    // This module provides a base for service models.
    // A service model defines a service name and namespace,
    // and provides methods to generate operations JSON and the service JSON.
    // This module is abstract and should be refined by specific service models.
    abstract module ServiceModel
    {
        import opened JSON.Values

        const serviceName: string
        const serviceNamespace: string

        // Implementing modules only need to implement this method.
        method GenerateOperationsJSON() returns (operationsJSON: JSON)

        method GenerateJSON() returns (outputJSON: JSON)
        {
            var operationsJSON := GenerateOperationsJSON();
            outputJSON :=
                Object(
                    [(serviceNamespace + ":" + serviceName, operationsJSON)]
                );
        }
    }
}
