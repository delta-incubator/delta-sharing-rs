// @generated by protoc-gen-es v2.2.3 with parameter "target=ts,json_types=true"
// @generated from file delta_sharing/recipients/v1/svc.proto (package delta_sharing.recipients.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import { file_buf_validate_validate } from "../../../buf/validate/validate_pb";
import type { AuthenticationType, AuthenticationTypeJson, RecipientInfo, RecipientInfoJson, RecipientInfoSchema } from "./models_pb";
import { file_delta_sharing_recipients_v1_models } from "./models_pb";
import { file_gnostic_openapi_v3_annotations } from "../../../gnostic/openapi/v3/annotations_pb";
import { file_google_api_annotations } from "../../../google/api/annotations_pb";
import { file_google_api_field_behavior } from "../../../google/api/field_behavior_pb";
import type { EmptySchema, StructJson } from "@bufbuild/protobuf/wkt";
import { file_google_protobuf_empty, file_google_protobuf_struct } from "@bufbuild/protobuf/wkt";
import type { JsonObject, Message } from "@bufbuild/protobuf";

/**
 * Describes the file delta_sharing/recipients/v1/svc.proto.
 */
export const file_delta_sharing_recipients_v1_svc: GenFile = /*@__PURE__*/
  fileDesc("CiVkZWx0YV9zaGFyaW5nL3JlY2lwaWVudHMvdjEvc3ZjLnByb3RvEhtkZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjEifQoVTGlzdFJlY2lwaWVudHNSZXF1ZXN0EicKC21heF9yZXN1bHRzGAEgASgFQg3gQQG6SAcaBRDoByAASACIAQESHAoKcGFnZV90b2tlbhgCIAEoCUID4EEBSAGIAQFCDgoMX21heF9yZXN1bHRzQg0KC19wYWdlX3Rva2VuIooBChZMaXN0UmVjaXBpZW50c1Jlc3BvbnNlEj4KCnJlY2lwaWVudHMYASADKAsyKi5kZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjEuUmVjaXBpZW50SW5mbxIcCg9uZXh0X3BhZ2VfdG9rZW4YAiABKAlIAIgBAUISChBfbmV4dF9wYWdlX3Rva2VuItcCChZDcmVhdGVSZWNpcGllbnRSZXF1ZXN0EjIKBG5hbWUYASABKAlCJOBBArpIHnIcMhpeW2Etel1bMC05YS16Ll9dKlswLTlhLXpdJBJRChNhdXRoZW50aWNhdGlvbl90eXBlGAIgASgOMi8uZGVsdGFfc2hhcmluZy5yZWNpcGllbnRzLnYxLkF1dGhlbnRpY2F0aW9uVHlwZUID4EECEhIKBW93bmVyGAMgASgJQgPgQQESGQoHY29tbWVudBgEIAEoCUID4EEBSACIAQESNQoKcHJvcGVydGllcxgFIAEoCzIXLmdvb2dsZS5wcm90b2J1Zi5TdHJ1Y3RCA+BBAUgBiAEBEiEKD2V4cGlyYXRpb25fdGltZRgGIAEoA0ID4EEBSAKIAQFCCgoIX2NvbW1lbnRCDQoLX3Byb3BlcnRpZXNCEgoQX2V4cGlyYXRpb25fdGltZSJJChNHZXRSZWNpcGllbnRSZXF1ZXN0EjIKBG5hbWUYASABKAlCJOBBArpIHnIcMhpeW2Etel1bMC05YS16Ll9dKlswLTlhLXpdJCLdAgoWVXBkYXRlUmVjaXBpZW50UmVxdWVzdBIyCgRuYW1lGAEgASgJQiTgQQK6SB5yHDIaXlthLXpdWzAtOWEtei5fXSpbMC05YS16XSQSOwoIbmV3X25hbWUYAiABKAlCJOBBAbpIHnIcMhpeW2Etel1bMC05YS16Ll9dKlswLTlhLXpdJEgAiAEBEhcKBW93bmVyGAMgASgJQgPgQQFIAYgBARIZCgdjb21tZW50GAQgASgJQgPgQQFIAogBARI1Cgpwcm9wZXJ0aWVzGAUgASgLMhcuZ29vZ2xlLnByb3RvYnVmLlN0cnVjdEID4EEBSAOIAQESIQoPZXhwaXJhdGlvbl90aW1lGAYgASgDQgPgQQFIBIgBAUILCglfbmV3X25hbWVCCAoGX293bmVyQgoKCF9jb21tZW50Qg0KC19wcm9wZXJ0aWVzQhIKEF9leHBpcmF0aW9uX3RpbWUiTAoWRGVsZXRlUmVjaXBpZW50UmVxdWVzdBIyCgRuYW1lGAEgASgJQiTgQQK6SB5yHDIaXlthLXpdWzAtOWEtei5fXSpbMC05YS16XSQyrQYKEVJlY2lwaWVudHNTZXJ2aWNlEqEBCg5MaXN0UmVjaXBpZW50cxIyLmRlbHRhX3NoYXJpbmcucmVjaXBpZW50cy52MS5MaXN0UmVjaXBpZW50c1JlcXVlc3QaMy5kZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjEuTGlzdFJlY2lwaWVudHNSZXNwb25zZSImukcQKg5MaXN0UmVjaXBpZW50c4LT5JMCDRILL3JlY2lwaWVudHMSngEKD0NyZWF0ZVJlY2lwaWVudBIzLmRlbHRhX3NoYXJpbmcucmVjaXBpZW50cy52MS5DcmVhdGVSZWNpcGllbnRSZXF1ZXN0GiouZGVsdGFfc2hhcmluZy5yZWNpcGllbnRzLnYxLlJlY2lwaWVudEluZm8iKrpHESoPQ3JlYXRlUmVjaXBpZW50gtPkkwIQOgEqIgsvcmVjaXBpZW50cxKZAQoMR2V0UmVjaXBpZW50EjAuZGVsdGFfc2hhcmluZy5yZWNpcGllbnRzLnYxLkdldFJlY2lwaWVudFJlcXVlc3QaKi5kZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjEuUmVjaXBpZW50SW5mbyIrukcOKgxHZXRSZWNpcGllbnSC0+STAhQSEi9yZWNpcGllbnRzL3tuYW1lfRKlAQoPVXBkYXRlUmVjaXBpZW50EjMuZGVsdGFfc2hhcmluZy5yZWNpcGllbnRzLnYxLlVwZGF0ZVJlY2lwaWVudFJlcXVlc3QaKi5kZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjEuUmVjaXBpZW50SW5mbyIxukcRKg9VcGRhdGVSZWNpcGllbnSC0+STAhc6ASoyEi9yZWNpcGllbnRzL3tuYW1lfRKOAQoPRGVsZXRlUmVjaXBpZW50EjMuZGVsdGFfc2hhcmluZy5yZWNpcGllbnRzLnYxLkRlbGV0ZVJlY2lwaWVudFJlcXVlc3QaFi5nb29nbGUucHJvdG9idWYuRW1wdHkiLrpHESoPRGVsZXRlUmVjaXBpZW50gtPkkwIUKhIvcmVjaXBpZW50cy97bmFtZX1CjgIKH2NvbS5kZWx0YV9zaGFyaW5nLnJlY2lwaWVudHMudjFCCFN2Y1Byb3RvUAFaV2dpdGh1Yi5jb20vZGVsdGEtaW5jdWJhdG9yL2RlbHRhLXNoYXJpbmctcnMvZ28vZGVsdGFfc2hhcmluZy9yZWNpcGllbnRzL3YxO3JlY2lwaWVudHN2MaICA0RSWKoCGkRlbHRhU2hhcmluZy5SZWNpcGllbnRzLlYxygIaRGVsdGFTaGFyaW5nXFJlY2lwaWVudHNcVjHiAiZEZWx0YVNoYXJpbmdcUmVjaXBpZW50c1xWMVxHUEJNZXRhZGF0YeoCHERlbHRhU2hhcmluZzo6UmVjaXBpZW50czo6VjFiBnByb3RvMw", [file_buf_validate_validate, file_delta_sharing_recipients_v1_models, file_gnostic_openapi_v3_annotations, file_google_api_annotations, file_google_api_field_behavior, file_google_protobuf_empty, file_google_protobuf_struct]);

