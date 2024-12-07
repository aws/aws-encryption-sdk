// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use std::sync::LazyLock;
use crate::deps::com_amazonaws_dynamodb::conversions;

#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    pub inner: aws_sdk_dynamodb::Client
}

impl ::std::cmp::PartialEq for Client {
  fn eq(&self, other: &Self) -> bool {
    false
  }
}

impl ::std::convert::Into<Client> for aws_sdk_dynamodb::Client {
    fn into(self) -> Client {
        Client { inner: self }
    }
}

/// A runtime for executing operations on the asynchronous client in a blocking manner.
/// Necessary because Dafny only generates synchronous code.
static dafny_tokio_runtime: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()
          .unwrap()
});

impl dafny_runtime::UpcastObject<dyn std::any::Any> for Client {
    ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
}

impl dafny_runtime::UpcastObject<dyn crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient);
}

impl crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient
  for Client {
  fn BatchExecuteStatement(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchExecuteStatementInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchExecuteStatementOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::batch_execute_statement::_batch_execute_statement_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.batch_execute_statement()
        .set_statements(inner_input.statements)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_execute_statement::_batch_execute_statement_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_execute_statement::to_dafny_error)
}
 fn BatchGetItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::batch_get_item::_batch_get_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.batch_get_item()
        .set_request_items(inner_input.request_items)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_get_item::_batch_get_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_get_item::to_dafny_error)
}
 fn BatchWriteItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchWriteItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchWriteItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::batch_write_item::_batch_write_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.batch_write_item()
        .set_request_items(inner_input.request_items)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_return_item_collection_metrics(inner_input.return_item_collection_metrics)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_write_item::_batch_write_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::batch_write_item::to_dafny_error)
}
 fn CreateBackup(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateBackupInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateBackupOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::create_backup::_create_backup_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.create_backup()
        .set_table_name(inner_input.table_name)
.set_backup_name(inner_input.backup_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::create_backup::_create_backup_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::create_backup::to_dafny_error)
}
 fn CreateGlobalTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateGlobalTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateGlobalTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::create_global_table::_create_global_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.create_global_table()
        .set_global_table_name(inner_input.global_table_name)
.set_replication_group(inner_input.replication_group)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::create_global_table::_create_global_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::create_global_table::to_dafny_error)
}
 fn CreateTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::create_table::_create_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.create_table()
        .set_attribute_definitions(inner_input.attribute_definitions)
.set_table_name(inner_input.table_name)
.set_key_schema(inner_input.key_schema)
.set_local_secondary_indexes(inner_input.local_secondary_indexes)
.set_global_secondary_indexes(inner_input.global_secondary_indexes)
.set_billing_mode(inner_input.billing_mode)
.set_provisioned_throughput(inner_input.provisioned_throughput)
.set_stream_specification(inner_input.stream_specification)
.set_sse_specification(inner_input.sse_specification)
.set_tags(inner_input.tags)
.set_table_class(inner_input.table_class)
.set_deletion_protection_enabled(inner_input.deletion_protection_enabled)
.set_resource_policy(inner_input.resource_policy)
.set_on_demand_throughput(inner_input.on_demand_throughput)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::create_table::_create_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::create_table::to_dafny_error)
}
 fn DeleteBackup(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteBackupInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteBackupOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::delete_backup::_delete_backup_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.delete_backup()
        .set_backup_arn(inner_input.backup_arn)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_backup::_delete_backup_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_backup::to_dafny_error)
}
 fn DeleteItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::delete_item::_delete_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.delete_item()
        .set_table_name(inner_input.table_name)
