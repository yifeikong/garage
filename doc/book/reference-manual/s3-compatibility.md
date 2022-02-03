+++
title = "S3 Compatibility status"
weight = 20
+++

## Endpoint implementation

All APIs that are missing on Garage will return a 501 Not Implemented.

*The compatibility list for other platforms is given only for information purposes and based on available documentation. Some entries might be inexact. Feel free to open a PR to fix this table.*

### Signatures

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [s3v2](https://docs.aws.amazon.com/general/latest/gr/signature-version-2.html) (deprecated) | ❌Missing | ✅ |
| [s3v4](https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html) |  ✅ Implemented | ✅| 

### URL style

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [path-style](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#path-style-access) (eg. `host.tld/bucket/key`) |  ✅ Implemented | ✅ |
| [vhost-style](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#virtual-hosted-style-access) URL (eg. `bucket.host.tld/key`) |  ✅ Implemented | ❌|

### Core endoints

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [CreateBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html)                 | ✅ Implemented                      | ✅ |
| [DeleteBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html)                 | ✅ Implemented                      | ✅ | 
| [GetBucketLocation](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLocation.html)            | ✅ Implemented                      |  ✅  |
| [HeadBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html)                   | ✅ Implemented                      | ✅ |
| [ListBuckets](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html)                  | ✅ Implemented                      | ❌|
| [HeadObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html)                   | ✅ Implemented                      | ✅ |
| [CopyObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html)                   | ✅ Implemented                      |  ✅ |
| [DeleteObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html)                 | ✅ Implemented                      | ✅ |
| [DeleteObjects](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjects.html)                | ✅ Implemented                      |  ✅  |
| [GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)                    | ✅ Implemented                      |  ✅ | 
| [ListObjects](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjects.html)                  | ✅ Implemented (implementation details below)   | ✅ | 
| [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html)                | ✅ Implemented                      | ❌|
| [PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)                    | ✅ Implemented                      | ✅ | 

### Multipart Upload endpoints

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [AbortMultipartUpload](https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html)         | ✅ Implemented                      |  ✅ |
| [CompleteMultipartUpload](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html)      | ✅ Implemented                      | ✅ |
| [CreateMultipartUpload](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html)        | ✅ Implemented                      | ✅|
| [ListMultipartUpload](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUpload.html)          | ✅ Implemented                      | ✅ | 
| [ListParts](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html)                    | ✅ Implemented                      | ✅ | 
| [UploadPart](https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html)                   | ✅ Implemented                      | ✅ | 
| [UploadPartCopy](https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPartCopy.html)               | ✅ Implemented                      | ✅ | 

### Website endpoints

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketWebsite](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketWebsite.html)          | ✅ Implemented                      | ❌|
| [GetBucketWebsite](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketWebsite.html)             | ✅ Implemented                      |  ❌ |
| [PutBucketWebsite](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketWebsite.html)             | ⚠ Partially implemented (see below)| ❌|
| [DeleteBucketCors](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketCors.html)             | ✅ Implemented                      |  ❌|
| [GetBucketCors](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketCors.html)                | ✅ Implemented                      |  ❌ |
| [PutBucketCors](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketCors.html)                | ✅ Implemented                      | ❌|


### ACL, Policies endpoints

Amazon has 2 access control mechanisms in S3: ACL (legacy) and policies (new one).
Garage implements none of them, and has its own system instead, built around a per-access-key-per-bucket logic.
See Garage CLI reference manual to learn how to use Garage's permission system.

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketPolicy](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketPolicy.html) | ❌Missing | ❌|
| [GetBucketPolicy](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketPolicy.html) | ❌Missing | ❌|
| [GetBucketPolicyStatus](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketPolicyStatus.html) | ❌Missing | ❌|
| [PutBucketPolicy](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketPolicy.html) | ❌Missing | ❌|
| [GetBucketAcl](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAcl.html) | ❌Missing | ✅ | 
| [PutBucketAcl](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAcl.html) | ❌Missing | ✅ |
| [GetObjectAcl](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html) | ❌Missing | ✅ | 
| [PutObjectAcl](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectAcl.html) | ❌Missing | ✅ | 

### Versioning, Lifecycle endpoints

Garage does not support (yet) object versioning.
If you need this feature, please [share your use case in our dedicated issue](https://git.deuxfleurs.fr/Deuxfleurs/garage/issues/166).

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketLifecycle](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketLifecycle.html) | ❌Missing | ❌|
| [GetBucketLifecycleConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycleConfiguration.html) | ❌Missing | ❌|
| [PutBucketLifecycleConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html) | ❌Missing | ❌|
| [GetBucketVersioning](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html)          | ❌ Stub (see below)                 | ✅ |
| [ListObjectVersions](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectVersions.html) | ❌Missing | ❌|
| [PutBucketVersioning](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketVersioning.html) | ❌Missing | ❌|
 
### Replication endpoints

Please open an issue if you have a use case for replication.

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketReplication](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketReplication.html) | ❌Missing | ❌|
| [GetBucketReplication](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketReplication.html) | ❌Missing | ❌|
| [PutBucketReplication](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketReplication.html) | ❌Missing | ❌|

### (Server-side) encryption

We think that you can either encrypt your server partition or do client-side encryption, so we did not implement server-side encryption for Garage.
Please open an issue if you have a use case.

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketEncryption](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketEncryption.html) | ❌Missing | ❌|
| [GetBucketEncryption](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketEncryption.html) | ❌Missing | ❌|
| [PutBucketEncryption](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketEncryption.html) | ❌Missing | ❌|