/**
 * Request to list recipients.
 *
 * @generated from message delta_sharing.recipients.v1.ListRecipientsRequest
 */
export type ListRecipientsRequest = Message<"delta_sharing.recipients.v1.ListRecipientsRequest"> & {
  /**
   * The maximum number of results per page that should be returned.
   *
   * @generated from field: optional int32 max_results = 1;
   */
  maxResults?: number;

  /**
   * Opaque pagination token to go to next page based on previous query.
   *
   * @generated from field: optional string page_token = 2;
   */
  pageToken?: string;
};

/**
 * Request to list recipients.
 *
 * @generated from message delta_sharing.recipients.v1.ListRecipientsRequest
 */
export type ListRecipientsRequestJson = {
  /**
   * The maximum number of results per page that should be returned.
   *
   * @generated from field: optional int32 max_results = 1;
   */
  maxResults?: number;

  /**
   * Opaque pagination token to go to next page based on previous query.
   *
   * @generated from field: optional string page_token = 2;
   */
  pageToken?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.ListRecipientsRequest.
 * Use `create(ListRecipientsRequestSchema)` to create a new message.
 */
export const ListRecipientsRequestSchema: GenMessage<ListRecipientsRequest, ListRecipientsRequestJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 0);

/**
 * Response to list recipients.
 *
 * @generated from message delta_sharing.recipients.v1.ListRecipientsResponse
 */