.set_key(inner_input.key)
.set_expected(inner_input.expected)
.set_conditional_operator(inner_input.conditional_operator)
.set_return_values(inner_input.return_values)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_return_item_collection_metrics(inner_input.return_item_collection_metrics)
.set_condition_expression(inner_input.condition_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
.set_expression_attribute_values(inner_input.expression_attribute_values)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_item::_delete_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_item::to_dafny_error)
}
 fn DeleteResourcePolicy(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::delete_resource_policy::_delete_resource_policy_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.delete_resource_policy()
        .set_resource_arn(inner_input.resource_arn)
.set_expected_revision_id(inner_input.expected_revision_id)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_resource_policy::_delete_resource_policy_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_resource_policy::to_dafny_error)
}
 fn DeleteTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::delete_table::_delete_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.delete_table()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_table::_delete_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::delete_table::to_dafny_error)
}
 fn DescribeBackup(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeBackupInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeBackupOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_backup::_describe_backup_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_backup()
        .set_backup_arn(inner_input.backup_arn)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_backup::_describe_backup_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_backup::to_dafny_error)
}
 fn DescribeContinuousBackups(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContinuousBackupsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContinuousBackupsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_continuous_backups::_describe_continuous_backups_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_continuous_backups()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_continuous_backups::_describe_continuous_backups_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_continuous_backups::to_dafny_error)
}
 fn DescribeContributorInsights(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContributorInsightsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContributorInsightsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_contributor_insights::_describe_contributor_insights_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_contributor_insights()
        .set_table_name(inner_input.table_name)
.set_index_name(inner_input.index_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_contributor_insights::_describe_contributor_insights_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_contributor_insights::to_dafny_error)
}
 fn DescribeEndpoints(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsRequest>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsResponse>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_endpoints::_describe_endpoints_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_endpoints()

        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_endpoints::_describe_endpoints_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_endpoints::to_dafny_error)
}
 fn DescribeExport(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeExportInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeExportOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_export::_describe_export_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_export()
        .set_export_arn(inner_input.export_arn)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_export::_describe_export_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_export::to_dafny_error)
}
 fn DescribeGlobalTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table::_describe_global_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_global_table()
        .set_global_table_name(inner_input.global_table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table::_describe_global_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table::to_dafny_error)
}
 fn DescribeGlobalTableSettings(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableSettingsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableSettingsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table_settings::_describe_global_table_settings_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_global_table_settings()
        .set_global_table_name(inner_input.global_table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table_settings::_describe_global_table_settings_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_global_table_settings::to_dafny_error)
}
 fn DescribeImport(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeImportInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeImportOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_import::_describe_import_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_import()
        .set_import_arn(inner_input.import_arn)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_import::_describe_import_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_import::to_dafny_error)
}
 fn DescribeKinesisStreamingDestination(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeKinesisStreamingDestinationInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeKinesisStreamingDestinationOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_kinesis_streaming_destination()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_kinesis_streaming_destination::to_dafny_error)
}
 fn DescribeLimits(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeLimitsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeLimitsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_limits::_describe_limits_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_limits()

        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_limits::_describe_limits_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_limits::to_dafny_error)
}
 fn DescribeTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_table::_describe_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_table()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_table::_describe_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_table::to_dafny_error)
}
 fn DescribeTableReplicaAutoScaling(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableReplicaAutoScalingInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableReplicaAutoScalingOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_table_replica_auto_scaling()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_table_replica_auto_scaling::to_dafny_error)
}
 fn DescribeTimeToLive(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTimeToLiveInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTimeToLiveOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::describe_time_to_live::_describe_time_to_live_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.describe_time_to_live()
        .set_table_name(inner_input.table_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_time_to_live::_describe_time_to_live_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::describe_time_to_live::to_dafny_error)
}
 fn DisableKinesisStreamingDestination(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DisableKinesisStreamingDestinationInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DisableKinesisStreamingDestinationOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.disable_kinesis_streaming_destination()
        .set_table_name(inner_input.table_name)
.set_stream_arn(inner_input.stream_arn)
.set_enable_kinesis_streaming_configuration(inner_input.enable_kinesis_streaming_configuration)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::disable_kinesis_streaming_destination::to_dafny_error)
}
 fn EnableKinesisStreamingDestination(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::EnableKinesisStreamingDestinationInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::EnableKinesisStreamingDestinationOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.enable_kinesis_streaming_destination()
        .set_table_name(inner_input.table_name)
.set_stream_arn(inner_input.stream_arn)
.set_enable_kinesis_streaming_configuration(inner_input.enable_kinesis_streaming_configuration)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::enable_kinesis_streaming_destination::to_dafny_error)
}
 fn ExecuteStatement(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExecuteStatementInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExecuteStatementOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::execute_statement::_execute_statement_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.execute_statement()
        .set_statement(inner_input.statement)
.set_parameters(inner_input.parameters)
.set_consistent_read(inner_input.consistent_read)
.set_next_token(inner_input.next_token)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_limit(inner_input.limit)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::execute_statement::_execute_statement_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::execute_statement::to_dafny_error)
}
 fn ExecuteTransaction(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExecuteTransactionInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExecuteTransactionOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::execute_transaction::_execute_transaction_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.execute_transaction()
        .set_transact_statements(inner_input.transact_statements)
.set_client_request_token(inner_input.client_request_token)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::execute_transaction::_execute_transaction_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::execute_transaction::to_dafny_error)
}
 fn ExportTableToPointInTime(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportTableToPointInTimeInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportTableToPointInTimeOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::export_table_to_point_in_time::_export_table_to_point_in_time_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.export_table_to_point_in_time()
        .set_table_arn(inner_input.table_arn)
.set_export_time(inner_input.export_time)
.set_client_token(inner_input.client_token)
.set_s3_bucket(inner_input.s3_bucket)
.set_s3_bucket_owner(inner_input.s3_bucket_owner)
.set_s3_prefix(inner_input.s3_prefix)
.set_s3_sse_algorithm(inner_input.s3_sse_algorithm)
.set_s3_sse_kms_key_id(inner_input.s3_sse_kms_key_id)
.set_export_format(inner_input.export_format)
.set_export_type(inner_input.export_type)
.set_incremental_export_specification(inner_input.incremental_export_specification)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::export_table_to_point_in_time::_export_table_to_point_in_time_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::export_table_to_point_in_time::to_dafny_error)
}
 fn GetItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::get_item::_get_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.get_item()
        .set_table_name(inner_input.table_name)