### Amazon specific endpoints

| Endpoint                     | Garage                           | [Openstack Swift](https://docs.openstack.org/swift/latest/s3_compat.html) | [Ceph Object Gateway](https://docs.ceph.com/en/latest/radosgw/s3/) | [Riak CS](https://docs.riak.com/riak/cs/2.1.1/references/apis/storage/s3/index.html) |  
|------------------------------|----------------------------------|-----------------|---------------|---------|
| [DeleteBucketAnalyticsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketAnalyticsConfiguration.html) | ❌Missing | ❌|
| [DeleteBucketIntelligentTieringConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketIntelligentTieringConfiguration.html) | ❌Missing | ❌|
| [DeleteBucketInventoryConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketInventoryConfiguration.html) | ❌Missing | ❌|
| [DeleteBucketMetricsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketMetricsConfiguration.html) | ❌Missing | ❌|
| [DeleteBucketOwnershipControls](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketOwnershipControls.html) | ❌Missing | ❌|
| [DeleteBucketTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketTagging.html) | ❌Missing | ❌|
| [DeleteObjectTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjectTagging.html) | ❌Missing | ❌|
| [DeletePublicAccessBlock](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html) | ❌Missing | ❌|
| [GetBucketAccelerateConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAccelerateConfiguration.html) | ❌Missing | ❌|
| [GetBucketAnalyticsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketAnalyticsConfiguration.html) | ❌Missing | ❌|
| [GetBucketIntelligentTieringConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html) | ❌Missing | ❌|
| [GetBucketInventoryConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketInventoryConfiguration.html) | ❌Missing | ❌|
| [GetBucketLogging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLogging.html) | ❌Missing | ❌|
| [GetBucketMetricsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketMetricsConfiguration.html) | ❌Missing | ❌|
| [GetBucketNotificationConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketNotificationConfiguration.html) | ❌Missing | ❌|
| [GetBucketOwnershipControls](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketOwnershipControls.html) | ❌Missing | ❌|
| [GetBucketRequestPayment](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketRequestPayment.html) | ❌Missing | ❌|
| [GetBucketTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketTagging.html) | ❌Missing | ❌|
| [GetObjectLegalHold](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLegalHold.html) | ❌Missing | ❌|
| [GetObjectLockConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLockConfiguration.html) | ❌Missing | ❌|
| [GetObjectRetention](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectRetention.html) | ❌Missing | ❌|
| [GetObjectTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html) | ❌Missing | ❌|
| [GetObjectTorrent](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTorrent.html) | ❌Missing | ❌|
| [GetPublicAccessBlock](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html) | ❌Missing | ❌|
| [ListBucketAnalyticsConfigurations](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketAnalyticsConfigurations.html) | ❌Missing | ❌|
| [ListBucketIntelligentTieringConfigurations](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketIntelligentTieringConfigurations.html) | ❌Missing | ❌|
| [ListBucketInventoryConfigurations](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketInventoryConfigurations.html) | ❌Missing | ❌|
| [ListBucketMetricsConfigurations](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketMetricsConfigurations.html) | ❌Missing | ❌|
| [PutBucketAccelerateConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAccelerateConfiguration.html) | ❌Missing | ❌|
| [PutBucketAnalyticsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAnalyticsConfiguration.html) | ❌Missing | ❌|
| [PutBucketIntelligentTieringConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html) | ❌Missing | ❌|
| [PutBucketInventoryConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketInventoryConfiguration.html) | ❌Missing | ❌|
| [PutBucketLogging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLogging.html) | ❌Missing | ❌|
| [PutBucketMetricsConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html) | ❌Missing | ❌|
| [PutBucketNotificationConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketNotificationConfiguration.html) | ❌Missing | ❌|
| [PutBucketOwnershipControls](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketOwnershipControls.html) | ❌Missing | ❌|
| [PutBucketRequestPayment](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketRequestPayment.html) | ❌Missing | ❌|
| [PutBucketTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketTagging.html) | ❌Missing | ❌|
| [PutObjectLegalHold](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectLegalHold.html) | ❌Missing | ❌|
| [PutObjectLockConfiguration](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectLockConfiguration.html) | ❌Missing | ❌|
| [PutObjectRetention](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectRetention.html) | ❌Missing | ❌|
| [PutObjectTagging](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectTagging.html) | ❌Missing | ❌|
| [PutPublicAccessBlock](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutPublicAccessBlock.html) | ❌Missing | ❌|
| [RestoreObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_RestoreObject.html) | ❌Missing | ❌|
| [SelectObjectContent](https://docs.aws.amazon.com/AmazonS3/latest/API/API_SelectObjectContent.html) | ❌Missing | ❌|

## Implementation details

Most `x-amz-` headers are not implemented.

### Endpoint specific details

- **GetBucketVersioning:** Stub implementation (Garage does not yet support versionning so this always returns
"versionning not enabled").

- **ListObjects:** Implemented, but there isn't a very good specification of what `encoding-type=url` covers so there might be some encoding bugs. In our implementation the url-encoded fields are in the same in ListObjects as they are in ListObjectsV2.

- **PutBucketWebsite:** Implemented, but only stores the index document suffix and the error document path. Redirects are not supported.