export type ListRecipientsResponse = Message<"delta_sharing.recipients.v1.ListRecipientsResponse"> & {
  /**
   * List of recipients.
   *
   * @generated from field: repeated delta_sharing.recipients.v1.RecipientInfo recipients = 1;
   */
  recipients: RecipientInfo[];

  /**
   * Opaque pagination token to go to next page based on previous query.
   *
   * @generated from field: optional string next_page_token = 2;
   */
  nextPageToken?: string;
};

/**
 * Response to list recipients.
 *
 * @generated from message delta_sharing.recipients.v1.ListRecipientsResponse
 */
export type ListRecipientsResponseJson = {
  /**
   * List of recipients.
   *
   * @generated from field: repeated delta_sharing.recipients.v1.RecipientInfo recipients = 1;
   */
  recipients?: RecipientInfoJson[];

  /**
   * Opaque pagination token to go to next page based on previous query.
   *
   * @generated from field: optional string next_page_token = 2;
   */
  nextPageToken?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.ListRecipientsResponse.
 * Use `create(ListRecipientsResponseSchema)` to create a new message.
 */
export const ListRecipientsResponseSchema: GenMessage<ListRecipientsResponse, ListRecipientsResponseJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 1);

/**
 * Creates a new recipient
 *
 * @generated from message delta_sharing.recipients.v1.CreateRecipientRequest
 */
export type CreateRecipientRequest = Message<"delta_sharing.recipients.v1.CreateRecipientRequest"> & {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name: string;

  /**
   * The delta sharing authentication type.
   *
   * @generated from field: delta_sharing.recipients.v1.AuthenticationType authentication_type = 2;
   */
  authenticationType: AuthenticationType;

  /**
   * Username of the recipient owner.
   *
   * @generated from field: string owner = 3;
   */
  owner: string;

  /**
   * Description about the recipient.
   *
   * @generated from field: optional string comment = 4;
   */
  comment?: string;

  /**
   * Recipient properties as map of string key-value pairs.
   *
   * When provided in update request, the specified properties will override the existing properties.
   * To add and remove properties, one would need to perform a read-modify-write.
   *
   * @generated from field: optional google.protobuf.Struct properties = 5;
   */
  properties?: JsonObject;

  /**
   * Expiration timestamp of the token, in epoch milliseconds.
   *
   * @generated from field: optional int64 expiration_time = 6;
   */
  expirationTime?: bigint;
};