.set_key(inner_input.key)
.set_attributes_to_get(inner_input.attributes_to_get)
.set_consistent_read(inner_input.consistent_read)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_projection_expression(inner_input.projection_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::get_item::_get_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::get_item::to_dafny_error)
}
 fn GetResourcePolicy(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetResourcePolicyInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetResourcePolicyOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::get_resource_policy::_get_resource_policy_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.get_resource_policy()
        .set_resource_arn(inner_input.resource_arn)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::get_resource_policy::_get_resource_policy_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::get_resource_policy::to_dafny_error)
}
 fn ImportTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::import_table::_import_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.import_table()
        .set_client_token(inner_input.client_token)
.set_s3_bucket_source(inner_input.s3_bucket_source)
.set_input_format(inner_input.input_format)
.set_input_format_options(inner_input.input_format_options)
.set_input_compression_type(inner_input.input_compression_type)
.set_table_creation_parameters(inner_input.table_creation_parameters)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::import_table::_import_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::import_table::to_dafny_error)
}
 fn ListBackups(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_backups::_list_backups_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_backups()
        .set_table_name(inner_input.table_name)
.set_limit(inner_input.limit)
.set_time_range_lower_bound(inner_input.time_range_lower_bound)
.set_time_range_upper_bound(inner_input.time_range_upper_bound)
.set_exclusive_start_backup_arn(inner_input.exclusive_start_backup_arn)
.set_backup_type(inner_input.backup_type)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_backups::_list_backups_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_backups::to_dafny_error)
}
 fn ListContributorInsights(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_contributor_insights::_list_contributor_insights_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_contributor_insights()
        .set_table_name(inner_input.table_name)
.set_next_token(inner_input.next_token)
.set_max_results(inner_input.max_results)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_contributor_insights::_list_contributor_insights_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_contributor_insights::to_dafny_error)
}
 fn ListExports(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListExportsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListExportsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_exports::_list_exports_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_exports()
        .set_table_arn(inner_input.table_arn)
.set_max_results(inner_input.max_results)
.set_next_token(inner_input.next_token)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_exports::_list_exports_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_exports::to_dafny_error)
}
 fn ListGlobalTables(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListGlobalTablesInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListGlobalTablesOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_global_tables::_list_global_tables_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_global_tables()
        .set_exclusive_start_global_table_name(inner_input.exclusive_start_global_table_name)
.set_limit(inner_input.limit)
.set_region_name(inner_input.region_name)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_global_tables::_list_global_tables_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_global_tables::to_dafny_error)
}
 fn ListImports(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListImportsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListImportsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_imports::_list_imports_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_imports()
        .set_table_arn(inner_input.table_arn)
.set_page_size(inner_input.page_size)
.set_next_token(inner_input.next_token)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_imports::_list_imports_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_imports::to_dafny_error)
}
 fn ListTables(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTablesInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTablesOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_tables::_list_tables_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_tables()
        .set_exclusive_start_table_name(inner_input.exclusive_start_table_name)
.set_limit(inner_input.limit)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_tables::_list_tables_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_tables::to_dafny_error)
}
 fn ListTagsOfResource(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTagsOfResourceInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTagsOfResourceOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::list_tags_of_resource::_list_tags_of_resource_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.list_tags_of_resource()
        .set_resource_arn(inner_input.resource_arn)
.set_next_token(inner_input.next_token)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::list_tags_of_resource::_list_tags_of_resource_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::list_tags_of_resource::to_dafny_error)
}
 fn PutItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::put_item::_put_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.put_item()
        .set_table_name(inner_input.table_name)
.set_item(inner_input.item)
.set_expected(inner_input.expected)
.set_return_values(inner_input.return_values)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_return_item_collection_metrics(inner_input.return_item_collection_metrics)
.set_conditional_operator(inner_input.conditional_operator)
.set_condition_expression(inner_input.condition_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
.set_expression_attribute_values(inner_input.expression_attribute_values)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::put_item::_put_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::put_item::to_dafny_error)
}
 fn PutResourcePolicy(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::put_resource_policy::_put_resource_policy_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.put_resource_policy()
        .set_resource_arn(inner_input.resource_arn)
.set_policy(inner_input.policy)
.set_expected_revision_id(inner_input.expected_revision_id)
.set_confirm_remove_self_resource_access(inner_input.confirm_remove_self_resource_access)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::put_resource_policy::_put_resource_policy_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::put_resource_policy::to_dafny_error)
}
 fn Query(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::query::_query_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.query()
        .set_table_name(inner_input.table_name)
.set_index_name(inner_input.index_name)
.set_select(inner_input.select)
.set_attributes_to_get(inner_input.attributes_to_get)
.set_limit(inner_input.limit)
.set_consistent_read(inner_input.consistent_read)
.set_key_conditions(inner_input.key_conditions)
.set_query_filter(inner_input.query_filter)
.set_conditional_operator(inner_input.conditional_operator)
.set_scan_index_forward(inner_input.scan_index_forward)
.set_exclusive_start_key(inner_input.exclusive_start_key)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_projection_expression(inner_input.projection_expression)
.set_filter_expression(inner_input.filter_expression)
.set_key_condition_expression(inner_input.key_condition_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
.set_expression_attribute_values(inner_input.expression_attribute_values)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::query::_query_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::query::to_dafny_error)
}
 fn RestoreTableFromBackup(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableFromBackupInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableFromBackupOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::restore_table_from_backup::_restore_table_from_backup_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.restore_table_from_backup()
        .set_target_table_name(inner_input.target_table_name)
.set_backup_arn(inner_input.backup_arn)
.set_billing_mode_override(inner_input.billing_mode_override)
.set_global_secondary_index_override(inner_input.global_secondary_index_override)
.set_local_secondary_index_override(inner_input.local_secondary_index_override)
.set_provisioned_throughput_override(inner_input.provisioned_throughput_override)
.set_on_demand_throughput_override(inner_input.on_demand_throughput_override)
.set_sse_specification_override(inner_input.sse_specification_override)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::restore_table_from_backup::_restore_table_from_backup_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::restore_table_from_backup::to_dafny_error)
}
 fn RestoreTableToPointInTime(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableToPointInTimeInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableToPointInTimeOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::restore_table_to_point_in_time::_restore_table_to_point_in_time_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.restore_table_to_point_in_time()
        .set_source_table_arn(inner_input.source_table_arn)
.set_source_table_name(inner_input.source_table_name)
.set_target_table_name(inner_input.target_table_name)
.set_use_latest_restorable_time(inner_input.use_latest_restorable_time)
.set_restore_date_time(inner_input.restore_date_time)
.set_billing_mode_override(inner_input.billing_mode_override)
.set_global_secondary_index_override(inner_input.global_secondary_index_override)
.set_local_secondary_index_override(inner_input.local_secondary_index_override)
.set_provisioned_throughput_override(inner_input.provisioned_throughput_override)
.set_on_demand_throughput_override(inner_input.on_demand_throughput_override)
.set_sse_specification_override(inner_input.sse_specification_override)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::restore_table_to_point_in_time::_restore_table_to_point_in_time_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::restore_table_to_point_in_time::to_dafny_error)
}
 fn Scan(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::scan::_scan_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.scan()
        .set_table_name(inner_input.table_name)
.set_index_name(inner_input.index_name)
.set_attributes_to_get(inner_input.attributes_to_get)
.set_limit(inner_input.limit)
.set_select(inner_input.select)
.set_scan_filter(inner_input.scan_filter)
.set_conditional_operator(inner_input.conditional_operator)
.set_exclusive_start_key(inner_input.exclusive_start_key)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_total_segments(inner_input.total_segments)
.set_segment(inner_input.segment)
.set_projection_expression(inner_input.projection_expression)
.set_filter_expression(inner_input.filter_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
.set_expression_attribute_values(inner_input.expression_attribute_values)
.set_consistent_read(inner_input.consistent_read)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::scan::_scan_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::scan::to_dafny_error)
}
 fn TagResource(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TagResourceInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    (),
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::tag_resource::_tag_resource_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.tag_resource()
        .set_resource_arn(inner_input.resource_arn)
.set_tags(inner_input.tags)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    |x| (),
    crate::deps::com_amazonaws_dynamodb::conversions::tag_resource::to_dafny_error)
}
 fn TransactGetItems(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::transact_get_items::_transact_get_items_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.transact_get_items()
        .set_transact_items(inner_input.transact_items)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::transact_get_items::_transact_get_items_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::transact_get_items::to_dafny_error)
}
 fn TransactWriteItems(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::transact_write_items::_transact_write_items_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.transact_write_items()
        .set_transact_items(inner_input.transact_items)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_return_item_collection_metrics(inner_input.return_item_collection_metrics)
.set_client_request_token(inner_input.client_request_token)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::transact_write_items::_transact_write_items_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::transact_write_items::to_dafny_error)
}
 fn UntagResource(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UntagResourceInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    (),
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::untag_resource::_untag_resource_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.untag_resource()
        .set_resource_arn(inner_input.resource_arn)
.set_tag_keys(inner_input.tag_keys)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    |x| (),
    crate::deps::com_amazonaws_dynamodb::conversions::untag_resource::to_dafny_error)
}
 fn UpdateContinuousBackups(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContinuousBackupsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContinuousBackupsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_continuous_backups::_update_continuous_backups_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_continuous_backups()
        .set_table_name(inner_input.table_name)
.set_point_in_time_recovery_specification(inner_input.point_in_time_recovery_specification)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_continuous_backups::_update_continuous_backups_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_continuous_backups::to_dafny_error)
}
 fn UpdateContributorInsights(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContributorInsightsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContributorInsightsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_contributor_insights::_update_contributor_insights_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_contributor_insights()
        .set_table_name(inner_input.table_name)
.set_index_name(inner_input.index_name)
.set_contributor_insights_action(inner_input.contributor_insights_action)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_contributor_insights::_update_contributor_insights_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_contributor_insights::to_dafny_error)
}
 fn UpdateGlobalTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_global_table::_update_global_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_global_table()
        .set_global_table_name(inner_input.global_table_name)