/**
 * Creates a new recipient
 *
 * @generated from message delta_sharing.recipients.v1.CreateRecipientRequest
 */
export type CreateRecipientRequestJson = {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name?: string;

  /**
   * The delta sharing authentication type.
   *
   * @generated from field: delta_sharing.recipients.v1.AuthenticationType authentication_type = 2;
   */
  authenticationType?: AuthenticationTypeJson;

  /**
   * Username of the recipient owner.
   *
   * @generated from field: string owner = 3;
   */
  owner?: string;

  /**
   * Description about the recipient.
   *
   * @generated from field: optional string comment = 4;
   */
  comment?: string;

  /**
   * Recipient properties as map of string key-value pairs.
   *
   * When provided in update request, the specified properties will override the existing properties.
   * To add and remove properties, one would need to perform a read-modify-write.
   *
   * @generated from field: optional google.protobuf.Struct properties = 5;
   */
  properties?: StructJson;

  /**
   * Expiration timestamp of the token, in epoch milliseconds.
   *
   * @generated from field: optional int64 expiration_time = 6;
   */
  expirationTime?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.CreateRecipientRequest.
 * Use `create(CreateRecipientRequestSchema)` to create a new message.
 */
export const CreateRecipientRequestSchema: GenMessage<CreateRecipientRequest, CreateRecipientRequestJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 2);

/**
 * Get a recipient by name.
 *
 * @generated from message delta_sharing.recipients.v1.GetRecipientRequest
 */
export type GetRecipientRequest = Message<"delta_sharing.recipients.v1.GetRecipientRequest"> & {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name: string;
};

/**
 * Get a recipient by name.
 *
 * @generated from message delta_sharing.recipients.v1.GetRecipientRequest
 */
export type GetRecipientRequestJson = {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.GetRecipientRequest.
 * Use `create(GetRecipientRequestSchema)` to create a new message.
 */
export const GetRecipientRequestSchema: GenMessage<GetRecipientRequest, GetRecipientRequestJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 3);

/**
 * Update a recipient
 *
 * @generated from message delta_sharing.recipients.v1.UpdateRecipientRequest
 */
export type UpdateRecipientRequest = Message<"delta_sharing.recipients.v1.UpdateRecipientRequest"> & {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name: string;

  /**
   * New name for the recipient
   *
   * @generated from field: optional string new_name = 2;
   */
  newName?: string;

  /**
   * Username of the recipient owner.
   *
   * @generated from field: optional string owner = 3;
   */
  owner?: string;

  /**
   * Description about the recipient.
   *
   * @generated from field: optional string comment = 4;
   */
  comment?: string;

  /**
   * Recipient properties as map of string key-value pairs.
   *
   * When provided in update request, the specified properties will override the existing properties.
   * To add and remove properties, one would need to perform a read-modify-write.
   *
   * @generated from field: optional google.protobuf.Struct properties = 5;
   */
  properties?: JsonObject;

  /**
   * Expiration timestamp of the token, in epoch milliseconds.
   *
   * @generated from field: optional int64 expiration_time = 6;
   */
  expirationTime?: bigint;
};

/**
 * Update a recipient
 *
 * @generated from message delta_sharing.recipients.v1.UpdateRecipientRequest
 */