.set_replica_updates(inner_input.replica_updates)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_global_table::_update_global_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_global_table::to_dafny_error)
}
 fn UpdateGlobalTableSettings(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableSettingsInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableSettingsOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_global_table_settings::_update_global_table_settings_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_global_table_settings()
        .set_global_table_name(inner_input.global_table_name)
.set_global_table_billing_mode(inner_input.global_table_billing_mode)
.set_global_table_provisioned_write_capacity_units(inner_input.global_table_provisioned_write_capacity_units)
.set_global_table_provisioned_write_capacity_auto_scaling_settings_update(inner_input.global_table_provisioned_write_capacity_auto_scaling_settings_update)
.set_global_table_global_secondary_index_settings_update(inner_input.global_table_global_secondary_index_settings_update)
.set_replica_settings_update(inner_input.replica_settings_update)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_global_table_settings::_update_global_table_settings_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_global_table_settings::to_dafny_error)
}
 fn UpdateItem(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_item::_update_item_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_item()
        .set_table_name(inner_input.table_name)
.set_key(inner_input.key)
.set_attribute_updates(inner_input.attribute_updates)
.set_expected(inner_input.expected)
.set_conditional_operator(inner_input.conditional_operator)
.set_return_values(inner_input.return_values)
.set_return_consumed_capacity(inner_input.return_consumed_capacity)
.set_return_item_collection_metrics(inner_input.return_item_collection_metrics)
.set_update_expression(inner_input.update_expression)
.set_condition_expression(inner_input.condition_expression)
.set_expression_attribute_names(inner_input.expression_attribute_names)
.set_expression_attribute_values(inner_input.expression_attribute_values)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_item::_update_item_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_item::to_dafny_error)
}
 fn UpdateKinesisStreamingDestination(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateKinesisStreamingDestinationInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateKinesisStreamingDestinationOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_kinesis_streaming_destination::_update_kinesis_streaming_destination_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_kinesis_streaming_destination()
        .set_table_name(inner_input.table_name)
.set_stream_arn(inner_input.stream_arn)
.set_update_kinesis_streaming_configuration(inner_input.update_kinesis_streaming_configuration)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_kinesis_streaming_destination::_update_kinesis_streaming_destination_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_kinesis_streaming_destination::to_dafny_error)
}
 fn UpdateTable(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_table::_update_table_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_table()
        .set_attribute_definitions(inner_input.attribute_definitions)
.set_table_name(inner_input.table_name)
.set_billing_mode(inner_input.billing_mode)
.set_provisioned_throughput(inner_input.provisioned_throughput)
.set_global_secondary_index_updates(inner_input.global_secondary_index_updates)
.set_stream_specification(inner_input.stream_specification)
.set_sse_specification(inner_input.sse_specification)
.set_replica_updates(inner_input.replica_updates)
.set_table_class(inner_input.table_class)
.set_deletion_protection_enabled(inner_input.deletion_protection_enabled)
.set_on_demand_throughput(inner_input.on_demand_throughput)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_table::_update_table_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_table::to_dafny_error)
}
 fn UpdateTableReplicaAutoScaling(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableReplicaAutoScalingInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableReplicaAutoScalingOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_table_replica_auto_scaling()
        .set_global_secondary_index_updates(inner_input.global_secondary_index_updates)
.set_table_name(inner_input.table_name)
.set_provisioned_write_capacity_auto_scaling_update(inner_input.provisioned_write_capacity_auto_scaling_update)
.set_replica_updates(inner_input.replica_updates)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_table_replica_auto_scaling::to_dafny_error)
}
 fn UpdateTimeToLive(&self, input: &std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTimeToLiveInput>)
  -> std::rc::Rc<crate::r#_Wrappers_Compile::Result<
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTimeToLiveOutput>,
    std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
  >
> {
  let inner_input = crate::deps::com_amazonaws_dynamodb::conversions::update_time_to_live::_update_time_to_live_request::from_dafny(input.clone());
  let native_result = tokio::task::block_in_place(|| {
    dafny_tokio_runtime.block_on(async {
      self.inner.update_time_to_live()
        .set_table_name(inner_input.table_name)
.set_time_to_live_specification(inner_input.time_to_live_specification)
        .send()
        .await
      })
    });
  crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_dynamodb::conversions::update_time_to_live::_update_time_to_live_response::to_dafny,
    crate::deps::com_amazonaws_dynamodb::conversions::update_time_to_live::to_dafny_error)
}
} 