export type UpdateRecipientRequestJson = {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name?: string;

  /**
   * New name for the recipient
   *
   * @generated from field: optional string new_name = 2;
   */
  newName?: string;

  /**
   * Username of the recipient owner.
   *
   * @generated from field: optional string owner = 3;
   */
  owner?: string;

  /**
   * Description about the recipient.
   *
   * @generated from field: optional string comment = 4;
   */
  comment?: string;

  /**
   * Recipient properties as map of string key-value pairs.
   *
   * When provided in update request, the specified properties will override the existing properties.
   * To add and remove properties, one would need to perform a read-modify-write.
   *
   * @generated from field: optional google.protobuf.Struct properties = 5;
   */
  properties?: StructJson;

  /**
   * Expiration timestamp of the token, in epoch milliseconds.
   *
   * @generated from field: optional int64 expiration_time = 6;
   */
  expirationTime?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.UpdateRecipientRequest.
 * Use `create(UpdateRecipientRequestSchema)` to create a new message.
 */
export const UpdateRecipientRequestSchema: GenMessage<UpdateRecipientRequest, UpdateRecipientRequestJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 4);

/**
 * Delete a recipient
 *
 * @generated from message delta_sharing.recipients.v1.DeleteRecipientRequest
 */
export type DeleteRecipientRequest = Message<"delta_sharing.recipients.v1.DeleteRecipientRequest"> & {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name: string;
};

/**
 * Delete a recipient
 *
 * @generated from message delta_sharing.recipients.v1.DeleteRecipientRequest
 */
export type DeleteRecipientRequestJson = {
  /**
   * Name of the recipient.
   *
   * @generated from field: string name = 1;
   */
  name?: string;
};

/**
 * Describes the message delta_sharing.recipients.v1.DeleteRecipientRequest.
 * Use `create(DeleteRecipientRequestSchema)` to create a new message.
 */
export const DeleteRecipientRequestSchema: GenMessage<DeleteRecipientRequest, DeleteRecipientRequestJson> = /*@__PURE__*/
  messageDesc(file_delta_sharing_recipients_v1_svc, 5);

/**
 * Recipients
 *
 * A recipient is an object you create using recipients/create to represent an organization which
 * you want to allow access shares. when you create a recipient object, Unity Catalog generates an
 * activation link you can send to the recipient. The recipient follows the activation link to download
 * the credential file, and then uses the credential file to establish a secure connection to receive
 * the shared data. This sharing mode is called open sharing.
 *
 * @generated from service delta_sharing.recipients.v1.RecipientsService
 */
export const RecipientsService: GenService<{
  /**
   * List recipients.
   *
   * @generated from rpc delta_sharing.recipients.v1.RecipientsService.ListRecipients
   */
  listRecipients: {
    methodKind: "unary";
    input: typeof ListRecipientsRequestSchema;
    output: typeof ListRecipientsResponseSchema;
  },
  /**
   * Create a new recipient.
   *
   * @generated from rpc delta_sharing.recipients.v1.RecipientsService.CreateRecipient
   */
  createRecipient: {
    methodKind: "unary";
    input: typeof CreateRecipientRequestSchema;
    output: typeof RecipientInfoSchema;
  },
  /**
   * Get a recipient by name.
   *
   * @generated from rpc delta_sharing.recipients.v1.RecipientsService.GetRecipient
   */
  getRecipient: {
    methodKind: "unary";
    input: typeof GetRecipientRequestSchema;
    output: typeof RecipientInfoSchema;
  },
  /**
   * Update a recipient.
   *
   * @generated from rpc delta_sharing.recipients.v1.RecipientsService.UpdateRecipient
   */
  updateRecipient: {
    methodKind: "unary";
    input: typeof UpdateRecipientRequestSchema;
    output: typeof RecipientInfoSchema;
  },
  /**
   * Delete a recipient.
   *
   * @generated from rpc delta_sharing.recipients.v1.RecipientsService.DeleteRecipient
   */
  deleteRecipient: {
    methodKind: "unary";
    input: typeof DeleteRecipientRequestSchema;
    output: typeof EmptySchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_delta_sharing_recipients_v1_svc, 0);

